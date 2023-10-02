use crate::tests::*;

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
