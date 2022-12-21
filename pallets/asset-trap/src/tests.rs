use crate::{mock::*, Event};
use frame_support::assert_ok;
use sp_runtime::{
	traits::{BlakeTwo256, Hash},
	AccountId32,
};
use xcm::{latest::prelude::*, VersionedMultiAssets};
use xcm_executor::{
	traits::{ClaimAssets, DropAssets},
	Assets as XcmAssets,
};

const ALICE: AccountId32 = AccountId32::new([0u8; 32]);

fn register_reserve_asset() {
	let statemine_para_id = StatemineParaIdInfo::get();
	let statemine_assets_pallet = StatemineAssetsInstanceInfo::get();
	let statemine_asset_id = StatemineAssetIdInfo::get();
	let local_asset_id = LocalAssetId::get();

	let statemine_asset_multi_location = MultiLocation {
		parents: 1,
		interior: X3(
			Parachain(statemine_para_id),
			PalletInstance(statemine_assets_pallet),
			GeneralIndex(statemine_asset_id),
		),
	};

	assert_ok!(AssetRegistry::register_reserve_asset(
		RuntimeOrigin::root(),
		local_asset_id,
		statemine_asset_multi_location.clone(),
	));
}

fn statemine_asset_multi_location() -> MultiLocation {
	let statemine_para_id = StatemineParaIdInfo::get();
	let statemine_assets_pallet = StatemineAssetsInstanceInfo::get();
	let statemine_asset_id = StatemineAssetIdInfo::get();

	MultiLocation {
		parents: 1,
		interior: X3(
			Parachain(statemine_para_id),
			PalletInstance(statemine_assets_pallet),
			GeneralIndex(statemine_asset_id),
		),
	}
}

// make sure we can trap and claim a native asset
#[test]
fn native_trap_claim_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
			fun: Fungible((CurrencyMinBalance::get() * 10) as u128),
		}
		.into();

		AssetTrap::drop_assets(&origin, native_asset.clone());

		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		System::assert_has_event(
			Event::AssetTrapped {
				0: expected_hash,
				1: origin.clone(),
				2: expected_versioned.clone(),
			}
			.into(),
		);

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(
			read_asset_trap,
			Some((MultiLocation { parents: 0, interior: Junctions::Here }, 1))
		);

		// claim the asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into());

		System::assert_has_event(
			Event::AssetClaimed { 0: expected_hash, 1: origin.clone(), 2: expected_versioned }
				.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, None);
	});
}

// make sure that native dust is not trapped
#[test]
fn native_dust_trap_doesnt_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
			fun: Fungible((CurrencyMinBalance::get() / 10) as u128),
		}
		.into();

		AssetTrap::drop_assets(&origin, native_asset.clone());

		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, None);
	});
}

// make sure we can trap and claim known derivative fungibles
#[test]
fn fungible_trap_claim_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// make sure asset exists on AssetRegistry
		register_reserve_asset();

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let fungible_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(statemine_asset_multi_location()),
			fun: Fungible((LocalAssetMinBalance::get() * 10) as u128),
		}
		.into();

		AssetTrap::drop_assets(&origin, fungible_asset.clone());

		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		System::assert_has_event(
			Event::AssetTrapped {
				0: expected_hash,
				1: origin.clone(),
				2: expected_versioned.clone(),
			}
			.into(),
		);

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, Some((statemine_asset_multi_location(), 1)));

		// claim the asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &fungible_asset.into());

		System::assert_has_event(
			Event::AssetClaimed { 0: expected_hash, 1: origin.clone(), 2: expected_versioned }
				.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, None);
	});
}

// make sure that fungible dust does not get trapped
#[test]
fn fungible_dust_trap_doesnt_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// make sure asset exists on AssetRegistry
		register_reserve_asset();

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let fungible_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(statemine_asset_multi_location()),
			fun: Fungible((LocalAssetMinBalance::get() / 10) as u128),
		}
		.into();

		AssetTrap::drop_assets(&origin, fungible_asset.clone());

		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, None);
	});
}

// make sure that unknown fungibles (not on AssetRegistry) do not get trapped
#[test]
fn fungible_non_registered_trap_doesnt_work() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let fungible_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(statemine_asset_multi_location()),
			fun: Fungible((LocalAssetMinBalance::get() / 10) as u128),
		}
		.into();

		AssetTrap::drop_assets(&origin, fungible_asset.clone());

		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// nothing was written into asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, None);
	});
}

// make sure multiple assets are trapped separatedly
#[test]
fn multiple_assets_trap_claim_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		// make sure asset exists on AssetRegistry
		register_reserve_asset();

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let mut multi_assets = Vec::new();

		let native_asset = MultiAsset {
			id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
			fun: Fungible((CurrencyMinBalance::get() * 10) as u128),
		};
		multi_assets.push(native_asset.clone());

		let fungible_asset = MultiAsset {
			id: AssetId::Concrete(statemine_asset_multi_location()),
			fun: Fungible((LocalAssetMinBalance::get() * 10) as u128),
		};
		multi_assets.push(fungible_asset.clone());

		AssetTrap::drop_assets(&origin, multi_assets.clone().into());

		let expected_versioned_native =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash_native = BlakeTwo256::hash_of(&(&origin, &expected_versioned_native));

		// assert trap event for native asset
		System::assert_has_event(
			Event::AssetTrapped {
				0: expected_hash_native,
				1: origin.clone(),
				2: expected_versioned_native.clone(),
			}
			.into(),
		);

		// we can read the native trap storage
		let read_native_trap = AssetTrap::asset_trap(expected_hash_native);
		assert_eq!(
			read_native_trap,
			Some((MultiLocation { parents: 0, interior: Junctions::Here }, 1))
		);

		let expected_versioned_fungible =
			VersionedMultiAssets::from(MultiAssets::from(fungible_asset.clone()));
		let expected_hash_fungible = BlakeTwo256::hash_of(&(&origin, &expected_versioned_fungible));

		// assert trap event for fungible asset
		System::assert_has_event(
			Event::AssetTrapped {
				0: expected_hash_fungible,
				1: origin.clone(),
				2: expected_versioned_fungible.clone(),
			}
			.into(),
		);

		// we can read the fungible trap storage
		let read_fungible_trap = AssetTrap::asset_trap(expected_hash_fungible);
		assert_eq!(read_fungible_trap, Some((statemine_asset_multi_location(), 1)));

		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };

		// claim the native asset back
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into());

		System::assert_has_event(
			Event::AssetClaimed {
				0: expected_hash_native,
				1: origin.clone(),
				2: expected_versioned_native,
			}
			.into(),
		);

		let read_native_trap = AssetTrap::asset_trap(expected_hash_native);
		assert_eq!(read_native_trap, None);

		// claim the fungible asset back
		AssetTrap::claim_assets(&origin, &claim_ticket, &fungible_asset.into());

		System::assert_has_event(
			Event::AssetClaimed {
				0: expected_hash_fungible,
				1: origin.clone(),
				2: expected_versioned_fungible,
			}
			.into(),
		);

		let read_fungible_trap = AssetTrap::asset_trap(expected_hash_fungible);
		assert_eq!(read_fungible_trap, None);
	});
}

// assert that the trap counter works
#[test]
fn trap_counter_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
			fun: Fungible((CurrencyMinBalance::get() * 10) as u128),
		}
		.into();

		// trap assets 3x
		AssetTrap::drop_assets(&origin, native_asset.clone());
		AssetTrap::drop_assets(&origin, native_asset.clone());
		AssetTrap::drop_assets(&origin, native_asset.clone());

		let expected_versioned =
			VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
		let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(
			read_asset_trap,
			Some((MultiLocation { parents: 0, interior: Junctions::Here }, 3))
		);

		// claim the asset back (1)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into());

		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(
			read_asset_trap,
			Some((MultiLocation { parents: 0, interior: Junctions::Here }, 2))
		);

		// claim the asset back (2)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into());

		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(
			read_asset_trap,
			Some((MultiLocation { parents: 0, interior: Junctions::Here }, 1))
		);

		// claim the asset back (3)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into());

		let read_asset_trap = AssetTrap::asset_trap(expected_hash);
		assert_eq!(read_asset_trap, None);
	});
}
