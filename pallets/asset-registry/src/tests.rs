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

mod register_reserve_assest {
	use super::*;

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
	fn valid_locations_succced() {
		let native_frame_based_currency =
			MultiLocation { parents: 1, interior: X2(Parachain(1000), PalletInstance(1)) };
		let multiasset_pallet_instance = MultiLocation {
			parents: 1,
			interior: X3(Parachain(1000), PalletInstance(1), GeneralIndex(2)),
		};
		let relay_native_currency = MultiLocation { parents: 1, interior: Junctions::Here };
		let erc20_frame_sm_asset = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(1000),
				PalletInstance(2),
				AccountId32 { network: Some(Rococo), id: [0; 32] },
			),
		};
		let erc20_ethereum_sm_asset = MultiLocation {
			parents: 1,
			interior: X2(
				Parachain(2000),
				AccountKey20 { network: Some(Ethereum { chain_id: 56 }), key: [0; 20] },
			),
		};

		new_test_ext().execute_with(|| {
			assert_ok!(AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				native_frame_based_currency,
			));
		});
		new_test_ext().execute_with(|| {
			assert_ok!(AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				multiasset_pallet_instance,
			));
		});
		new_test_ext().execute_with(|| {
			assert_ok!(AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				relay_native_currency,
			));
		});
		new_test_ext().execute_with(|| {
			assert_ok!(AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				erc20_frame_sm_asset,
			));
		});
		new_test_ext().execute_with(|| {
			assert_ok!(AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				erc20_ethereum_sm_asset,
			));
		});
	}

	#[test]
	fn invalid_locations_fail() {
		let governance_location = MultiLocation {
			parents: 1,
			interior: X2(
				Parachain(1000),
				Plurality { id: BodyId::Executive, part: BodyPart::Voice },
			),
		};
		let invalid_general_index =
			MultiLocation { parents: 1, interior: X2(Parachain(1000), GeneralIndex(1u128)) };

		new_test_ext().execute_with(|| {
			assert_noop!(
				AssetRegistry::register_reserve_asset(
					RuntimeOrigin::root(),
					LOCAL_ASSET_ID,
					governance_location,
				),
				Error::<Test>::WrongMultiLocation
			);

			assert_noop!(
				AssetRegistry::register_reserve_asset(
					RuntimeOrigin::root(),
					LOCAL_ASSET_ID,
					invalid_general_index,
				),
				Error::<Test>::WrongMultiLocation
			);
		})
	}
}

mod unregister_reserve_asset {
	use super::*;

	#[test]
	fn unregister_reserve_asset_works() {
		new_test_ext().execute_with(|| {
			assert_ok!(AssetRegistry::register_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID,
				STATEMINE_ASSET_MULTI_LOCATION,
			));

			assert_ok!(AssetRegistry::unregister_reserve_asset(
				RuntimeOrigin::root(),
				LOCAL_ASSET_ID
			));

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
}
