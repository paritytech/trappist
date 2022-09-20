#![cfg(test)]

use super::*;
use crate::tests::parachain::Origin as ParaOrigin;
use crate::tests::parachain::Runtime as ParaRuntime;
use frame_support::{assert_noop, assert_ok};
use mock::*;
use xcm::latest::prelude::*;
use xcm_simulator::TestExt;

#[test]
fn reserve_transfer_works() {
	MockNet::reset();

	let withdraw_amount = 123;

	Statemine::execute_with(|| {
		assert_eq!(statemine::Assets::balance(STATEMINE_ASSET_ID, ALICE), INITIAL_BALANCE);

		// todo:
		// first we just need the mock environment to work with a simple naive reserve transfer
		// where STATEMINE_ASSET_ID == PARA_ASSET_ID
		// after this works, we change one of the constants so it stops working, and then call 
		// Para::execute_with(|| { parachain::AssetRegistry::register_reserve_asset(...) });
		// before doing the reserve_transfer_assets below

		// for some reason, this is not moving funds to the parachain's sovereign account
		// that means there's something wrong in the XCM configs of the mock environment
		assert_ok!(StateminePalletXcm::reserve_transfer_assets(
			statemine::Origin::signed(ALICE),
			Box::new(MultiLocation { parents: 1, interior: X1(Parachain(PARA_ID)) }.into()),
			Box::new(
				MultiLocation {
					parents: 0,
					interior: X1(AccountId32 { network: Any, id: ALICE.into() })
				}
				.into()
			),
			Box::new(
				MultiAsset {
					id: AssetId::Concrete(MultiLocation {
						parents: 0,
						interior: X2(
							PalletInstance(STATEMINE_ASSETS_PALLET_INSTANCE),
							GeneralIndex(STATEMINE_ASSET_ID.into())
						)
					}),
					fun: Fungible(withdraw_amount)
				}
				.into()
			),
			0,
		));

		// did the funds move away from Alice's account?
		assert_eq!(
			statemine::Assets::balance(STATEMINE_ASSET_ID, ALICE),
			INITIAL_BALANCE - withdraw_amount
		);

		// did the funds move into the Parachain's sovereign account?
		assert_eq!(
			statemine::Assets::balance(STATEMINE_ASSET_ID, &para_account_id(PARA_ID)),
			withdraw_amount
		);
	});

	// once we figure out the issues on the mock environement, we can enable this assertion
	// Para::execute_with(|| {
	// 	// free execution, full amount received
	// 	assert_eq!(
	// 		parachain::Assets::balance(PARA_ASSET_ID, ALICE),
	// 		INITIAL_BALANCE + withdraw_amount
	// 	);
	// });
}

#[test]
fn register_reserve_asset_works() {
	MockNet::reset();

	Para::execute_with(|| {
		let statemine_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(STATEMINE_PARA_ID),
				PalletInstance(STATEMINE_ASSETS_PALLET_INSTANCE),
				GeneralIndex(STATEMINE_ASSET_ID.into()),
			),
		};

		assert_ok!(parachain::AssetRegistry::register_reserve_asset(
			ParaOrigin::root(),
			PARA_ASSET_ID,
			statemine_asset_multi_location.clone(),
		));

		if let Some(read_asset_multi_location) =
			parachain::AssetRegistry::asset_id_multilocation(PARA_ASSET_ID)
		{
			assert_eq!(read_asset_multi_location, statemine_asset_multi_location);
		} else {
			panic!("error reading AssetIdMultiLocation");
		}

		if let Some(read_asset_id) =
			parachain::AssetRegistry::asset_multilocation_id(statemine_asset_multi_location.clone())
		{
			assert_eq!(read_asset_id, PARA_ASSET_ID);
		} else {
			panic!("error reading AssetMultiLocationId");
		}

		assert_noop!(
			parachain::AssetRegistry::register_reserve_asset(
				ParaOrigin::root(),
				PARA_ASSET_ID,
				statemine_asset_multi_location.clone(),
			),
			Error::<ParaRuntime>::AssetAlreadyRegistered
		);
	});
}

#[test]
fn unregister_reserve_asset_works() {
	MockNet::reset();

	Para::execute_with(|| {
		let statemine_asset_multi_location = MultiLocation {
			parents: 1,
			interior: X3(
				Parachain(STATEMINE_PARA_ID),
				PalletInstance(STATEMINE_ASSETS_PALLET_INSTANCE),
				GeneralIndex(STATEMINE_ASSET_ID.into()),
			),
		};

		assert_ok!(parachain::AssetRegistry::register_reserve_asset(
			ParaOrigin::root(),
			PARA_ASSET_ID,
			statemine_asset_multi_location.clone(),
		));

		assert_ok!(parachain::AssetRegistry::unregister_reserve_asset(
			ParaOrigin::root(),
			PARA_ASSET_ID
		));

		assert!(parachain::AssetRegistry::asset_id_multilocation(PARA_ASSET_ID).is_none());
		assert!(parachain::AssetRegistry::asset_multilocation_id(
			statemine_asset_multi_location.clone()
		)
		.is_none());
	});
}
