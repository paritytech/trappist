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
use parachains::{asset_reserve, base, template, trappist};
use polkadot_parachain::primitives::Id as ParaId;
use sp_core::Get;
use sp_runtime::traits::AccountIdConversion;

use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const INITIAL_BALANCE: u128 = 2_000_000_000;

const ASSET_RESERVE_PARA_ID: u32 = 1000;
decl_test_parachain! {
	// An asset reserve parachain (Statemine)
	pub struct AssetReserve {
		Runtime = asset_reserve::Runtime,
		XcmpMessageHandler = asset_reserve::MsgQueue,
		DmpMessageHandler = asset_reserve::MsgQueue,
		new_ext = {
			// Initialise parachain-specific genesis state
			use asset_reserve::{MsgQueue, Runtime, System};

			const INITIAL_BALANCE: u128 = <Runtime as pallet_assets::Config>::AssetDeposit::get() * 2;

			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

			pallet_balances::GenesisConfig::<Runtime> { balances: vec![
					(ALICE, INITIAL_BALANCE),
					(asset_reserve::sovereign_account(TRAPPIST_PARA_ID), INITIAL_BALANCE)
				]}
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
	// The trappist parachain
	pub struct Trappist {
		Runtime = trappist::Runtime,
		XcmpMessageHandler = trappist::MsgQueue,
		DmpMessageHandler = trappist::MsgQueue,
		new_ext = {
			// Initialise parachain-specific genesis state
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
	// A parachain using the trappist 'base' runtime
	pub struct Base {
		Runtime = base::Runtime,
		XcmpMessageHandler = base::MsgQueue,
		DmpMessageHandler = base::MsgQueue,
		new_ext = {
			// Initialise parachain-specific genesis state
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
				MsgQueue::set_para_id(BASE_PARA_ID.into());
			});
			ext
		},
	}
}

const TEMPLATE_PARA_ID: u32 = 4000;
decl_test_parachain! {
	// A simple parachain configuration template, which can be copied and customised to add another mock
	// parachain to the network
	pub struct Template {
		Runtime = template::Runtime,
		XcmpMessageHandler = template::MsgQueue,
		DmpMessageHandler = template::MsgQueue,
		new_ext = {
			use template::{MsgQueue, Runtime, System};

			let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

			pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
				.assimilate_storage(&mut t)
				.unwrap();

			let mut ext = sp_io::TestExternalities::new(t);
			ext.execute_with(|| {
				sp_tracing::try_init_simple();
				System::set_block_number(1);
				MsgQueue::set_para_id(ParaId::new(TEMPLATE_PARA_ID));
			});
			ext
		},
	}
}

decl_test_relay_chain! {
	// The relay chain (Rococo)
	pub struct Relay {
		Runtime = relay_chain::Runtime,
		XcmConfig = relay_chain::XcmConfig,
		new_ext = {
			// Initialise relay chain genesis state
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
			(ASSET_RESERVE_PARA_ID, AssetReserve),
			(TRAPPIST_PARA_ID, Trappist),
			(BASE_PARA_ID, Base),
		],
	}
}

pub fn para_account_id(id: u32) -> relay_chain::AccountId {
	ParaId::from(id).into_account_truncating()
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::relay_chain::mock_paras_sudo_wrapper;
	use codec::Encode;
	use frame_support::{
		assert_ok,
		pallet_prelude::{DispatchResult, DispatchResultWithPostInfo},
		traits::PalletInfoAccess,
	};
	use std::sync::Once;
	use thousands::Separable;
	use xcm::prelude::*;
	use xcm_simulator::{TestExt, Weight};

	static INIT: Once = Once::new();
	pub fn init_tracing() {
		INIT.call_once(|| {
			// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
			let _ = tracing_subscriber::fmt()
				.with_max_level(tracing::Level::TRACE)
				.with_env_filter("xcm=trace") // Comment out this line to see all traces
				.with_test_writer()
				.init();
		});
	}

	#[allow(non_upper_case_globals)]
	const xUSD: u32 = 1;
	#[allow(non_upper_case_globals)]
	const txUSD: u32 = 10;
	#[allow(non_upper_case_globals)]
	const pxUSD: u32 = xUSD; // Must match asset reserve identifier as no asset registry available in base runtime

	// Teleports some amount of the native asset of the relay chain to the asset reserve parachain
	// (DMP)
	#[test]
	fn teleport_native_asset_from_relay_chain_to_asset_reserve_parachain() {
		init_tracing();

		MockNet::reset();

		let mut beneficiary_balance = 0;
		let mut total_issuance = 0;

		AssetReserve::execute_with(|| {
			// Check beneficiary balance and total issuance on asset reserve before teleport
			beneficiary_balance = asset_reserve::Balances::free_balance(&ALICE);
			total_issuance = asset_reserve::Balances::total_issuance();
		});

		const AMOUNT: u128 = 1_000_000_000;

		Relay::execute_with(|| {
			// Teleport, ensuring relay chain total issuance remains constant
			let total_issuance = relay_chain::Balances::total_issuance();
			assert_ok!(relay_chain::XcmPallet::teleport_assets(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(Parachain(ASSET_RESERVE_PARA_ID).into().into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Here, AMOUNT).into()),
				0
			));
			assert_eq!(relay_chain::Balances::total_issuance(), total_issuance);

			// Ensure teleport amount 'checked out' to check account
			assert_eq!(relay_chain::Balances::free_balance(&relay_chain::check_account()), AMOUNT);
			// Ensure sender balance decreased by teleport amount
			assert_eq!(relay_chain::Balances::free_balance(&ALICE), INITIAL_BALANCE - AMOUNT);
		});

		const EST_FEES: u128 = 4_000_000;
		AssetReserve::execute_with(|| {
			// Ensure receiver balance and total issuance increased by teleport amount
			let current_balance = asset_reserve::Balances::free_balance(&ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
			assert_eq!(asset_reserve::Balances::total_issuance(), total_issuance + AMOUNT);

			println!(
				"Teleport: initial balance {} teleport amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EST_FEES.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	// Teleports some amount of the (shared) native asset of the asset reserve parachain back to the
	// relay chain (UMP)
	#[test]
	fn teleport_native_asset_from_asset_reserve_parachain_to_relay_chain() {
		init_tracing();

		MockNet::reset();

		const AMOUNT: u128 = 1_000_000_000;
		let mut beneficiary_balance = 0;

		Relay::execute_with(|| {
			// Teleport some amount to asset reserve so there are tokens to teleport back
			assert_ok!(relay_chain::XcmPallet::teleport_assets(
				relay_chain::RuntimeOrigin::signed(ALICE),
				Box::new(Parachain(ASSET_RESERVE_PARA_ID).into().into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Here, AMOUNT).into()),
				0
			));

			// Check beneficiary balance
			beneficiary_balance = relay_chain::Balances::free_balance(&ALICE);
		});

		AssetReserve::execute_with(|| {
			// Check sender balance & total issuance of native asset on asset reserve before
			// teleporting
			let sender_balance = asset_reserve::Balances::free_balance(&ALICE);
			let total_issuance = asset_reserve::Balances::total_issuance();
			assert_ok!(asset_reserve::PolkadotXcm::teleport_assets(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				Box::new(Parent.into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new((Parent, AMOUNT).into()),
				0
			));

			// Ensure sender balance and total issuance (of native asset on asset reserve) decreased
			// by teleport amount
			assert_eq!(asset_reserve::Balances::free_balance(&ALICE), sender_balance - AMOUNT);
			assert_eq!(asset_reserve::Balances::total_issuance(), total_issuance - AMOUNT)
		});

		const EST_FEES: u128 = 2_500_000;
		Relay::execute_with(|| {
			// Ensure receiver balance increased by teleport amount
			let current_balance = relay_chain::Balances::free_balance(&ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
			println!(
				"Teleport: initial balance {} teleport amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EST_FEES.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	// Initiates a reserve-transfer of some asset on the asset reserve parachain to the trappist
	// parachain (HRMP)
	#[test]
	fn reserve_transfer_asset_from_asset_reserve_parachain_to_trappist_parachain() {
		init_tracing();

		MockNet::reset();

		const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
		const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

		AssetReserve::execute_with(|| {
			// Create fungible asset on Reserve Parachain
			assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));

			// Mint fungible asset
			assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, MINT_AMOUNT));
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
					min_balance: ASSET_MIN_BALANCE,
					is_sufficient: true,
					is_frozen: false,
				},
			));
		});

		let mut beneficiary_balance = 0;
		Trappist::execute_with(|| {
			// Create derivative asset on Trappist Parachain
			assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

			// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
			// Parachain) via Asset Registry
			assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD));
			assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());

			// Check beneficiary balance
			beneficiary_balance = trappist::Assets::balance(txUSD, &ALICE);
		});

		const AMOUNT: u128 = 10_000_000_000;

		AssetReserve::execute_with(|| {
			// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
			assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
				Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
				Box::new(
					(
						X2(
							PalletInstance(asset_reserve::Assets::index() as u8),
							GeneralIndex(xUSD as u128)
						),
						AMOUNT
					)
						.into()
				),
				0,
				WeightLimit::Unlimited,
			));

			// Ensure send amount moved to sovereign account
			let sovereign_account = asset_reserve::sovereign_account(TRAPPIST_PARA_ID);
			assert_eq!(asset_reserve::Assets::balance(xUSD, &sovereign_account), AMOUNT);
		});

		const EST_FEES: u128 = 1_600_000_000;
		Trappist::execute_with(|| {
			// Ensure beneficiary account balance increased
			let current_balance = trappist::Assets::balance(txUSD, &ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
			println!(
				"Reserve-transfer: initial balance {} transfer amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EST_FEES.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	// Initiates a send of a XCM message from trappist to the asset reserve parachain, instructing
	// it to transfer some amount of a fungible asset to some tertiary (base) parachain (HRMP)
	#[test]
	fn two_hop_reserve_transfer_from_trappist_parachain_to_tertiary_parachain() {
		init_tracing();

		MockNet::reset();

		const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
		const AMOUNT: u128 = 100_000_000_000;

		AssetReserve::execute_with(|| {
			// Create and mint fungible asset on Reserve Parachain
			assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));
			assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, AMOUNT * 2));

			// Touch parachain account
			assert_ok!(asset_reserve::Assets::transfer(
				asset_reserve::RuntimeOrigin::signed(ALICE),
				xUSD,
				asset_reserve::sovereign_account(TRAPPIST_PARA_ID).into(),
				AMOUNT
			));
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
					min_balance: ASSET_MIN_BALANCE,
					is_sufficient: true,
					is_frozen: false,
				},
			));
		});

		let mut beneficiary_balance = 0;
		Base::execute_with(|| {
			// Create fungible asset on tertiary parachain
			assert_ok!(create_derivative_asset_on_tertiary_parachain(
				pxUSD,
				ALICE,
				ASSET_MIN_BALANCE
			));
			beneficiary_balance = base::Assets::balance(pxUSD, &ALICE);
		});

		const MAX_WEIGHT: u128 = 1_000_000_000 * 2; // 1,000,000,000 per instruction
		const EXECUTION_COST: u128 = 65_000_000_000;

		Trappist::execute_with(|| {
			// Create derivative asset on Trappist Parachain
			assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

			// Mint derivative asset on Trappist Parachain
			assert_ok!(trappist::Assets::mint(
				trappist::RuntimeOrigin::signed(ALICE),
				txUSD,
				ALICE.into(),
				AMOUNT * 2
			));
			assert_eq!(trappist::Assets::balance(txUSD, &ALICE), AMOUNT * 2);

			// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
			// Parachain) via Asset Registry
			assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD));
			assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());

			// Trappist parachain should be able to reserve-transfer an asset to Tertiary Parachain
			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![
					WithdrawAsset(
						(
							(
								Parent,
								X3(
									Parachain(ASSET_RESERVE_PARA_ID),
									PalletInstance(asset_reserve::Assets::index() as u8),
									GeneralIndex(xUSD as u128)
								)
							),
							AMOUNT
						)
							.into()
					),
					InitiateReserveWithdraw {
						assets: Wild(All),
						reserve: (Parent, Parachain(ASSET_RESERVE_PARA_ID)).into(),
						xcm: Xcm(vec![
							BuyExecution {
								fees: (
									X2(
										PalletInstance(asset_reserve::Assets::index() as u8),
										GeneralIndex(xUSD as u128)
									),
									EXECUTION_COST
								)
									.into(),
								weight_limit: Unlimited
							},
							DepositReserveAsset {
								assets: Wild(All),
								max_assets: 1,
								dest: (Parent, Parachain(BASE_PARA_ID)).into(),
								xcm: Xcm(vec![DepositAsset {
									assets: Wild(All),
									max_assets: 1,
									beneficiary: X1(AccountId32 { network: Any, id: ALICE.into() })
										.into()
								}])
							}
						])
					},
				]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			// // Check send amount moved to sovereign account
			// let sovereign_account = asset_reserve::sovereign_account(TRAPPIST_PARA_ID);
			// assert_eq!(asset_reserve::Assets::balance(xUSD, &sovereign_account), AMOUNT);
		});

		Base::execute_with(|| {
			// Ensure beneficiary received amount, less fees
			let current_balance = base::Assets::balance(pxUSD, &ALICE);
			assert_balance(current_balance, beneficiary_balance + AMOUNT, EXECUTION_COST);
			println!(
				"Two-hop Reserve-transfer: initial balance {} transfer amount {} current balance {} estimated fees {} actual fees {}",
				beneficiary_balance.separate_with_commas(),
				AMOUNT.separate_with_commas(),
				current_balance.separate_with_commas(),
				EXECUTION_COST.separate_with_commas(),
				(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
			);
		});
	}

	fn assert_balance(actual: u128, expected: u128, fees: u128) {
		assert!(
			actual >= (expected - fees) && actual <= expected,
			"expected: {expected}, actual: {actual} fees: {fees}"
		)
	}

	fn create_asset_on_asset_reserve(
		id: asset_reserve::AssetId,
		admin: asset_reserve::AccountId,
		min_balance: asset_reserve::Balance,
	) -> DispatchResult {
		asset_reserve::Assets::create(
			asset_reserve::RuntimeOrigin::signed(ALICE),
			id,
			admin.into(),
			min_balance,
		)
	}

	fn create_derivative_asset_on_trappist(
		id: trappist::AssetId,
		admin: trappist::AccountId,
		min_balance: trappist::Balance,
	) -> DispatchResult {
		trappist::Assets::create(
			trappist::RuntimeOrigin::signed(ALICE),
			id,
			admin.into(),
			min_balance,
		)
	}

	fn create_derivative_asset_on_tertiary_parachain(
		id: base::AssetId,
		admin: base::AccountId,
		min_balance: base::Balance,
	) -> DispatchResult {
		base::Assets::create(base::RuntimeOrigin::signed(ALICE), id, admin.into(), min_balance)
	}

	fn mint_asset_on_asset_reserve(
		asset_id: asset_reserve::AssetId,
		origin: asset_reserve::AccountId,
		mint_amount: asset_reserve::Balance,
	) -> DispatchResult {
		asset_reserve::Assets::mint(
			asset_reserve::RuntimeOrigin::signed(origin),
			asset_id,
			ALICE.into(),
			mint_amount,
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

	fn register_reserve_asset_on_trappist(
		origin: trappist::AccountId,
		asset_id: trappist::AssetId,
	) -> DispatchResultWithPostInfo {
		trappist::Sudo::sudo(
			trappist::RuntimeOrigin::signed(origin),
			Box::new(trappist::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
				trappist::Runtime,
			>::register_reserve_asset {
				asset_id,
				asset_multi_location: (
					Parent,
					X3(
						Parachain(ASSET_RESERVE_PARA_ID),
						PalletInstance(asset_reserve::Assets::index() as u8),
						GeneralIndex(xUSD as u128),
					),
				)
					.into(),
			})),
		)
	}
}
