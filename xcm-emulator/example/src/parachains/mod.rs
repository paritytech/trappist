use codec::Encode;
use cumulus_primitives_core::ParaId;
use frame_support::{
	assert_ok, log,
	pallet_prelude::{DispatchResult, DispatchResultWithPostInfo},
	traits::{GenesisBuild, PalletInfoAccess},
};
pub use parachains_common::{AccountId, AssetId, Balance, Index};
pub use polkadot_runtime_common::paras_sudo_wrapper;
use sp_runtime::AccountId32;
use std::sync::Once;
use trappist_runtime::xcm_config::Convert;
use xcm::prelude::*;

pub(crate) mod asset_reserve;
pub(crate) mod stout;
pub(crate) mod trappist;

pub const INITIAL_BALANCE: u128 = 1_000_000_000_000_000;

pub const ASSET_RESERVE_PARA_ID: u32 = 1000;
pub const TRAPPIST_PARA_ID: u32 = 1836;
pub const STOUT_PARA_ID: u32 = 3000;

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

static INIT: Once = Once::new();
pub fn init_tracing() {
	INIT.call_once(|| {
		// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
		let _ = tracing_subscriber::fmt()
			.with_max_level(tracing::Level::TRACE)
			.with_env_filter("xcm=trace,system::events=trace,assets=trace") // Comment out this line to see all traces
			.with_test_writer()
			.init();
	});
}

pub fn assert_balance(actual: u128, expected: u128, fees: u128) {
	assert!(
		actual >= (expected - fees) && actual <= expected,
		"expected: {expected}, actual: {actual} fees: {fees}"
	)
}

pub fn sovereign_account(para_id: u32) -> AccountId {
	trappist_runtime::xcm_config::LocationToAccountId::convert_ref(MultiLocation::new(
		1,
		X1(Parachain(para_id)),
	))
	.unwrap()
}

pub fn create_asset_on_asset_reserve(
	id: AssetId,
	admin: AccountId,
	min_balance: Balance,
) -> DispatchResult {
	statemine_runtime::Assets::create(
		statemine_runtime::RuntimeOrigin::signed(ALICE),
		id.into(),
		admin.into(),
		min_balance,
	)
}

pub fn create_derivative_asset_on_trappist(
	id: trappist_runtime::AssetId,
	admin: trappist_runtime::AccountId,
	min_balance: trappist_runtime::Balance,
) -> DispatchResult {
	trappist_runtime::Assets::create(
		trappist_runtime::RuntimeOrigin::signed(ALICE),
		id.into(),
		admin.into(),
		min_balance,
	)
}

pub fn mint_asset_on_asset_reserve(
	asset_id: AssetId,
	origin: AccountId,
	mint_amount: Balance,
) -> DispatchResult {
	statemine_runtime::Assets::mint(
		statemine_runtime::RuntimeOrigin::signed(origin),
		asset_id.into(),
		ALICE.into(),
		mint_amount,
	)
}

// Helper for outputting events
pub fn output_events<Runtime: frame_system::Config>() {
	const TARGET: &str = "system::events";
	let events = frame_system::Pallet::<Runtime>::events();
	log::trace!(target: TARGET, "{} events", events.len());
	for event in events {
		log::trace!(target: TARGET, "{:?}", event)
	}
}

pub fn register_reserve_asset_on_trappist(
	origin: trappist_runtime::AccountId,
	trappist_asset_id: trappist_runtime::AssetId,
	asset_reserve_asset_id: AssetId,
) -> DispatchResultWithPostInfo {
	trappist_runtime::Sudo::sudo(
		trappist_runtime::RuntimeOrigin::signed(origin),
		Box::new(trappist_runtime::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
			trappist_runtime::Runtime,
		>::register_reserve_asset {
			asset_id: trappist_asset_id,
			asset_multi_location: (
				Parent,
				X3(
					Parachain(ASSET_RESERVE_PARA_ID),
					PalletInstance(statemine_runtime::Assets::index() as u8),
					GeneralIndex(asset_reserve_asset_id as u128),
				),
			)
				.into(),
		})),
	)
}
