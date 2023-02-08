use crate::tests::*;
use frame_support::assert_ok;
use xcm_simulator::TestExt;

#[test]
fn event_collection_works() {
	init_tracing();

	MockNet::reset();

	const AMOUNT: u128 = trappist::EXISTENTIAL_DEPOSIT * 10;
	const MAX_WEIGHT: u128 = 1_000_000_000;

	Trappist::execute_with(|| {
		assert_ok!(trappist::PolkadotXcm::execute(
			trappist::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
			MAX_WEIGHT as u64
		));
		output_events::<trappist::Runtime>();
		assert_eq!(3, trappist::System::events().len());
	});

	Stout::execute_with(|| {
		assert_ok!(stout::PolkadotXcm::execute(
			stout::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
			MAX_WEIGHT as u64
		));
		output_events::<stout::Runtime>();
		assert_eq!(1, trappist::System::events().len());
	});
}
