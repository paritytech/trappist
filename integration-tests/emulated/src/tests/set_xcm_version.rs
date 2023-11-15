use crate::tests::*;

#[test]
fn trappist_sets_stout_para_xcm_supported_version() {
	init_tracing();
	// Init tests variables
	let sudo_origin = <Trappist as Chain>::RuntimeOrigin::root();
	let stout_para_destination: MultiLocation = MultiLocation::new(1, X1(3000u64.into()));

	// Relay Chain sets supported version for Asset Parachain
	Trappist::execute_with(|| {
		type RuntimeEvent = <Trappist as Chain>::RuntimeEvent;

		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::force_xcm_version(
			sudo_origin,
			bx!(stout_para_destination),
			XCM_V3
		));

		assert_expected_events!(
			Trappist,
			vec![
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::SupportedVersionChanged {
					location,
					version: XCM_V3
				}) => { location: *location == stout_para_destination, },
			]
		);
	});
}
