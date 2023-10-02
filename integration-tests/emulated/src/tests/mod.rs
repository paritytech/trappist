use super::*;

mod reserve_asset_transfer;
mod set_xcm_version;

#[allow(non_upper_case_globals)]
const xUSD: u32 = 1984;
#[allow(non_upper_case_globals)]
const txUSD: u32 = 10;

const ASSET_HUB_ID: u32 = 1_000;
const TRAPPIST_ID: u32 = 1_836;

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
