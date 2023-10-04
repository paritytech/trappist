use crate::tests::*;

#[test]
fn event_collection_works() {
	//init_tracing();

	RococoMockNet::reset();

	const AMOUNT: u128 = 1_000_000_000 * 10;
	const MAX_WEIGHT: u128 = 1_000_000_000;
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	ParaA::execute_with(|| {
		assert_ok!(<ParaA as ParaAPallet>::XcmPallet::execute(
			<ParaA as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));
		output_events::<<ParaA as Parachain>::Runtime>();
		assert_eq!(3, <ParaA as Parachain>::System::events().len());
	});

	ParaB::execute_with(|| {
		assert_ok!(<ParaB as ParaBPallet>::XcmPallet::execute(
			<ParaB as Parachain>::RuntimeOrigin::signed(alice_account),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));
		output_events::<<ParaB as Parachain>::Runtime>();
		assert_eq!(1, <ParaB as Parachain>::System::events().len());
	});
}
