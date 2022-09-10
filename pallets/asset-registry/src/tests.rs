use crate::tests::{StatemineAssetsInstanceInfo, StatemineParaIdInfo};
use crate::{mock::*, Error, ForeignAssetMetadata};
use frame_support::{assert_noop, assert_ok};
use xcm::latest::prelude::*;

#[test]
fn registering_foreign_works() {
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

		let statemine_asset_metadata = ForeignAssetMetadata {
			name: "xUSD".as_bytes().to_vec(),
			symbol: "xUSD".as_bytes().to_vec(),
			decimals: 12,
			is_frozen: false,
		};

		let local_asset_id = 1;
		let local_asset_min_amount = 1_000_000_000;

		assert_ok!(AssetRegistry::register_foreign_asset(
			Origin::root(),
			local_asset_id.clone(),
			statemine_asset_multi_location.clone(),
			statemine_asset_metadata.clone(),
			local_asset_min_amount.clone(),
			true,
		));

		assert_noop!(
			AssetRegistry::register_foreign_asset(
				Origin::root(),
				1,
				statemine_asset_multi_location,
				statemine_asset_metadata,
				1_000_000_000,
				true,
			),
			Error::<Test>::AssetAlreadyExists
		);
	});
}
