use super::*;
use frame_support::{assert_ok, traits::Get};
use relay_chain::{BOB, DAVE};
use sp_runtime::{
	app_crypto::{sp_core::H160, ByteArray},
	traits::{Hash, Keccak256},
};
use std::sync::Once;
use xcm_emulator::TestExt;

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
