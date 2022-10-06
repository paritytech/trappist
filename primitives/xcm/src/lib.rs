#![cfg_attr(not(feature = "std"), no_std)]

use sp_std::{borrow::Borrow, marker::PhantomData};
use xcm::latest::MultiLocation;

pub struct AsAssetMultiLocation<AssetId, AssetIdInfoGetter>(
	PhantomData<(AssetId, AssetIdInfoGetter)>,
);
impl<AssetId, AssetIdInfoGetter> xcm_executor::traits::Convert<MultiLocation, AssetId>
	for AsAssetMultiLocation<AssetId, AssetIdInfoGetter>
where
	AssetId: Clone,
	AssetIdInfoGetter: AssetMultiLocationGetter<AssetId>,
{
	fn convert_ref(asset_multi_location: impl Borrow<MultiLocation>) -> Result<AssetId, ()> {
		if let Some(asset_id) =
			AssetIdInfoGetter::get_asset_id(asset_multi_location.borrow().clone())
		{
			Ok(asset_id)
		} else {
			Err(())
		}
	}

	fn reverse_ref(asset_id: impl Borrow<AssetId>) -> Result<MultiLocation, ()> {
		if let Some(asset_multi_location) =
			AssetIdInfoGetter::get_asset_multi_location(asset_id.borrow().clone())
		{
			Ok(asset_multi_location)
		} else {
			Err(())
		}
	}
}

pub trait AssetMultiLocationGetter<AssetId> {
	fn get_asset_multi_location(asset_id: AssetId) -> Option<MultiLocation>;
	fn get_asset_id(asset_multi_location: MultiLocation) -> Option<AssetId>;
}
