#![cfg(test)]

use frame_support::traits::GenesisBuild;
use parachains_common::AssetId as AssetIdType;
use polkadot_parachain::primitives::Id as ParaId;
use sp_runtime::traits::AccountIdConversion;
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub mod constants;
pub mod parachain;
pub mod relay;
pub mod statemine;

pub const STATEMINE_PARA_ID: u32 = 1000;
pub const PARA_ID: u32 = 2000;

pub const STATEMINE_ASSETS_PALLET_INSTANCE: u8 = 7;

pub const STATEMINE_ASSET_ID: AssetIdType = 999;
pub const PARA_ASSET_ID: AssetIdType = 999; // todo: make different

pub const ALICE: sp_runtime::AccountId32 = sp_runtime::AccountId32::new([0u8; 32]);

pub const INITIAL_BALANCE: u128 = 1_000_000_000;

decl_test_parachain! {
	pub struct Statemine {
		Runtime = reserve_parachain::Runtime,
		XcmpMessageHandler = statemine::XcmpQueue,
		DmpMessageHandler = statemine::DmpQueue,
		new_ext = statemine_ext(),
	}
}

decl_test_parachain! {
	pub struct Para {
		Runtime = parachain::Runtime,
		XcmpMessageHandler = parachain::XcmpQueue,
		DmpMessageHandler = parachain::DmpQueue,
		new_ext = para_ext(),
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay::Runtime,
		XcmConfig = relay::XcmConfig,
		new_ext = relay_ext(),
	}
}

decl_test_network! {
	pub struct MockNet {
		relay_chain = Relay,
		parachains = vec![
			(STATEMINE_PARA_ID, Statemine),
			(PARA_ID, Para),
		],
	}
}

pub fn para_account_id(id: u32) -> relay::AccountId {
	ParaId::from(id).into_account_truncating()
}

pub fn statemine_ext() -> sp_io::TestExternalities {
	use statemine::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let parachain_info_config =
		parachain_info::GenesisConfig { parachain_id: STATEMINE_PARA_ID.into() };
	<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
		&parachain_info_config,
		&mut t,
	)
	.unwrap();

	pallet_assets::GenesisConfig::<Runtime> {
		assets: vec![
			// id, owner, is_sufficient, min_balance
			(STATEMINE_ASSET_ID, ALICE, true, 1),
		],
		metadata: vec![
			// id, name, symbol, decimals
			(STATEMINE_ASSET_ID, "xUSD".into(), "xUSD".into(), 12),
		],
		accounts: vec![
			// id, account_id, balance
			(STATEMINE_ASSET_ID, ALICE, INITIAL_BALANCE.try_into().unwrap()),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

pub fn para_ext() -> sp_io::TestExternalities {
	use parachain::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let parachain_info_config = parachain_info::GenesisConfig { parachain_id: PARA_ID.into() };
	<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
		&parachain_info_config,
		&mut t,
	)
	.unwrap();

	pallet_assets::GenesisConfig::<Runtime> {
		assets: vec![
			// id, owner, is_sufficient, min_balance
			(PARA_ASSET_ID, ALICE, true, 1),
		],
		metadata: vec![
			// id, name, symbol, decimals
			(PARA_ASSET_ID, "txUSD".into(), "txUSD".into(), 12),
		],
		accounts: vec![
			// id, account_id, balance
			(PARA_ASSET_ID, ALICE, INITIAL_BALANCE.try_into().unwrap()),
		],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| {
		System::set_block_number(1);
	});
	ext
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay::{Runtime, System};

	let t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub type StateminePalletXcm = pallet_xcm::Pallet<statemine::Runtime>;
