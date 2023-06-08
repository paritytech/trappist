use super::*;
use sp_runtime::AccountId32;

pub const ALICE: AccountId32 = AccountId32::new([
	212, 53, 147, 199, 21, 253, 211, 28, 97, 20, 26, 189, 4, 169, 159, 214, 130, 44, 133, 88, 133,
	76, 205, 227, 154, 86, 132, 231, 165, 109, 162, 125,
]);
pub const BOB: AccountId32 = AccountId32::new([
	142, 175, 4, 21, 22, 135, 115, 99, 38, 201, 254, 161, 126, 37, 252, 82, 135, 97, 54, 147, 201,
	18, 144, 156, 178, 38, 170, 71, 148, 242, 106, 72,
]);
pub const CHARLIE: AccountId32 = AccountId32::new([
	144, 181, 171, 32, 92, 105, 116, 201, 234, 132, 27, 230, 136, 134, 70, 51, 220, 156, 168, 163,
	87, 132, 62, 234, 207, 35, 20, 100, 153, 101, 254, 34,
]);
pub const DAVE: AccountId32 = AccountId32::new([
	48, 103, 33, 33, 29, 84, 4, 189, 157, 168, 142, 2, 4, 54, 10, 26, 154, 184, 184, 124, 102, 193,
	188, 47, 205, 211, 127, 60, 34, 34, 204, 32,
]);
const INITIAL_BALANCE: u128 = 1_000_000_000_000;

pub(crate) fn new_ext(para_id: u32) -> sp_io::TestExternalities {
	use stout_runtime::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	// set parachain id
	let parachain_info_config = parachain_info::GenesisConfig { parachain_id: para_id.into() };
	<parachain_info::GenesisConfig as GenesisBuild<Runtime, _>>::assimilate_storage(
		&parachain_info_config,
		&mut t,
	)
	.unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, INITIAL_BALANCE)] }
		.assimilate_storage(&mut t)
		.unwrap();

	pallet_sudo::GenesisConfig::<Runtime> { key: Some(ALICE) }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}
