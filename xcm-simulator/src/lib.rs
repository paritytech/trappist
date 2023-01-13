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

#[cfg(test)]
mod tests_misc;
#[cfg(test)]
mod test_xcm_use_cases;
#[cfg(test)]
mod test_xcm_trap;

use frame_support::{sp_tracing, traits::GenesisBuild};
use parachains::{asset_reserve, base, template, trappist};
use polkadot_parachain::primitives::Id as ParaId;
use sp_core::Get;
use sp_runtime::traits::AccountIdConversion;

use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);
pub const BOB: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([1u8; 32]);
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