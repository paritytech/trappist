use crate::{mock::*, Event, MultiAssetVersion, TrappedAsset};
use frame_support::assert_ok;
use sp_runtime::AccountId32;
use xcm::latest::prelude::*;
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

		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		}
		.into();

		AssetTrap::drop_assets(&origin, native_asset.clone());

		System::assert_has_event(
			Event::AssetTrapped {
				0: native_asset_multi_location.clone(),
				1: origin.clone(),
				2: native_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: native_asset_amount,
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		// claim the asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into());

		System::assert_has_event(
			Event::AssetClaimed {
				0: native_asset_multi_location.clone(),
				1: origin.clone(),
				2: native_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location);
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
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location);
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

		let fungible_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(fungible_asset_multi_location.clone()),
			fun: Fungible(fungible_asset_amount),
		}
		.into();

		AssetTrap::drop_assets(&origin, fungible_asset.clone());

		System::assert_has_event(
			Event::AssetTrapped {
				0: fungible_asset_multi_location.clone(),
				1: origin.clone(),
				2: fungible_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		// we can read the asset trap storage
		let read_asset_trap = AssetTrap::asset_trap(fungible_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: fungible_asset_amount.clone(),
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		// claim the asset back
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &fungible_asset.into());

		System::assert_has_event(
			Event::AssetClaimed {
				0: fungible_asset_multi_location.clone(),
				1: origin.clone(),
				2: fungible_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(fungible_asset_multi_location);
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
		let read_asset_trap = AssetTrap::asset_trap(fungible_asset_multi_location);
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
		let read_asset_trap = AssetTrap::asset_trap(fungible_asset_multi_location);
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

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		// above min_balance
		let native_asset_amount: u128 = (CurrencyMinBalance::get() * 10) as u128;

		let native_asset = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		};

		multi_assets.push(native_asset.clone());

		let fungible_asset_multi_location: MultiLocation = statemine_asset_multi_location();

		// above min_balance
		let fungible_asset_amount: u128 = (LocalAssetMinBalance::get() * 10) as u128;

		let fungible_asset = MultiAsset {
			id: AssetId::Concrete(fungible_asset_multi_location.clone()),
			fun: Fungible(fungible_asset_amount),
		};

		multi_assets.push(fungible_asset.clone());

		AssetTrap::drop_assets(&origin, multi_assets.clone().into());

		// assert trap event for native asset
		System::assert_has_event(
			Event::AssetTrapped {
				0: native_asset_multi_location.clone(),
				1: origin.clone(),
				2: native_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		// we can read the native trap storage
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: native_asset_amount,
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		// assert trap event for fungible asset
		System::assert_has_event(
			Event::AssetTrapped {
				0: fungible_asset_multi_location.clone(),
				1: origin.clone(),
				2: fungible_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		// we can read the fungible trap storage
		let read_asset_trap = AssetTrap::asset_trap(fungible_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: fungible_asset_amount.clone(),
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };

		// claim the native asset back
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into());

		System::assert_has_event(
			Event::AssetClaimed {
				0: native_asset_multi_location.clone(),
				1: origin.clone(),
				2: native_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location);
		assert_eq!(read_asset_trap, None);

		// claim the fungible asset back
		AssetTrap::claim_assets(&origin, &claim_ticket, &fungible_asset.into());

		System::assert_has_event(
			Event::AssetClaimed {
				0: fungible_asset_multi_location.clone(),
				1: origin.clone(),
				2: fungible_asset_amount,
				3: MultiAssetVersion::V1,
			}
			.into(),
		);

		let read_asset_trap = AssetTrap::asset_trap(fungible_asset_multi_location);
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

		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		}
		.into();

		// trap assets 3x
		AssetTrap::drop_assets(&origin, native_asset.clone());
		AssetTrap::drop_assets(&origin, native_asset.clone());
		AssetTrap::drop_assets(&origin, native_asset.clone());

		// we can see the asset trap storage counter is 3
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: native_asset_amount,
				multi_asset_version: MultiAssetVersion::V1,
				n: 3
			}])
		);

		// claim the asset back (1)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into());

		// we can see the asset trap storage counter is 2
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: native_asset_amount,
				multi_asset_version: MultiAssetVersion::V1,
				n: 2
			}])
		);

		// claim the asset back (2)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.clone().into());

		// we can see the asset trap storage counter is 1
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: native_asset_amount,
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		// claim the asset back (3)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset.into());

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location);
		assert_eq!(read_asset_trap, None);
	});
}

// assert that traping different amounts work
#[test]
fn different_amounts_trap_claim_works() {
	new_test_ext().execute_with(|| {
		let origin: MultiLocation =
			Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

		let native_asset_multi_location: MultiLocation =
			MultiLocation { parents: 0, interior: Junctions::Here };

		let native_asset_amount_a: u128 = (CurrencyMinBalance::get() * 10) as u128;
		let native_asset_a: XcmAssets = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount_a),
		}
		.into();

		let native_asset_amount_b: u128 = (CurrencyMinBalance::get() * 20) as u128;
		let native_asset_b: XcmAssets = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount_b),
		}
		.into();

		AssetTrap::drop_assets(&origin, native_asset_a.clone());
		AssetTrap::drop_assets(&origin, native_asset_b.clone());

		// we can see the assets are trapped separatedly
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![
				TrappedAsset {
					origin: origin.clone(),
					amount: native_asset_amount_a,
					multi_asset_version: MultiAssetVersion::V1,
					n: 1
				},
				TrappedAsset {
					origin: origin.clone(),
					amount: native_asset_amount_b,
					multi_asset_version: MultiAssetVersion::V1,
					n: 1
				}
			])
		);

		// claim the asset back (a)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset_a.clone().into());

		// we can see there's only one asset (b) remaining in trap
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin.clone(),
				amount: native_asset_amount_b,
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		// claim the asset back (b)
		AssetTrap::claim_assets(&origin, &claim_ticket, &native_asset_b.clone().into());

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location);
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
		let native_asset: XcmAssets = MultiAsset {
			id: AssetId::Concrete(native_asset_multi_location.clone()),
			fun: Fungible(native_asset_amount),
		}
		.into();

		AssetTrap::drop_assets(&origin_a, native_asset.clone());
		AssetTrap::drop_assets(&origin_b, native_asset.clone());

		// we can see the assets are trapped separatedly
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![
				TrappedAsset {
					origin: origin_a.clone(),
					amount: native_asset_amount,
					multi_asset_version: MultiAssetVersion::V1,
					n: 1
				},
				TrappedAsset {
					origin: origin_b.clone(),
					amount: native_asset_amount,
					multi_asset_version: MultiAssetVersion::V1,
					n: 1
				}
			])
		);

		// claim the asset back (a)
		let claim_ticket = MultiLocation { parents: 0, interior: Junctions::Here };
		AssetTrap::claim_assets(&origin_a, &claim_ticket, &native_asset.clone().into());

		// we can see there's only one asset (b) remaining in trap
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location.clone());
		assert_eq!(
			read_asset_trap,
			Some(vec![TrappedAsset {
				origin: origin_b.clone(),
				amount: native_asset_amount,
				multi_asset_version: MultiAssetVersion::V1,
				n: 1
			}])
		);

		// claim the asset back (b)
		AssetTrap::claim_assets(&origin_b, &claim_ticket, &native_asset.clone().into());

		// we can see the asset trap storage is empty
		let read_asset_trap = AssetTrap::asset_trap(native_asset_multi_location);
		assert_eq!(read_asset_trap, None);
	});
}
