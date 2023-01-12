use super::*;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::relay_chain::mock_paras_sudo_wrapper;
	use codec::Encode;
	use frame_support::{
		assert_ok,
		pallet_prelude::{DispatchResult, DispatchResultWithPostInfo},
		traits::PalletInfoAccess,
	};
	use sp_runtime::traits::{BlakeTwo256, Hash};
	use std::sync::Once;
	use trappist_runtime::constants::currency::EXISTENTIAL_DEPOSIT;
	use xcm::prelude::*;
	use xcm_executor::Assets;
	use xcm_simulator::{TestExt, Weight};

	static INIT: Once = Once::new();
	pub fn init_tracing() {
		INIT.call_once(|| {
			// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
			let _ = tracing_subscriber::fmt()
				.with_max_level(tracing::Level::TRACE)
				.with_env_filter("xcm=trace") // Comment out this line to see all traces
				.with_test_writer()
				.init();
		});
	}

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

			let origin: MultiLocation =
				Junction::AccountId32 { network: NetworkId::Polkadot, id: ALICE.into() }.into();

			let native_asset: Assets = MultiAsset {
				id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
				fun: Fungible((AMOUNT) as u128),
			}
			.into();

			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			assert!(System::events().iter().any(|r| matches!(
				r.event,
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
			)));

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

			let origin: MultiLocation =
				Junction::AccountId32 { network: NetworkId::Polkadot, id: ALICE.into() }.into();

			let native_asset: Assets = MultiAsset {
				id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
				fun: Fungible((AMOUNT) as u128),
			}
			.into();

			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			assert!(!System::events().iter().any(|r| matches!(
				r.event,
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
			)));

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
			assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD));
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

			let origin: MultiLocation =
				Junction::AccountId32 { network: NetworkId::Polkadot, id: ALICE.into() }.into();

			let fungible_asset_multi_location = MultiLocation {
				parents: 1,
				interior: X3(
					Parachain(ASSET_RESERVE_PARA_ID),
					PalletInstance(asset_reserve::Assets::index() as u8),
					GeneralIndex(xUSD as u128),
				),
			};

			let fungible_asset = MultiAsset {
				id: AssetId::Concrete(fungible_asset_multi_location.clone()),
				fun: Fungible(TRAP_AMOUNT),
			};

			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
					(fungible_asset_multi_location, TRAP_AMOUNT).into()
				)]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			assert!(System::events().iter().any(|r| matches!(
				r.event,
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
			)));

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
			assert_ok!(register_reserve_asset_on_trappist(ALICE, txUSD));
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

			let origin: MultiLocation =
				Junction::AccountId32 { network: NetworkId::Polkadot, id: ALICE.into() }.into();

			let fungible_asset_multi_location = MultiLocation {
				parents: 1,
				interior: X3(
					Parachain(ASSET_RESERVE_PARA_ID),
					PalletInstance(asset_reserve::Assets::index() as u8),
					GeneralIndex(xUSD as u128),
				),
			};

			let fungible_asset = MultiAsset {
				id: AssetId::Concrete(fungible_asset_multi_location.clone()),
				fun: Fungible(TRAP_AMOUNT),
			};

			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
					(fungible_asset_multi_location, TRAP_AMOUNT).into()
				)]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			assert!(!System::events().iter().any(|r| matches!(
				r.event,
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
			)));

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

			// Explicitly do not map derivative asset (txUSD) to multi-location (xUSD within Assets pallet on Reserve
			// Parachain) via Asset Registry
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

			let origin: MultiLocation =
				Junction::AccountId32 { network: NetworkId::Polkadot, id: ALICE.into() }.into();

			let fungible_asset_multi_location = MultiLocation {
				parents: 1,
				interior: X3(
					Parachain(ASSET_RESERVE_PARA_ID),
					PalletInstance(asset_reserve::Assets::index() as u8),
					GeneralIndex(xUSD as u128),
				),
			};

			let fungible_asset = MultiAsset {
				id: AssetId::Concrete(fungible_asset_multi_location.clone()),
				fun: Fungible(TRAP_AMOUNT),
			};

			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(
					(fungible_asset_multi_location, TRAP_AMOUNT).into()
				)]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			assert!(!System::events().iter().any(|r| matches!(
				r.event,
				RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
			)));

			let expected_versioned =
				VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
			let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

			// nothing was written into asset trap storage
			let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
			assert_eq!(read_asset_trap, 0);
		});
	}

	fn create_asset_on_asset_reserve(
		id: asset_reserve::AssetId,
		admin: asset_reserve::AccountId,
		min_balance: asset_reserve::Balance,
	) -> DispatchResult {
		asset_reserve::Assets::create(
			asset_reserve::RuntimeOrigin::signed(ALICE),
			id,
			admin.into(),
			min_balance,
		)
	}

	fn mint_asset_on_asset_reserve(
		asset_id: asset_reserve::AssetId,
		origin: asset_reserve::AccountId,
		mint_amount: asset_reserve::Balance,
	) -> DispatchResult {
		asset_reserve::Assets::mint(
			asset_reserve::RuntimeOrigin::signed(origin),
			asset_id,
			ALICE.into(),
			mint_amount,
		)
	}

	fn paras_sudo_wrapper_sudo_queue_downward_xcm<RuntimeCall: Encode>(call: RuntimeCall) {
		let sudo_queue_downward_xcm =
			relay_chain::RuntimeCall::ParasSudoWrapper(mock_paras_sudo_wrapper::Call::<
				relay_chain::Runtime,
			>::sudo_queue_downward_xcm {
				id: ParaId::new(ASSET_RESERVE_PARA_ID),
				xcm: Box::new(VersionedXcm::V2(Xcm(vec![Transact {
					origin_type: OriginKind::Superuser,
					require_weight_at_most: 10000000000u64,
					call: call.encode().into(),
				}]))),
			});

		assert_ok!(relay_chain::Sudo::sudo(
			relay_chain::RuntimeOrigin::signed(ALICE),
			Box::new(sudo_queue_downward_xcm),
		));
	}

	fn create_derivative_asset_on_trappist(
		id: trappist::AssetId,
		admin: trappist::AccountId,
		min_balance: trappist::Balance,
	) -> DispatchResult {
		trappist::Assets::create(
			trappist::RuntimeOrigin::signed(ALICE),
			id,
			admin.into(),
			min_balance,
		)
	}

	fn register_reserve_asset_on_trappist(
		origin: trappist::AccountId,
		asset_id: trappist::AssetId,
	) -> DispatchResultWithPostInfo {
		trappist::Sudo::sudo(
			trappist::RuntimeOrigin::signed(origin),
			Box::new(trappist::RuntimeCall::AssetRegistry(pallet_asset_registry::Call::<
				trappist::Runtime,
			>::register_reserve_asset {
				asset_id,
				asset_multi_location: (
					Parent,
					X3(
						Parachain(ASSET_RESERVE_PARA_ID),
						PalletInstance(asset_reserve::Assets::index() as u8),
						GeneralIndex(xUSD as u128),
					),
				)
					.into(),
			})),
		)
	}
}
