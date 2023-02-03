use crate::{relay_chain::mock_paras_sudo_wrapper, *};
use codec::Encode;
use frame_support::{
	assert_ok, log,
	pallet_prelude::{DispatchResult, DispatchResultWithPostInfo},
	traits::PalletInfoAccess,
};
use std::sync::Once;
use xcm::prelude::*;

mod misc;
mod xcm_asset_trap;
mod xcm_use_cases;

static INIT: Once = Once::new();
fn init_tracing() {
	INIT.call_once(|| {
		// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
		let _ = tracing_subscriber::fmt()
			.with_max_level(tracing::Level::TRACE)
			.with_env_filter("xcm=trace,system::events=trace") // Comment out this line to see all traces
			.with_test_writer()
			.init();
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
		id.into(),
		admin.into(),
		min_balance,
	)
}

fn create_derivative_asset_on_trappist(
	id: trappist::AssetId,
	admin: trappist::AccountId,
	min_balance: trappist::Balance,
) -> DispatchResult {
	trappist::Assets::create(trappist::RuntimeOrigin::signed(ALICE), id.into(), admin.into(), min_balance)
}

fn mint_asset_on_asset_reserve(
	asset_id: asset_reserve::AssetId,
	origin: asset_reserve::AccountId,
	mint_amount: asset_reserve::Balance,
) -> DispatchResult {
	asset_reserve::Assets::mint(
		asset_reserve::RuntimeOrigin::signed(origin),
		asset_id.into(),
		ALICE.into(),
		mint_amount,
	)
}

// Helper for outputting events
fn output_events<Runtime: frame_system::Config>() {
	const TARGET: &str = "system::events";
	let events = frame_system::Pallet::<Runtime>::events();
	log::trace!(target: TARGET, "{} events", events.len());
	for event in events {
		log::trace!(target: TARGET, "{:?}", event)
	}
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
	trappist_asset_id: trappist::AssetId,
	asset_reserve_asset_id: asset_reserve::AssetId,
) -> DispatchResultWithPostInfo {
	trappist::Sudo::sudo(
		trappist::RuntimeOrigin::signed(origin),
		Box::new(trappist::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
			trappist::Runtime,
		>::register_reserve_asset {
			asset_id: trappist_asset_id,
			asset_multi_location: (
				Parent,
				X3(
					Parachain(ASSET_RESERVE_PARA_ID),
					PalletInstance(asset_reserve::Assets::index() as u8),
					GeneralIndex(asset_reserve_asset_id as u128),
				),
			)
				.into(),
		})),
	)
}
