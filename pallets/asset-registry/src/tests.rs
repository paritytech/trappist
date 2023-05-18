use frame_support::{assert_noop, assert_ok};
use xcm::latest::prelude::*;

use crate::{mock::*, AssetIdMultiLocation, AssetMultiLocationId, Error};

const STATEMINE_ASSET_MULTI_LOCATION: MultiLocation = MultiLocation {
	parents: 1,
	interior: X3(
		Parachain(StatemineParaIdInfo::get()),
		PalletInstance(StatemineAssetsInstanceInfo::get()),
		GeneralIndex(StatemineAssetIdInfo::get()),
	),
};

#[test]
fn register_reserve_asset_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRegistry::register_reserve_asset(
			RuntimeOrigin::root(),
			LOCAL_ASSET_ID,
			STATEMINE_ASSET_MULTI_LOCATION,
		));

		assert_eq!(
			AssetIdMultiLocation::<Test>::get(LOCAL_ASSET_ID),
			Some(STATEMINE_ASSET_MULTI_LOCATION)
		);
		assert_eq!(
			AssetMultiLocationId::<Test>::get(STATEMINE_ASSET_MULTI_LOCATION),
			Some(LOCAL_ASSET_ID)
		);
	});
}

#[test]
fn cannot_register_unexisting_asset() {
	new_test_ext().execute_with(|| {
		let unexisting_asset_id = 9999;

		assert_noop!(
			AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				unexisting_asset_id,
				STATEMINE_ASSET_MULTI_LOCATION,
			),
			Error::<Test>::AssetDoesNotExist
		);
	});
}

#[test]
fn cannot_double_register() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRegistry::register_reserve_asset(
			RuntimeOrigin::root(),
			LOCAL_ASSET_ID,
			STATEMINE_ASSET_MULTI_LOCATION,
		));

		assert_noop!(
			AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				STATEMINE_ASSET_MULTI_LOCATION,
			),
			Error::<Test>::AssetAlreadyRegistered
		);
	});
}

#[test]
fn unregister_reserve_asset_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(AssetRegistry::register_reserve_asset(
			RuntimeOrigin::root(),
			LOCAL_ASSET_ID,
			STATEMINE_ASSET_MULTI_LOCATION,
		));

		assert_ok!(AssetRegistry::unregister_reserve_asset(RuntimeOrigin::root(), LOCAL_ASSET_ID));

		assert!(AssetIdMultiLocation::<Test>::get(LOCAL_ASSET_ID).is_none());
		assert!(AssetMultiLocationId::<Test>::get(STATEMINE_ASSET_MULTI_LOCATION).is_none());
	});
}

#[test]
fn cannot_register_unregistered_asset() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			AssetRegistry::unregister_reserve_asset(RuntimeOrigin::root(), LOCAL_ASSET_ID),
			Error::<Test>::AssetIsNotRegistered
		);
	});
}