use super::*;
use frame_support::assert_ok;
use integration_tests_common::{constants::XCM_V3, ALICE};
use xcm_emulator::assert_expected_events;

#[allow(dead_code)]
fn overview() {
	type MockNetwork = RococoMockNet;
	type RelayChain = Rococo;
	type AssetReseve = AssetHubRococo;
	type Para = ParaA;
	type TestExt = dyn xcm_emulator::TestExt;
	let _messagesemulator = xcm_emulator::DOWNWARD_MESSAGES;
	type RuntimeA = <ParaA as Parachain>::Runtime;
	type XcmPallet = pallet_xcm::Pallet<RuntimeA>;
}

#[allow(non_upper_case_globals)]
const xUSD: u32 = 1;
#[allow(non_upper_case_globals)]
const txUSD: u32 = 10;

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

// Initiates a reserve-transfer of some asset on the asset reserve parachain to the trappist
// parachain (HRMP)
#[test]
fn reserve_transfer_asset_from_asset_reserve_parachain_to_trappist_parachain() {
	init_tracing();

	RococoMockNet::reset();

	let alice_account: sp_runtime::AccountId32 = get_account_id_from_seed::<sr25519::Public>(ALICE);

	const ASSET_MIN_BALANCE: Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	AssetHubRococo::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::create(
			<AssetHubRococo as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// Mint fungible asset
		assert_ok!(<AssetHubRococo as AssetHubRococoPallet>::Assets::mint(
			<AssetHubRococo as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			xUSD.into(),
			alice_account.clone().into(),
			MINT_AMOUNT
		));

		assert_eq!(
			<AssetHubRococo as AssetHubRococoPallet>::Assets::balance(xUSD, &alice_account),
			MINT_AMOUNT
		);
	});

	// Relay::execute_with(|| {
	// 	// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
	// 	paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
	// 		pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
	// 			id: xUSD,
	// 			owner: ALICE.into(),
	// 			issuer: ALICE.into(),
	// 			admin: ALICE.into(),
	// 			freezer: ALICE.into(),
	// 			min_balance: ASSET_MIN_BALANCE,
	// 			is_sufficient: true,
	// 			is_frozen: false,
	// 		},
	// 	));
	// });

	// let mut beneficiary_balance = 0;
	ParaA::execute_with(|| {
		// Create fungible asset on Asset Hub
		assert_ok!(<ParaA as ParaAPallet>::Assets::create(
			<ParaA as Parachain>::RuntimeOrigin::signed(alice_account.clone()),
			txUSD.into(),
			alice_account.clone().into(),
			ASSET_MIN_BALANCE
		));

		// 	// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// 	// Parachain) via Asset Registry
		// 	assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD, xUSD));
		// 	assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());

		// 	// Check beneficiary balance
		// 	beneficiary_balance = trappist::Assets::balance(txUSD, &ALICE);
	});

	// const AMOUNT: u128 = 20_000_000_000;

	// AssetReserve::execute_with(|| {
	// 	// Reserve parachain should be able to reserve-transfer an asset to Trappist Parachain
	// 	assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
	// 		asset_reserve::RuntimeOrigin::signed(ALICE),
	// 		Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
	// 		Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
	// 		Box::new(
	// 			(
	// 				X2(
	// 					PalletInstance(asset_reserve::Assets::index() as u8),
	// 					GeneralIndex(xUSD as u128)
	// 				),
	// 				AMOUNT
	// 			)
	// 				.into()
	// 		),
	// 		0,
	// 		WeightLimit::Unlimited,
	// 	));

	// 	// Ensure send amount moved to sovereign account
	// 	let sovereign_account = asset_reserve::sovereign_account(TRAPPIST_PARA_ID);
	// 	assert_eq!(asset_reserve::Assets::balance(xUSD, &sovereign_account), AMOUNT);
	// });

	// const EST_FEES: u128 = 1_600_000_000 * 10;
	// Trappist::execute_with(|| {
	// 	// Ensure beneficiary account balance increased
	// 	let current_balance = trappist::Assets::balance(txUSD, &ALICE);
	// 	assert_balance(current_balance, beneficiary_balance + AMOUNT, EST_FEES);
	// 	println!(
	// 		"Reserve-transfer: initial balance {} transfer amount {} current balance {} estimated fees {} actual fees {}",
	// 		beneficiary_balance.separate_with_commas(),
	// 		AMOUNT.separate_with_commas(),
	// 		current_balance.separate_with_commas(),
	// 		EST_FEES.separate_with_commas(),
	// 		(beneficiary_balance + AMOUNT - current_balance).separate_with_commas()
	// 	);
	// });
}

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
