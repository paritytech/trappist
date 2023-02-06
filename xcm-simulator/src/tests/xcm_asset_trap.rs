use crate::tests::*;
use frame_support::{assert_ok, traits::PalletInfoAccess};
use sp_runtime::traits::{BlakeTwo256, Hash};
use trappist_runtime::constants::currency::EXISTENTIAL_DEPOSIT;
use xcm_executor::Assets;
use xcm_simulator::{TestExt, Weight};

#[allow(non_upper_case_globals)]
const xUSD: u32 = 1;
#[allow(non_upper_case_globals)]
const txUSD: u32 = 10;

// make sure we can trap a native asset
#[test]
fn native_trap_works() {
	init_tracing();

	MockNet::reset();

	const AMOUNT: u128 = EXISTENTIAL_DEPOSIT * 10;
	const MAX_WEIGHT: u128 = 1_000_000_000;

	Trappist::execute_with(|| {
		use trappist::{RuntimeEvent, System};

		assert_ok!(trappist::PolkadotXcm::execute(
			trappist::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
			MAX_WEIGHT as u64
		));

		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: Polkadot, id: ALICE.into() }.into();
		let native_asset: Assets = MultiAsset {
			id: Concrete(MultiLocation { parents: 0, interior: Here }),
			fun: Fungible((AMOUNT) as u128),
		}
		.into();
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 1);
	});
}

// make sure that native dust is not trapped
#[test]
fn native_dust_trap_doesnt_work() {
	init_tracing();

	MockNet::reset();

	const AMOUNT: u128 = EXISTENTIAL_DEPOSIT / 10; // dust
	const MAX_WEIGHT: u128 = 1_000_000_000;

	Trappist::execute_with(|| {
		use trappist::{RuntimeEvent, System};

		assert_ok!(trappist::PolkadotXcm::execute(
			trappist::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
			MAX_WEIGHT as u64
		));

		assert!(!System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: Polkadot, id: ALICE.into() }.into();
		let native_asset: Assets = MultiAsset {
			id: Concrete(MultiLocation { parents: 0, interior: Here }),
			fun: Fungible((AMOUNT) as u128),
		}
		.into();
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 0);
	});
}

// make sure we can trap known derivative fungibles
#[test]
fn fungible_trap_works() {
	init_tracing();

	MockNet::reset();

	const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	AssetReserve::execute_with(|| {
		// Create fungible asset on Reserve Parachain
		assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));

		// Mint fungible asset
		assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, MINT_AMOUNT));
		assert_eq!(asset_reserve::Assets::balance(xUSD, &ALICE), MINT_AMOUNT);
	});

	Relay::execute_with(|| {
		// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
		paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
			pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
				id: xUSD,
				owner: ALICE.into(),
				issuer: ALICE.into(),
				admin: ALICE.into(),
				freezer: ALICE.into(),
				min_balance: ASSET_MIN_BALANCE,
				is_sufficient: true,
				is_frozen: false,
			},
		));
	});

	Trappist::execute_with(|| {
		// Create derivative asset on Trappist Parachain
		assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

		// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// Parachain) via Asset Registry
		assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD, xUSD));
		assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());
	});

	const AMOUNT: u128 = ASSET_MIN_BALANCE * 20;

	AssetReserve::execute_with(|| {
		assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
			asset_reserve::RuntimeOrigin::signed(ALICE),
			Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
			Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
			Box::new(
				(
					X2(
						PalletInstance(asset_reserve::Assets::index() as u8),
						GeneralIndex(xUSD as u128)
					),
					AMOUNT
				)
					.into()
			),
			0,
			WeightLimit::Unlimited,
		));
	});

	// the actual test starts here
	Trappist::execute_with(|| {
		use trappist::{RuntimeEvent, System};

		const TRAP_AMOUNT: u128 = ASSET_MIN_BALANCE * 10;
		const MAX_WEIGHT: u128 = 1_000_000_000;

		let fungible_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(ASSET_RESERVE_PARA_ID),
				PalletInstance(asset_reserve::Assets::index() as u8),
				GeneralIndex(xUSD as u128),
			),
		};

		assert_ok!(trappist::PolkadotXcm::execute(
			trappist::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
				(fungible_asset_multi_location.clone(), TRAP_AMOUNT).into()
			)]))),
			MAX_WEIGHT as u64
		));

		assert!(System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: Polkadot, id: ALICE.into() }.into();
		let fungible_asset =
			MultiAsset { id: Concrete(fungible_asset_multi_location), fun: Fungible(TRAP_AMOUNT) };
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 1);
	});
}

// make sure we can trap known derivative fungibles
#[test]
fn fungible_dust_trap_doesnt_work() {
	init_tracing();

	MockNet::reset();

	const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	AssetReserve::execute_with(|| {
		// Create fungible asset on Reserve Parachain
		assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));

		// Mint fungible asset
		assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, MINT_AMOUNT));
		assert_eq!(asset_reserve::Assets::balance(xUSD, &ALICE), MINT_AMOUNT);
	});

	Relay::execute_with(|| {
		// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
		paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
			pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
				id: xUSD,
				owner: ALICE.into(),
				issuer: ALICE.into(),
				admin: ALICE.into(),
				freezer: ALICE.into(),
				min_balance: ASSET_MIN_BALANCE,
				is_sufficient: true,
				is_frozen: false,
			},
		));
	});

	Trappist::execute_with(|| {
		// Create derivative asset on Trappist Parachain
		assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

		// Map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
		// Parachain) via Asset Registry
		assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD, xUSD));
		assert!(trappist::AssetRegistry::asset_id_multilocation(txUSD).is_some());
	});

	const AMOUNT: u128 = ASSET_MIN_BALANCE * 20;

	AssetReserve::execute_with(|| {
		assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
			asset_reserve::RuntimeOrigin::signed(ALICE),
			Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
			Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
			Box::new(
				(
					X2(
						PalletInstance(asset_reserve::Assets::index() as u8),
						GeneralIndex(xUSD as u128)
					),
					AMOUNT
				)
					.into()
			),
			0,
			WeightLimit::Unlimited,
		));
	});

	// the actual test starts here
	Trappist::execute_with(|| {
		use trappist::{RuntimeEvent, System};

		const TRAP_AMOUNT: u128 = ASSET_MIN_BALANCE / 10; // dust
		const MAX_WEIGHT: u128 = 1_000_000_000;

		let fungible_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(ASSET_RESERVE_PARA_ID),
				PalletInstance(asset_reserve::Assets::index() as u8),
				GeneralIndex(xUSD as u128),
			),
		};

		assert_ok!(trappist::PolkadotXcm::execute(
			trappist::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
				(fungible_asset_multi_location.clone(), TRAP_AMOUNT).into()
			)]))),
			MAX_WEIGHT as u64
		));

		assert!(!System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: Polkadot, id: ALICE.into() }.into();
		let fungible_asset =
			MultiAsset { id: Concrete(fungible_asset_multi_location), fun: Fungible(TRAP_AMOUNT) };
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 0);
	});
}

// make sure that unknown fungibles (not on AssetRegistry) do not get trapped
#[test]
fn fungible_non_registered_trap_doesnt_work() {
	init_tracing();

	MockNet::reset();

	const ASSET_MIN_BALANCE: asset_reserve::Balance = 1_000_000_000;
	const MINT_AMOUNT: u128 = 1_000_000_000_000_000_000;

	AssetReserve::execute_with(|| {
		// Create fungible asset on Reserve Parachain
		assert_ok!(create_asset_on_asset_reserve(xUSD, ALICE, ASSET_MIN_BALANCE));

		// Mint fungible asset
		assert_ok!(mint_asset_on_asset_reserve(xUSD, ALICE, MINT_AMOUNT));
		assert_eq!(asset_reserve::Assets::balance(xUSD, &ALICE), MINT_AMOUNT);
	});

	Relay::execute_with(|| {
		// Declare xUSD (on Reserve Parachain) as self-sufficient via Relay Chain
		paras_sudo_wrapper_sudo_queue_downward_xcm(asset_reserve::RuntimeCall::Assets(
			pallet_assets::Call::<asset_reserve::Runtime>::force_asset_status {
				id: xUSD,
				owner: ALICE.into(),
				issuer: ALICE.into(),
				admin: ALICE.into(),
				freezer: ALICE.into(),
				min_balance: ASSET_MIN_BALANCE,
				is_sufficient: true,
				is_frozen: false,
			},
		));
	});

	Trappist::execute_with(|| {
		// Create derivative asset on Trappist Parachain
		assert_ok!(create_derivative_asset_on_trappist(txUSD, ALICE.into(), ASSET_MIN_BALANCE));

		// Explicitly do not map derivative asset (txUSD) to multi-location (xUSD within Assets
		// pallet on Reserve Parachain) via Asset Registry
	});

	const AMOUNT: u128 = ASSET_MIN_BALANCE * 20;

	AssetReserve::execute_with(|| {
		assert_ok!(asset_reserve::PolkadotXcm::limited_reserve_transfer_assets(
			asset_reserve::RuntimeOrigin::signed(ALICE),
			Box::new((Parent, Parachain(TRAPPIST_PARA_ID)).into()),
			Box::new(X1(AccountId32 { network: Any, id: ALICE.into() }).into().into()),
			Box::new(
				(
					X2(
						PalletInstance(asset_reserve::Assets::index() as u8),
						GeneralIndex(xUSD as u128)
					),
					AMOUNT
				)
					.into()
			),
			0,
			WeightLimit::Unlimited,
		));
	});

	// the actual test starts here
	Trappist::execute_with(|| {
		use trappist::{RuntimeEvent, System};

		const TRAP_AMOUNT: u128 = ASSET_MIN_BALANCE * 10;
		const MAX_WEIGHT: u128 = 1_000_000_000;

		let fungible_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(ASSET_RESERVE_PARA_ID),
				PalletInstance(asset_reserve::Assets::index() as u8),
				GeneralIndex(xUSD as u128),
			),
		};

		assert_ok!(trappist::PolkadotXcm::execute(
			trappist::RuntimeOrigin::signed(ALICE),
			Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
				(fungible_asset_multi_location.clone(), TRAP_AMOUNT).into()
			)]))),
			MAX_WEIGHT as u64
		));

		assert!(!System::events().iter().any(|r| matches!(
			r.event,
			RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
		)));

		let origin: MultiLocation = AccountId32 { network: Polkadot, id: ALICE.into() }.into();
		let fungible_asset =
			MultiAsset { id: Concrete(fungible_asset_multi_location), fun: Fungible(TRAP_AMOUNT) };
		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, 0);
	});
}
