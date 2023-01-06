// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

extern crate core;

mod parachains;
mod relay_chain;

use frame_support::{sp_tracing, traits::GenesisBuild};
use parachains::{asset_reserve, parachain, trappist};
use polkadot_parachain::primitives::Id as ParaId;
use sp_core::Get;
use sp_runtime::traits::AccountIdConversion;

use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const INITIAL_BALANCE: u128 = 10_000_000_000;

const ASSET_RESERVE_PARA_ID: u32 = 1000;
decl_test_parachain! {
	pub struct AssetReserve {
		Runtime = asset_reserve::Runtime,
		XcmpMessageHandler = asset_reserve::MsgQueue,
		DmpMessageHandler = asset_reserve::MsgQueue,
		new_ext = {
			use asset_reserve::{MsgQueue, Runtime, System};

			const INITIAL_BALANCE: u128 = <Runtime as pallet_assets::Config>::AssetDeposit::get() * 2;

			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

			pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
				.assimilate_storage(&mut t)
				.unwrap();

			let mut ext = sp_io::TestExternalities::new(t);
			ext.execute_with(|| {
				sp_tracing::try_init_simple();
				System::set_block_number(1);
				MsgQueue::set_para_id(ASSET_RESERVE_PARA_ID.into());
			});
			ext
		},
	}
}

const TRAPPIST_PARA_ID: u32 = 2000;
decl_test_parachain! {
	pub struct Trappist {
		Runtime = trappist::Runtime,
		XcmpMessageHandler = trappist::MsgQueue,
		DmpMessageHandler = trappist::MsgQueue,
		new_ext = {
			use trappist::{MsgQueue, Runtime, System};

			let asset_deposit: u128 = <trappist::Runtime as pallet_assets::Config>::AssetDeposit::get();
			let initial_balance: u128 = asset_deposit * 2;

			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

			pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, initial_balance)] }
				.assimilate_storage(&mut t)
				.unwrap();

			pallet_sudo::GenesisConfig::<Runtime> { key: Some(ALICE) }
				.assimilate_storage(&mut t)
				.unwrap();

			let mut ext = sp_io::TestExternalities::new(t);
			ext.execute_with(|| {
				sp_tracing::try_init_simple();
				System::set_block_number(1);
				MsgQueue::set_para_id(TRAPPIST_PARA_ID.into());
			});
			ext
		},
	}
}

const BASE_PARA_ID: u32 = 3000;
decl_test_parachain! {
	pub struct Base {
		Runtime = parachains::parachain::Runtime,
		XcmpMessageHandler = parachains::parachain::MsgQueue,
		DmpMessageHandler = parachains::parachain::MsgQueue,
		new_ext = para_ext(BASE_PARA_ID),
	}
}

const A_PARA_ID: u32 = 1;
decl_test_parachain! {
	pub struct ParaA {
		Runtime = parachains::parachain::Runtime,
		XcmpMessageHandler = parachains::parachain::MsgQueue,
		DmpMessageHandler = parachains::parachain::MsgQueue,
		new_ext = para_ext(A_PARA_ID),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		XcmConfig = relay_chain::XcmConfig,
		new_ext = {
			use relay_chain::{Runtime, System};

			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

			pallet_balances::GenesisConfig::<Runtime> {
				balances: vec![(ALICE, INITIAL_BALANCE), (para_account_id(1), INITIAL_BALANCE)],
			}
			.assimilate_storage(&mut t)
			.unwrap();

			pallet_sudo::GenesisConfig::<Runtime> { key: Some(ALICE) }
				.assimilate_storage(&mut t)
				.unwrap();

			let mut ext = sp_io::TestExternalities::new(t);
			ext.execute_with(|| System::set_block_number(1));
			ext
		},
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(A_PARA_ID, ParaA),
			(ASSET_RESERVE_PARA_ID, AssetReserve),
			(TRAPPIST_PARA_ID, Trappist),
			(BASE_PARA_ID, Base),
		],
	}
}

pub fn para_account_id(id: u32) -> relay_chain::AccountId {
	ParaId::from(id).into_account_truncating()
}

fn para_ext(para_id: u32) -> sp_io::TestExternalities {
	use parachain::{MsgQueue, Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		sp_tracing::try_init_simple();
		System::set_block_number(1);
		MsgQueue::set_para_id(para_id.into());
	});
	ext
}

#[cfg(test)]
mod tests {
	use super::*;

	use crate::relay_chain::mock_paras_sudo_wrapper;
	use codec::Encode;
	use frame_support::assert_ok;
	use std::sync::Once;
	use xcm::{latest::prelude::*, opaque::VersionedXcm};
	use xcm_simulator::TestExt;

	static INIT: Once = Once::new();
	pub fn init_tracing() {
		INIT.call_once(|| {
			// Add test tracing
			// todo: filter to only show xcm logs
			sp_tracing::init_for_tests();
		});
	}

	const ASSET_RESERVE_PALLET_INDEX: u8 = 50;

	#[test]
	fn teleport_asset_from_relay_chain_asset_reserve_parachain() {
		init_tracing();

		MockNet::reset();

		const AMOUNT: u128 = 1_000_000_000;
		let mut receiver_balance = 0;
		let mut total_issuance = 0;

		AssetReserve::execute_with(|| {
			// Check receiver balance increased by teleport amount
			receiver_balance = asset_reserve::Balances::free_balance(&ALICE);
			total_issuance = asset_reserve::Balances::total_issuance();
		});

		Relay::execute_with(|| {
			assert_ok!(relay_chain::XcmPallet::teleport_assets(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(Parachain(ASSET_RESERVE_PARA_ID).into().into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Here, AMOUNT).into()),
				0
			));

			// Check teleport amount checked out to check account
			assert_eq!(relay_chain::Balances::free_balance(&relay_chain::check_account()), AMOUNT);

			// Check sender balance decreased by teleport amount
			assert_eq!(relay_chain::Balances::free_balance(&ALICE), INITIAL_BALANCE - AMOUNT);
		});

		const EST_FEES: u128 = 4_000_000;
		AssetReserve::execute_with(|| {
			// Check receiver balance and total issuance increased by teleport amount
			assert_balance(
				asset_reserve::Balances::free_balance(&ALICE),
				receiver_balance + AMOUNT,
				EST_FEES,
			);
			assert_eq!(asset_reserve::Balances::total_issuance(), total_issuance + AMOUNT)
		});
	}

	#[test]
	fn teleport_asset_from_asset_reserve_parachain_to_relay_chain() {
		todo!()
	}

	#[test]
	#[allow(non_upper_case_globals)]
	fn reserve_transfer_asset_from_asset_reserve_parachain_to_trappist_parachain() {
		init_tracing();

		MockNet::reset();

		const xUSD: u32 = 1;
		const txUSD: u32 = 10;
		const MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
		const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

		AssetReserve::execute_with(|| {
			// Create fungible asset on Reserve Parachain
			assert_ok!(asset_reserve::Assets::create(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				xUSD,
				ALICE.into(),
				MIN_BALANCE
			));

			// Mint fungible asset
			assert_ok!(asset_reserve::Assets::mint(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				xUSD,
				ALICE.into(),
				MINT_AMOUNT
			));
			assert_eq!(asset_reserve::Assets::balance(xUSD, &ALICE), MINT_AMOUNT);
		});

		Relay::execute_with(|| {
			// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
			paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
				pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
					id: xUSD,
					owner: ALICE.into(),
					issuer: ALICE.into(),
					admin: ALICE.into(),
					freezer: ALICE.into(),
					min_balance: MIN_BALANCE,
					is_sufficient: true,
					is_frozen: false,
				},
			));
		});

		Trappist::execute_with(|| {
			// Create derivative asset on Trappist Parachain
			assert_ok!(trappist::Assets::create(
				trappist::RuntimeOrigin::signed(ALICE),
				txUSD,
				ALICE.into(),
				MIN_BALANCE
			));

			// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
			// Parachain) via Asset Registry
			assert_ok!(trappist::Sudo::sudo(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(trappist::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
					trappist::Runtime,
				>::register_reserve_asset {
					asset_id: txUSD,
					asset_multi_location: (
						Parent,
						X3(
							Parachain(ASSET_RESERVE_PARA_ID),
							PalletInstance(ASSET_RESERVE_PALLET_INDEX),
							GeneralIndex(xUSD as u128),
						),
					)
						.into(),
				})),
			));
			assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some())
		});

		const AMOUNT: u128 = 10_000_000_000_000_000;

		AssetReserve::execute_with(|| {
			// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
			assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new(
					(
						X2(PalletInstance(ASSET_RESERVE_PALLET_INDEX), GeneralIndex(xUSD as u128)),
						AMOUNT
					)
						.into()
				),
				0,
				WeightLimit::Unlimited,
			));

			// Check send amount moved to sovereign account
			let sovereign_account = asset_reserve::sovereign_account(TRAPPIST_PARA_ID);
			assert_eq!(asset_reserve::Assets::balance(xUSD, &sovereign_account), AMOUNT);
		});

		const EST_FEES: u128 = 1_600_000_000;
		Trappist::execute_with(|| {
			// Check beneficiary account balance
			assert_balance(trappist::Assets::balance(txUSD, &ALICE), AMOUNT, EST_FEES);
		});
	}

	#[test]
	#[allow(non_upper_case_globals)]
	fn two_hop_reserve_transfer_from_trappist_parachain_to_tertiary_parachain() {
		todo!()
	}

	fn assert_balance(actual: u128, expected: u128, fees: u128) {
		assert!(
			actual >= (expected - fees) && actual <= expected,
			"expected: {expected}, actual: {actual} fees: {fees}"
		)
	}

	fn paras_sudo_wrapper_sudo_queue_downward_xcm<RuntimeCall: Encode>(call: RuntimeCall) {
		let sudo_queue_downward_xcm =
			relay_chain::RuntimeCall::ParasSudoWrapper(mock_paras_sudo_wrapper::Call::<
				relay_chain::Runtime,
			>::sudo_queue_downward_xcm {
				id: ParaId::new(ASSET_RESERVE_PARA_ID),
				xcm: Box::new(VersionedXcm::V2(Xcm(vec![Transact {
					origin_type: OriginKind::Superuser,
					require_weight_at_most: 10000000000u64,
					call: call.encode().into(),
				}]))),
			});

		assert_ok!(relay_chain::Sudo::sudo(
			relay_chain::RuntimeOrigin::signed(ALICE),
			Box::new(sudo_queue_downward_xcm),
		));
	}
}
