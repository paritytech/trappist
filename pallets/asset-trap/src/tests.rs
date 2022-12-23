use crate::{mock::*, Event, TrappedAssets};
use frame_support::{assert_ok, BoundedVec};
use sp_runtime::AccountId32;
use xcm::{latest::prelude::*, VersionedMultiAssets};
use xcm_executor::{
	traits::{ClaimAssets, DropAssets},
	Assets as XcmAssets,
};

const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
const BOB: AccountId32 = AccountId32::new([1u8; 32]);

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

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		// above min_balance
		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;

		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		let versioned_native_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset.clone());

		AssetTrap::drop_assets(&origin, native_asset.clone().into());

		System::assert_has_event(
			Event::AssetsTrapped { 0: origin.clone(), 1: versioned_native_asset.clone() }.into(),
		);

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into()));

		System::assert_has_event(
			Event::AssetsClaimed { 0: origin.clone(), 1: versioned_native_asset }.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(origin);
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

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		// below min_balance
		let native_asset_amount: u128 = (CurrencyMinBalance::get() / 10) as u128;

		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		}
		.into();

		AssetTrap::drop_assets(&origin, native_asset.clone());

		// nothing was written into asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(origin);
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

		let fungible_asset_multi_location: MultiLocation = statemine_asset_multi_location();

		// above min_balance
		let fungible_asset_amount: u128 = (LocalAssetMinBalance::get() * 10) as u128;

		let fungible_asset = MultiAsset {
			id: AssetId::Concrete(fungible_asset_multi_location.clone()),
			fun: Fungible(fungible_asset_amount),
		};

		let versioned_fungible_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(fungible_asset.clone());

		AssetTrap::drop_assets(&origin, fungible_asset.clone().into());

		System::assert_has_event(
			Event::AssetsTrapped { 0: origin.clone(), 1: versioned_fungible_asset.clone() }.into(),
		);

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_fungible_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &fungible_asset.into()));

		System::assert_has_event(
			Event::AssetsClaimed { 0: origin.clone(), 1: versioned_fungible_asset.clone() }.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(origin);
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

		let fungible_asset_multi_location: MultiLocation = statemine_asset_multi_location();

		// below min_balance
		let fungible_asset_amount: u128 = (LocalAssetMinBalance::get() / 10) as u128;

		let fungible_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(fungible_asset_multi_location.clone()),
			fun: Fungible(fungible_asset_amount),
		}
		.into();

		AssetTrap::drop_assets(&origin, fungible_asset.clone());

		// nothing was written into asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(origin);
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

		let fungible_asset_multi_location: MultiLocation = statemine_asset_multi_location();

		// above min_balance
		let fungible_asset_amount: u128 = (LocalAssetMinBalance::get() * 10) as u128;

		let fungible_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(fungible_asset_multi_location.clone()),
			fun: Fungible(fungible_asset_amount),
		}
		.into();

		AssetTrap::drop_assets(&origin, fungible_asset.clone());

		// nothing was written into asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(origin);
		assert_eq!(read_asset_trap, None);
	});
}

// assert that the trap counter works
#[test]
fn trap_counter_works() {
	new_test_ext().execute_with(|| {
		System::set_block_number(1);

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		// above min_balance
		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;

		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		let versioned_native_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset.clone());

		// trap assets 3x
		AssetTrap::drop_assets(&origin, native_asset.clone().into());
		AssetTrap::drop_assets(&origin, native_asset.clone().into());
		AssetTrap::drop_assets(&origin, native_asset.clone().into());

		// we can see the asset trap storage counter is 3
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 3 }]
				.try_into()
				.unwrap();

		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back (1)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into()));

		// we can see the asset trap storage counter is 2
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 2 }]
				.try_into()
				.unwrap();

		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back (2)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into()));

		// we can see the asset trap storage counter is 1
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();

		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back (3)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into()));

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(origin);
		assert_eq!(read_asset_trap, None);
	});
}

// assert that trapping different assets work
#[test]
fn different_assets_trap_claim_works() {
	new_test_ext().execute_with(|| {
		// make sure asset exists on AssetRegistry
		register_reserve_asset();

		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		// above min_balance
		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;

		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		let versioned_native_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset.clone());

		let fungible_asset_multi_location: MultiLocation = statemine_asset_multi_location();

		// above min_balance
		let fungible_asset_amount: u128 = (LocalAssetMinBalance::get() * 10) as u128;

		let fungible_asset = MultiAsset {
			id: AssetId::Concrete(fungible_asset_multi_location.clone()),
			fun: Fungible(fungible_asset_amount),
		};

		let versioned_fungible_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(fungible_asset.clone());

		AssetTrap::drop_assets(&origin, native_asset.clone().into());
		AssetTrap::drop_assets(&origin, fungible_asset.clone().into());

		// we can see the assets are trapped
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> = vec![
			TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 },
			TrappedAssets { multi_assets: versioned_fungible_asset.clone(), n: 1 },
		]
		.try_into()
		.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the native asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into()));

		// we can see fungible asset is remaining in trap
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_fungible_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the fungible asset back
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &fungible_asset.clone().into()));

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(origin);
		assert_eq!(read_asset_trap, None);
	});
}

// assert that trapping different amounts work
#[test]
fn different_amounts_trap_claim_works() {
	new_test_ext().execute_with(|| {
		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		let native_asset_amount_a: u128 = (CurrencyMinBalance::get() * 10) as u128;
		let native_asset_a = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount_a),
		};

		let native_asset_amount_b: u128 = (CurrencyMinBalance::get() * 20) as u128;
		let native_asset_b = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount_b),
		};

		let versioned_native_asset_a: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset_a.clone());
		let versioned_native_asset_b: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset_b.clone());

		AssetTrap::drop_assets(&origin, native_asset_a.clone().into());
		AssetTrap::drop_assets(&origin, native_asset_b.clone().into());

		// we can see the assets are trapped
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> = vec![
			TrappedAssets { multi_assets: versioned_native_asset_a.clone(), n: 1 },
			TrappedAssets { multi_assets: versioned_native_asset_b.clone(), n: 1 },
		]
		.try_into()
		.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back (a)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset_a.clone().into()));

		// we can see there's only one asset (b) remaining in trap
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset_b.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));

		// claim the asset back (b)
		assert!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset_b.clone().into()));

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(origin);
		assert_eq!(read_asset_trap, None);
	});
}

// assert that trapping from different origins work
#[test]
fn different_origins_trap_claim_works() {
	new_test_ext().execute_with(|| {
		let origin_a: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let origin_b: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: BOB.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;
		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		let versioned_native_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset.clone());

		AssetTrap::drop_assets(&origin_a, native_asset.clone().into());
		AssetTrap::drop_assets(&origin_b, native_asset.clone().into());

		// we can see the assets are trapped separatedly
		let read_asset_trap_a = AssetTrap::asset_trap(origin_a.clone());
		let expected_bounded_vec_a: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap_a, Some(expected_bounded_vec_a));
		let read_asset_trap_b = AssetTrap::asset_trap(origin_b.clone());
		let expected_bounded_vec_b: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap_b, Some(expected_bounded_vec_b));

		// claim the asset back (a)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		assert!(AssetTrap::claim_assets(&origin_a, &claim_ticket, &native_asset.clone().into()));

		// we can see only asset (b) remainins in trap
		let read_asset_trap_a = AssetTrap::asset_trap(origin_a);
		assert_eq!(read_asset_trap_a, None);
		let read_asset_trap_b = AssetTrap::asset_trap(origin_b.clone());
		let expected_bounded_vec_b: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap_b, Some(expected_bounded_vec_b));

		// claim the asset back (b)
		assert!(AssetTrap::claim_assets(&origin_b, &claim_ticket, &native_asset.clone().into()));

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(origin_b);
		assert_eq!(read_asset_trap, None);
	});
}

// assert MaxTrapsPerOrigin
#[test]
fn max_traps_per_origin_works() {
	new_test_ext().execute_with(|| {
		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		let mut versioned_native_assets = Vec::new();

		let bound = MaxTrapsPerOrigin::get();
		for i in 0..bound {
			let native_asset_amount: u128 = (CurrencyMinBalance::get() * ((i + 1) as u64)) as u128;
			let native_asset = MultiAsset {
				id: AssetId::Concrete(native_asset_multi_location.clone()),
				fun: Fungible(native_asset_amount),
			};
			let versioned_native_asset: VersionedMultiAssets =
				VersionedMultiAssets::from(native_asset.clone());
			versioned_native_assets.push(versioned_native_asset);

			AssetTrap::drop_assets(&origin, native_asset.clone().into());
		}

		// assert full BoundedVec after loop
		let read_asset_trap = AssetTrap::asset_trap(origin.clone()).unwrap();
		assert_eq!(read_asset_trap.len() as u32, bound);
		assert_eq!(
			read_asset_trap[0],
			TrappedAssets { multi_assets: versioned_native_assets[0].clone(), n: 1 }
		);
		assert_eq!(
			read_asset_trap[(bound - 1) as usize],
			TrappedAssets {
				multi_assets: versioned_native_assets[(bound - 1) as usize].clone(),
				n: 1
			}
		);

		// trap again
		let extra_native_asset_amount: u128 = (CurrencyMinBalance::get() * 69) as u128;
		let extra_native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(extra_native_asset_amount),
		};
		let extra_versioned_native_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(extra_native_asset.clone());

		AssetTrap::drop_assets(&origin, extra_native_asset.clone().into());
		let read_asset_trap = AssetTrap::asset_trap(origin).unwrap();

		// assert BoundedVec after trapping again
		assert_eq!(read_asset_trap.len() as u32, bound);
		assert_eq!(
			read_asset_trap[0],
			TrappedAssets { multi_assets: versioned_native_assets[1].clone(), n: 1 }
		);
		assert_eq!(
			read_asset_trap[(bound - 1) as usize],
			TrappedAssets { multi_assets: extra_versioned_native_asset, n: 1 }
		);
	});
}

// assert bad claim fails
#[test]
fn bad_claim_fails() {
	new_test_ext().execute_with(|| {
		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;

		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		// claim the asset back, but it has never been trapped!
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };

		// we expect claim_assets to return false
		assert_eq!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into()), false);
	});
}

// assert bad claim ticket
#[test]
fn bad_claim_ticket_fails() {
	new_test_ext().execute_with(|| {
		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		// above min_balance
		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;

		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		let versioned_native_asset: VersionedMultiAssets =
			VersionedMultiAssets::from(native_asset.clone());

		AssetTrap::drop_assets(&origin, native_asset.clone().into());

		// claim the asset with a bad ticket
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::X1(GeneralIndex(3)) };

		// we expect claim_assets to return false
		assert_eq!(AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into()), false);

		// we can still read the asset trap storage, since the claim failed
		let read_asset_trap = AssetTrap::asset_trap(origin.clone());
		let expected_bounded_vec: BoundedVec<TrappedAssets, MaxTrapsPerOrigin> =
			vec![TrappedAssets { multi_assets: versioned_native_asset.clone(), n: 1 }]
				.try_into()
				.unwrap();
		assert_eq!(read_asset_trap, Some(expected_bounded_vec));
	});
}