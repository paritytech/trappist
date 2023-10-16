use crate::tests::*;

#[allow(dead_code)]
fn overview() {
	type MockNetwork = RococoMockNet;
	type RelayChain = Rococo;
	type AssetReseve = AssetHubRococo;
	type Para = Trappist;
	type TestExt = dyn xcm_emulator::TestExt;
	let _messagesemulator = xcm_emulator::DOWNWARD_MESSAGES;
	type RuntimeA = <Trappist as Chain>::Runtime;
	type XcmPallet = pallet_xcm::Pallet<RuntimeA>;
}

// Initiates a reserve-transfer of some asset on the asset reserve parachain to the trappist
// parachain (HRMP)
#[test]
fn reserve_transfer_asset_from_asset_reserve_parachain_to_trappist_parachain() {
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

	// Check that balance increased on Trappist

	Trappist::execute_with(|| {
		// Ensure beneficiary account balance increased
		let current_balance = <Trappist as TrappistPallet>::Assets::balance(txUSD, alice_account);
		assert!(current_balance > 0u128.into());
		println!(
			"Reserve-transfer: initial balance {} transfer amount {} current balance {} actual fees {}",
			beneficiary_balance,
			AMOUNT,
			current_balance,
			(beneficiary_balance + AMOUNT - current_balance)
		);
	});
}

// Initiates a send of a XCM message from trappist to the asset reserve parachain, instructing
// it to transfer some amount of a fungible asset to some tertiary (stout) parachain (HRMP)
#[test]
fn two_hop_reserve_transfer_from_trappist_parachain_to_tertiary_parachain() {
	init_tracing();

	RococoMockNet::reset();

	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);
	let trappist_sovereign_account = AssetHubRococo::sovereign_account_id_of(MultiLocation {
		parents: 1,
		interior: Parachain(TRAPPIST_ID.into()).into(),
	});
	let stout_sovereign_account = AssetHubRococo::sovereign_account_id_of(MultiLocation {
		parents: 1,
		interior: Parachain(STOUT_ID.into()).into(),
	});

	const ASSET_MIN_BALANCE: Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 100_000_000_000;

	AssetHubRococo::execute_with(|| {
		// Create and mint fungible asset on Reserve Parachain
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
			MINT_AMOUNT * 2
		));

		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Balances::transfer(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			trappist_sovereign_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Touch parachain account
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::transfer(
			<AssetHubRococo as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			trappist_sovereign_account.into(),
			MINT_AMOUNT
		));
	});

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

	// XcmPallet send arguments
	let sudo_origin = <Rococo as Chain>::RuntimeOrigin::root();
	let assets_para_destination: VersionedMultiLocation =
		Rococo::child_location_of(AssetHubRococo::para_id()).into();

	let weight_limit = WeightLimit::Unlimited;
	let require_weight_at_most = Weight::from_parts(1000000000, 200000);
	let origin_kind = OriginKind::Superuser;
	let check_origin = None;

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

	let stout_asset_registry_call =
		<Stout as Chain>::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
			<Stout as Chain>::Runtime,
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

	let mut beneficiary_balance = 0;

	Stout::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<Stout as StoutPallet>::Assets::create(
			<Stout as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// Parachain) via Asset Registry
		assert_ok!(<Stout as StoutPallet>::Sudo::sudo(
			<Stout as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(stout_asset_registry_call),
		),);
		assert!(<Stout as StoutPallet>::AssetRegistry::get_asset_multi_location(txUSD).is_some());

		// // Check beneficiary balance
		beneficiary_balance =
			<Stout as StoutPallet>::Assets::balance(txUSD, &alice_account.clone());
	});

	let trappist_asset_registry_call =
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

	const MAX_WEIGHT: u128 = 1_000_000_000 * 2; // 1,000,000,000 per instruction
	const EXECUTION_COST: u128 = 65_000_000_000;

	Trappist::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<Trappist as TrappistPallet>::Assets::create(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Mint fungible asset
		assert_ok!(<Trappist as TrappistPallet>::Assets::mint(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			MINT_AMOUNT
		));

		// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// Parachain) via Asset Registry
		assert_ok!(<Trappist as TrappistPallet>::Sudo::sudo(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(trappist_asset_registry_call),
		),);
		assert!(
			<Trappist as TrappistPallet>::AssetRegistry::get_asset_multi_location(txUSD).is_some()
		);

		// // Check beneficiary balance
		beneficiary_balance =
			<Trappist as TrappistPallet>::Assets::balance(txUSD, &alice_account.clone());

		// Trappist parachain should be able to reserve-transfer an asset to Tertiary Parachain
		// Call must be hand constructed as pallet-xcm reserve-transfer call does not support third party reserve
		assert_ok!(<Trappist as TrappistPallet>::XcmPallet::execute(
			<Trappist as Chain>::RuntimeOrigin::signed(alice_account.clone()),
			Box::new(VersionedXcm::from(Xcm(vec![
				// Withdraw asset from Trappist Parachain
				WithdrawAsset(
					(
						(
							Parent,
							X3(
								Parachain(ASSET_HUB_ID),
								PalletInstance(
									<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8
								),
								GeneralIndex(xUSD as u128)
							)
						),
						MINT_AMOUNT
					)
						.into()
				),
				// Initiate reserve-transfer of asset
				InitiateReserveWithdraw {
					assets: Wild(AllCounted(1)),
					reserve: (Parent, Parachain(ASSET_HUB_ID)).into(),
					// This part of the message is intended to be executed on Asset Hub
					xcm: Xcm(vec![
						// Buy execution from Asset Hub
						BuyExecution {
							fees: (
								X2(
									PalletInstance(
										<AssetHubRococo as AssetHubRococoPallet>::Assets::index()
											as u8
									),
									GeneralIndex(xUSD as u128)
								),
								EXECUTION_COST
							)
								.into(),
							weight_limit: Unlimited
						},
						// At this point tokens are moved from one sovereign account to another
						DepositReserveAsset {
							assets: Wild(AllCounted(1)),
							dest: (Parent, Parachain(STOUT_ID)).into(),
							// This part of the message is intended to be executed by Stout
							xcm: Xcm(vec![
								// Buy execution on Stout
								BuyExecution {
									fees: (
										(Parent,
										X3(
											Parachain(ASSET_HUB_ID),
											PalletInstance(<AssetHubRococo as AssetHubRococoPallet>::Assets::index() as u8),
											GeneralIndex(xUSD as u128),
										)),
										EXECUTION_COST
									)
										.into(),
									weight_limit: Unlimited
								},
								// Deposit asset (derivative) to beneficiary account
								DepositAsset {
								assets: Wild(AllCounted(1)),
								beneficiary: X1(AccountId32 {
									network: None,
									id: alice_account.clone().into()
								})
								.into()
							}
							])
						}
					])
				},
			]))),
			(MAX_WEIGHT as u64).into(),
		));
	});

	AssetHubRococo::execute_with(|| {
		// Check send amount moved to sovereign account
		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(
				xUSD,
				&stout_sovereign_account
			),
			92813999939 //Mint amount minus fees
		);
	});

	Stout::execute_with(|| {
		// Ensure beneficiary received amount, less fees
		let current_balance = <Stout as StoutPallet>::Assets::balance(txUSD, &alice_account);
		assert!(current_balance > 0u128.into());
		println!(
			"Two-hop Reserve-transfer: initial balance {} transfer amount {} current balance {} estimated fees {} actual fees {}",
			beneficiary_balance.separate_with_commas(),
			MINT_AMOUNT.separate_with_commas(),
			current_balance.separate_with_commas(),
			EXECUTION_COST.separate_with_commas(),
			(beneficiary_balance + MINT_AMOUNT - current_balance).separate_with_commas()
		);
	});
}
