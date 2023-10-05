use crate::tests::*;

#[test]
fn event_collection_works() {
	//init_tracing();

	RococoMockNet::reset();

	const AMOUNT: u128 = 1_000_000_000 * 10;
	const MAX_WEIGHT: u128 = 1_000_000_000;
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	Trappist::execute_with(|| {
		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));
		output_events::<<Trappist as Parachain>::Runtime>();
		assert_eq!(3, <Trappist as Parachain>::System::events().len());
	});

	Stout::execute_with(|| {
		assert_ok!(<Stout as StoutPallet>::XcmPallet::execute(
			<Stout as Parachain>::RuntimeOrigin::signed(alice_account),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));
		output_events::<<Stout as Parachain>::Runtime>();
		assert_eq!(1, <Stout as Parachain>::System::events().len());
	});
}
