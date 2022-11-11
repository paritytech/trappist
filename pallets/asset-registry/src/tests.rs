use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};
use xcm::latest::prelude::*;

#[test]
fn register_reserve_asset_works() {
	new_test_ext().execute_with(|| {
		let statemine_para_id = StatemineParaIdInfo::get();
		let statemine_assets_pallet = StatemineAssetsInstanceInfo::get();
		let statemine_asset_id = StatemineAssetIdInfo::get();

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
			LOCAL_ASSET_ID,
			statemine_asset_multi_location.clone(),
		));

		let read_asset_multi_location = AssetRegistry::asset_id_multilocation(LOCAL_ASSET_ID)
			.expect("error reading AssetIdMultiLocation");
		assert_eq!(read_asset_multi_location, statemine_asset_multi_location);

		let read_asset_id = AssetRegistry::asset_multilocation_id(&statemine_asset_multi_location)
			.expect("error reading AssetMultiLocationId");
		assert_eq!(read_asset_id, LOCAL_ASSET_ID);

		assert_noop!(
			AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				statemine_asset_multi_location,
			),
			Error::<Test>::AssetAlreadyRegistered
		);
	});
}

#[test]
fn unregister_reserve_asset_works() {
	new_test_ext().execute_with(|| {
		let statemine_para_id = StatemineParaIdInfo::get();
		let statemine_assets_pallet = StatemineAssetsInstanceInfo::get();
		let statemine_asset_id = StatemineAssetIdInfo::get();

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
			LOCAL_ASSET_ID,
			statemine_asset_multi_location.clone(),
		));

		assert_ok!(AssetRegistry::unregister_reserve_asset(RuntimeOrigin::root(), LOCAL_ASSET_ID));

		assert!(AssetRegistry::asset_id_multilocation(LOCAL_ASSET_ID).is_none());
		assert!(
			AssetRegistry::asset_multilocation_id(statemine_asset_multi_location.clone()).is_none()
		);
	});
}
