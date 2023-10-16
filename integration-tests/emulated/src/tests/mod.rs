use super::*;
use frame_support::{assert_ok, instances::Instance1, traits::PalletInfoAccess};
use integration_tests_common::constants::{accounts::ALICE, XCM_V3};
use parachains_common::AccountId;
use xcm_emulator::{Chain, Network, Parachain};
use xcm_primitives::AssetMultiLocationGetter;

// mod misc;
mod reserve_asset_transfer;
mod set_xcm_version;
// mod xcm_asset_trap;

#[allow(non_upper_case_globals)]
const xUSD: u32 = 1984;
#[allow(non_upper_case_globals)]
const txUSD: u32 = 10;

const ASSET_HUB_ID: u32 = 1_000;
const TRAPPIST_ID: u32 = 1_836;
const STOUT_ID: u32 = 3_000;

static INIT: std::sync::Once = std::sync::Once::new();
fn init_tracing() {
	INIT.call_once(|| {
		// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
		let _ = tracing_subscriber::fmt()
			.with_max_level(tracing::Level::TRACE)
			// Comment out this line to see all traces
			.with_env_filter(
				vec![
					"xcm=trace",
					// PDD: xcm-emulator
					"events=trace",
					"hrmp=trace",
					"dmp=trace",
					"ump=trace",
				]
				.join(","),
			)
			.with_test_writer()
			.init();
	});
}

fn output_events<Runtime: frame_system::Config>() {
	const TARGET: &str = "system::events";
	let events = frame_system::Pallet::<Runtime>::events();
	log::trace!(target: TARGET, "{} events", events.len());
	for event in events {
		log::trace!(target: TARGET, "{:?}", event)
	}
}
