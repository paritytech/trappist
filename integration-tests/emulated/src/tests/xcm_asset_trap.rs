use crate::tests::*;

// make sure we can trap a native asset
#[test]
fn native_trap_works() {
	init_tracing();

	RococoMockNet::reset();

	const AMOUNT: u128 = 33_333_333 * 10;
	const MAX_WEIGHT: u128 = 1_000_000_000;
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	Trappist::execute_with(|| {
		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(<Trappist as Parachain>::System::events().iter().any(|r| matches!(
			r.event,
			trappist_runtime::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		//PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		let origin: MultiLocation = AccountId32 { network: None, id: alice_account.into() }.into();
		let native_asset: Assets = MultiAsset {
			id: Concrete(MultiLocation { parents: 0, interior: Here }),
			fun: Fungible((AMOUNT) as u128),
		}
		.into();
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = <Trappist as TrappistPallet>::XcmPallet::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 1);
	});
}

// make sure that native dust is not trapped
#[test]
fn native_dust_trap_doesnt_work() {
	init_tracing();

	RococoMockNet::reset();

	const AMOUNT: u128 = 33_333_333 / 10; // dust
	const MAX_WEIGHT: u128 = 1_000_000_000;
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	Trappist::execute_with(|| {
		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(!<Trappist as Parachain>::System::events().iter().any(|r| matches!(
			r.event,
			trappist_runtime::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: None, id: alice_account.into() }.into();
		let native_asset: Assets = MultiAsset {
			id: Concrete(MultiLocation { parents: 0, interior: Here }),
			fun: Fungible((AMOUNT) as u128),
		}
		.into();
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = <Trappist as TrappistPallet>::XcmPallet::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 0);
	});
}
