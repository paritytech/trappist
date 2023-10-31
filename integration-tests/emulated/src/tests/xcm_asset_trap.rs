use crate::tests::*;
/*
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
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(<Trappist as Chain>::System::events().iter().any(|r| matches!(
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
*/

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
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset((Here, AMOUNT).into())]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(!<Trappist as Chain>::System::events().iter().any(|r| matches!(
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

/*
// make sure we can trap known derivative fungibles
#[test]
fn fungible_trap_works() {
	init_tracing();

	//Reboot test network to init
	RococoMockNet::reset();

	// Get account from const seed. Funded on genesis config.
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	const ASSET_MIN_BALANCE: Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	// Create and mint fungible asset on Reserve Parachain

	AssetHubRococo::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::create(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Mint fungible asset
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::mint(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			MINT_AMOUNT
		));

		// Assert balance update
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &alice_account),
			MINT_AMOUNT
		);
	});

	// Make asset sufficient from Relay to Reserve Parachain

	// Pallet Asset called to be transacted from Relay to Reserve Parachain
	let call = <AssetHubRococo as Chain>::RuntimeCall::Assets(pallet_assets::Call::<
		<AssetHubRococo as Chain>::Runtime,
		Instance1,
	>::force_asset_status {
		id: xUSD.into(),
		owner: alice_account.clone().into(),
		issuer: alice_account.clone().into(),
		admin: alice_account.clone().into(),
		freezer: alice_account.clone().into(),
		min_balance: ASSET_MIN_BALANCE,
		is_sufficient: true,
		is_frozen: false,
	})
	.encode()
	.into();

	// Send arguments to be sent from Relay to Reserve Parachain via pallet-xcm
	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let assets_para_destination: VersionedMultiLocation =
		Rococo::child_location_of(AssetHubRococo::para_id()).into();

	let weight_limit = WeightLimit::Unlimited;
	let require_weight_at_most = Weight::from_parts(1000000000, 200000);
	let origin_kind = OriginKind::Superuser;
	let check_origin = None;

	// Reserve barrier requires explicit unpaid execution and accepts parent governance as source
	let xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit, check_origin },
		Transact { require_weight_at_most, origin_kind, call },
	]));

	Rococo::execute_with(|| {
		// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send(
			sudo_origin,
			bx!(assets_para_destination),
			bx!(xcm),
		));
	});

	// Create asset on Trappist and map to Asset Registry

	let mut beneficiary_balance = 0;

	// Call for asset regitry to be mapped to Trappist - Requires sudo
	let asset_registry_call =
		<Trappist as Chain>::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
			<Trappist as Chain>::Runtime,
		>::register_reserve_asset {
			asset_id: txUSD,
			asset_multi_location: (
				Parent,
				X3(
					Parachain(ASSET_HUB_ID),
					PalletInstance(<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8),
					GeneralIndex(xUSD as u128),
				),
			)
				.into(),
		});

	Trappist::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<Trappist as TrappistPallet>::Assets::create(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// Parachain) via Asset Registry
		assert_ok!(<Trappist as TrappistPallet>::Sudo::sudo(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(asset_registry_call),
		),);
		assert!(
			<Trappist as TrappistPallet>::AssetRegistry::get_asset_multi_location(txUSD).is_some()
		);

		// // Check beneficiary balance
		beneficiary_balance =
			<Trappist as TrappistPallet>::Assets::balance(txUSD, &alice_account.clone());
	});

	// Reserve asset transfer from Asset Hub to Trappist

	const AMOUNT: u128 = 20_000_000_000;

	AssetHubRococo::execute_with(|| {
		// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
		assert_ok!(
			<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::limited_reserve_transfer_assets(
				<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
				Box::new((Parent, Parachain(TRAPPIST_ID)).into()),
				Box::new(
					X1(AccountId32 { network: None, id: alice_account.clone().into() }).into(),
				),
				Box::new(
					vec![(
						X2(
							PalletInstance(
								<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8
							),
							GeneralIndex(xUSD as u128)
						),
						AMOUNT
					)
						.into()]
					.into()
				),
				0,
				WeightLimit::Unlimited,
			)
		);

		// Ensure send amount moved to sovereign account
		let sovereign_account = AssetHubRococo::sovereign_account_id_of(MultiLocation {
			parents: 1,
			interior: Parachain(TRAPPIST_ID.into()).into(),
		});
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &sovereign_account),
			AMOUNT
		);
	});

	// the actual test starts here
	Trappist::execute_with(|| {
		const TRAP_AMOUNT: u128 = ASSET_MIN_BALANCE * 10;
		const MAX_WEIGHT: u128 = 1_000_000_000;

		let fungible_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(ASSET_HUB_ID),
				PalletInstance(<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8),
				GeneralIndex(xUSD as u128),
			),
		};

		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
				(fungible_asset_multi_location.clone(), TRAP_AMOUNT).into()
			)]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(<Trappist as Chain>::System::events().iter().any(|r| matches!(
			r.event,
			trappist_runtime::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: None, id: alice_account.into() }.into();
		let fungible_asset =
			MultiAsset { id: Concrete(fungible_asset_multi_location), fun: Fungible(TRAP_AMOUNT) };
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = <Trappist as TrappistPallet>::XcmPallet::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 1);
	});
}
*/

// make sure we can trap known derivative fungibles
#[test]
fn fungible_dust_trap_doesnt_work() {
	init_tracing();

	//Reboot test network to init
	RococoMockNet::reset();

	// Get account from const seed. Funded on genesis config.
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	const ASSET_MIN_BALANCE: Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	// Create and mint fungible asset on Reserve Parachain

	AssetHubRococo::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::create(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Mint fungible asset
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::mint(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			MINT_AMOUNT
		));

		// Assert balance update
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &alice_account),
			MINT_AMOUNT
		);
	});

	// Make asset sufficient from Relay to Reserve Parachain

	// Pallet Asset called to be transacted from Relay to Reserve Parachain
	let call = <AssetHubRococo as Chain>::RuntimeCall::Assets(pallet_assets::Call::<
		<AssetHubRococo as Chain>::Runtime,
		Instance1,
	>::force_asset_status {
		id: xUSD.into(),
		owner: alice_account.clone().into(),
		issuer: alice_account.clone().into(),
		admin: alice_account.clone().into(),
		freezer: alice_account.clone().into(),
		min_balance: ASSET_MIN_BALANCE,
		is_sufficient: true,
		is_frozen: false,
	})
	.encode()
	.into();

	// Send arguments to be sent from Relay to Reserve Parachain via pallet-xcm
	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let assets_para_destination: VersionedMultiLocation =
		Rococo::child_location_of(AssetHubRococo::para_id()).into();

	let weight_limit = WeightLimit::Unlimited;
	let require_weight_at_most = Weight::from_parts(1000000000, 200000);
	let origin_kind = OriginKind::Superuser;
	let check_origin = None;

	// Reserve barrier requires explicit unpaid execution and accepts parent governance as source
	let xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit, check_origin },
		Transact { require_weight_at_most, origin_kind, call },
	]));

	Rococo::execute_with(|| {
		// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send(
			sudo_origin,
			bx!(assets_para_destination),
			bx!(xcm),
		));
	});

	// Create asset on Trappist and map to Asset Registry

	let mut beneficiary_balance = 0;

	// Call for asset regitry to be mapped to Trappist - Requires sudo
	let asset_registry_call =
		<Trappist as Chain>::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
			<Trappist as Chain>::Runtime,
		>::register_reserve_asset {
			asset_id: txUSD,
			asset_multi_location: (
				Parent,
				X3(
					Parachain(ASSET_HUB_ID),
					PalletInstance(<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8),
					GeneralIndex(xUSD as u128),
				),
			)
				.into(),
		});

	Trappist::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<Trappist as TrappistPallet>::Assets::create(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// Parachain) via Asset Registry
		assert_ok!(<Trappist as TrappistPallet>::Sudo::sudo(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(asset_registry_call),
		),);
		assert!(
			<Trappist as TrappistPallet>::AssetRegistry::get_asset_multi_location(txUSD).is_some()
		);

		// // Check beneficiary balance
		beneficiary_balance =
			<Trappist as TrappistPallet>::Assets::balance(txUSD, &alice_account.clone());
	});

	// Reserve asset transfer from Asset Hub to Trappist

	const AMOUNT: u128 = 20_000_000_000;

	AssetHubRococo::execute_with(|| {
		// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
		assert_ok!(
			<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::limited_reserve_transfer_assets(
				<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
				Box::new((Parent, Parachain(TRAPPIST_ID)).into()),
				Box::new(
					X1(AccountId32 { network: None, id: alice_account.clone().into() }).into(),
				),
				Box::new(
					vec![(
						X2(
							PalletInstance(
								<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8
							),
							GeneralIndex(xUSD as u128)
						),
						AMOUNT
					)
						.into()]
					.into()
				),
				0,
				WeightLimit::Unlimited,
			)
		);

		// Ensure send amount moved to sovereign account
		let sovereign_account = AssetHubRococo::sovereign_account_id_of(MultiLocation {
			parents: 1,
			interior: Parachain(TRAPPIST_ID.into()).into(),
		});
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &sovereign_account),
			AMOUNT
		);
	});

	// the actual test starts here
	Trappist::execute_with(|| {
		const TRAP_AMOUNT: u128 = ASSET_MIN_BALANCE / 10;
		const MAX_WEIGHT: u128 = 1_000_000_000;

		let fungible_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(ASSET_HUB_ID),
				PalletInstance(<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8),
				GeneralIndex(xUSD as u128),
			),
		};

		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
				(fungible_asset_multi_location.clone(), TRAP_AMOUNT).into()
			)]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(!<Trappist as Chain>::System::events().iter().any(|r| matches!(
			r.event,
			trappist_runtime::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: None, id: alice_account.into() }.into();
		let fungible_asset =
			MultiAsset { id: Concrete(fungible_asset_multi_location), fun: Fungible(TRAP_AMOUNT) };
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = <Trappist as TrappistPallet>::XcmPallet::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 0);
	});
}

// make sure that unknown fungibles (not on AssetRegistry) do not get trapped
#[test]
fn fungible_non_registered_trap_doesnt_work() {
	init_tracing();

	//Reboot test network to init
	RococoMockNet::reset();

	// Get account from const seed. Funded on genesis config.
	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	const ASSET_MIN_BALANCE: Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	// Create and mint fungible asset on Reserve Parachain

	AssetHubRococo::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::create(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Mint fungible asset
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::mint(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			MINT_AMOUNT
		));

		// Assert balance update
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &alice_account),
			MINT_AMOUNT
		);
	});

	// Make asset sufficient from Relay to Reserve Parachain

	// Pallet Asset called to be transacted from Relay to Reserve Parachain
	let call = <AssetHubRococo as Chain>::RuntimeCall::Assets(pallet_assets::Call::<
		<AssetHubRococo as Chain>::Runtime,
		Instance1,
	>::force_asset_status {
		id: xUSD.into(),
		owner: alice_account.clone().into(),
		issuer: alice_account.clone().into(),
		admin: alice_account.clone().into(),
		freezer: alice_account.clone().into(),
		min_balance: ASSET_MIN_BALANCE,
		is_sufficient: true,
		is_frozen: false,
	})
	.encode()
	.into();

	// Send arguments to be sent from Relay to Reserve Parachain via pallet-xcm
	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let assets_para_destination: VersionedMultiLocation =
		Rococo::child_location_of(AssetHubRococo::para_id()).into();

	let weight_limit = WeightLimit::Unlimited;
	let require_weight_at_most = Weight::from_parts(1000000000, 200000);
	let origin_kind = OriginKind::Superuser;
	let check_origin = None;

	// Reserve barrier requires explicit unpaid execution and accepts parent governance as source
	let xcm = VersionedXcm::from(Xcm(vec![
		UnpaidExecution { weight_limit, check_origin },
		Transact { require_weight_at_most, origin_kind, call },
	]));

	Rococo::execute_with(|| {
		// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
		assert_ok!(<Rococo as RococoPallet>::XcmPallet::send(
			sudo_origin,
			bx!(assets_para_destination),
			bx!(xcm),
		));
	});

	// Create asset on Trappist and map to Asset Registry

	let mut beneficiary_balance = 0;

	Trappist::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<Trappist as TrappistPallet>::Assets::create(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Explicitly do not map derivative asset (txUSD) to multi-location (xUSD within Assets
		// pallet on Reserve Parachain) via Asset Registry

		// // Check beneficiary balance
		beneficiary_balance =
			<Trappist as TrappistPallet>::Assets::balance(txUSD, &alice_account.clone());
	});

	// Reserve asset transfer from Asset Hub to Trappist

	const AMOUNT: u128 = 20_000_000_000;

	AssetHubRococo::execute_with(|| {
		// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
		assert_ok!(
			<AssetHubRococo as AssetHubRococoPallet>::PolkadotXcm::limited_reserve_transfer_assets(
				<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
				Box::new((Parent, Parachain(TRAPPIST_ID)).into()),
				Box::new(
					X1(AccountId32 { network: None, id: alice_account.clone().into() }).into(),
				),
				Box::new(
					vec![(
						X2(
							PalletInstance(
								<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8
							),
							GeneralIndex(xUSD as u128)
						),
						AMOUNT
					)
						.into()]
					.into()
				),
				0,
				WeightLimit::Unlimited,
			)
		);

		// Ensure send amount moved to sovereign account
		let sovereign_account = AssetHubRococo::sovereign_account_id_of(MultiLocation {
			parents: 1,
			interior: Parachain(TRAPPIST_ID.into()).into(),
		});
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &sovereign_account),
			AMOUNT
		);
	});

	// the actual test starts here
	Trappist::execute_with(|| {
		const TRAP_AMOUNT: u128 = ASSET_MIN_BALANCE * 10;
		const MAX_WEIGHT: u128 = 1_000_000_000;

		let fungible_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(ASSET_HUB_ID),
				PalletInstance(<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8),
				GeneralIndex(xUSD as u128),
			),
		};

		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
				(fungible_asset_multi_location.clone(), TRAP_AMOUNT).into()
			)]))),
			(MAX_WEIGHT as u64).into()
		));

		assert!(!<Trappist as Chain>::System::events().iter().any(|r| matches!(
			r.event,
			trappist_runtime::RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: None, id: alice_account.into() }.into();
		let fungible_asset =
			MultiAsset { id: Concrete(fungible_asset_multi_location), fun: Fungible(TRAP_AMOUNT) };
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = <Trappist as TrappistPallet>::XcmPallet::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 0);
	});
}
