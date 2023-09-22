use super::*;
use frame_support::assert_ok;
use xcm_emulator::assert_expected_events;

#[allow(dead_code)]
fn overview() {
	type MockNetwork = RococoMockNet;
	type RelayChain = Rococo;
	type Para = ParaA;

	type TestExt = dyn xcm_emulator::TestExt;

	let _messagesemulator = xcm_emulator::DOWNWARD_MESSAGES;

	type RuntimeA = <ParaA as Parachain>::Runtime;
	type XcmPallet = pallet_xcm::Pallet<RuntimeA>;
}

#[test]
fn trappist_sets_stout_para_xcm_supported_version() {
	init_tracing();
	// Init tests variables
	let sudo_origin = <ParaA as Parachain>::RuntimeOrigin::root();
	let stout_para_destination: MultiLocation = MultiLocation::new(1, X1(3000u64.into()));

	// Relay Chain sets supported version for Asset Parachain
	ParaA::execute_with(|| {
		type RuntimeEvent = <ParaA as Parachain>::RuntimeEvent;

		assert_ok!(<ParaA as ParaAPallet>::XcmPallet::force_xcm_version(
			sudo_origin,
			bx!(stout_para_destination),
			XCM_V3
		));

		assert_expected_events!(
			ParaA,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::SupportedVersionChanged {
					location,
					version: XCM_V3
				}) => { location: *location == stout_para_destination, },
			]
		);
	});
}

// #[test]
// fn teleport_native_asset_from_relay_chain_to_asset_reserve_parachain() {

//     init_tracing();

// }

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
