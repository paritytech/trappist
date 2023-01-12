#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	sp_runtime::SaturatedConversion,
	traits::{fungibles::Inspect, Currency},
};
use sp_std::{borrow::Borrow, marker::PhantomData, vec::Vec};
use xcm::latest::{
	AssetId::Concrete, Fungibility::Fungible, Junctions::Here, MultiAsset, MultiLocation,
};
use xcm_executor::{
	traits::{Convert, DropAssets, Error as MatchError, MatchesFungibles},
	Assets,
};

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
		AssetIdInfoGetter::get_asset_id(asset_multi_location.borrow().clone()).ok_or(())
	}

	fn reverse_ref(asset_id: impl Borrow<AssetId>) -> Result<MultiLocation, ()> {
		AssetIdInfoGetter::get_asset_multi_location(asset_id.borrow().clone()).ok_or(())
	}
}

pub trait AssetMultiLocationGetter<AssetId> {
	fn get_asset_multi_location(asset_id: AssetId) -> Option<MultiLocation>;
	fn get_asset_id(asset_multi_location: MultiLocation) -> Option<AssetId>;
}

pub struct ConvertedRegisteredAssetId<AssetId, Balance, ConvertAssetId, ConvertBalance>(
	PhantomData<(AssetId, Balance, ConvertAssetId, ConvertBalance)>,
);
impl<
		AssetId: Clone,
		Balance: Clone,
		ConvertAssetId: Convert<MultiLocation, AssetId>,
		ConvertBalance: Convert<u128, Balance>,
	> MatchesFungibles<AssetId, Balance>
	for ConvertedRegisteredAssetId<AssetId, Balance, ConvertAssetId, ConvertBalance>
{
	fn matches_fungibles(a: &MultiAsset) -> Result<(AssetId, Balance), MatchError> {
		let (amount, id) = match (&a.fun, &a.id) {
			(Fungible(ref amount), Concrete(ref id)) => (amount, id),
			_ => return Err(MatchError::AssetNotFound),
		};
		let what = ConvertAssetId::convert_ref(id).map_err(|_| MatchError::AssetNotFound)?;
		let amount = ConvertBalance::convert_ref(amount)
			.map_err(|_| MatchError::AmountToBalanceConversionFailed)?;
		Ok((what, amount))
	}
}

pub struct TrappistDropAssets<
	AssetId,
	AssetIdInfoGetter,
	AssetsPallet,
	BalancesPallet,
	XcmPallet,
	AccoundId,
>(PhantomData<(AssetId, AssetIdInfoGetter, AssetsPallet, BalancesPallet, XcmPallet, AccoundId)>);
impl<AssetId, AssetIdInfoGetter, AssetsPallet, BalancesPallet, XcmPallet, AccountId> DropAssets
	for TrappistDropAssets<
		AssetId,
		AssetIdInfoGetter,
		AssetsPallet,
		BalancesPallet,
		XcmPallet,
		AccountId,
	> where
	AssetId: Clone,
	AssetIdInfoGetter: AssetMultiLocationGetter<AssetId>,
	AssetsPallet: Inspect<AccountId, AssetId = AssetId>,
	BalancesPallet: Currency<AccountId>,
	XcmPallet: DropAssets,
{
	fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
		let multi_assets: Vec<MultiAsset> = assets.into();
		let mut trap: Vec<MultiAsset> = Vec::new();

		for asset in multi_assets {
			if let MultiAsset { id: Concrete(location), fun: Fungible(amount) } = asset.clone() {
				// is location a fungible on AssetRegistry?
				if let Some(asset_id) = AssetIdInfoGetter::get_asset_id(location.clone()) {
					let min_balance = AssetsPallet::minimum_balance(asset_id);

					// only trap if amount ≥ min_balance
					// do nothing otherwise (asset is lost)
					if min_balance <= amount.saturated_into::<AssetsPallet::Balance>() {
						trap.push(asset);
					}

				// is location the native token?
				} else if location == (MultiLocation { parents: 0, interior: Here }) {
					let min_balance = BalancesPallet::minimum_balance();

					// only trap if amount ≥ min_balance
					// do nothing otherwise (asset is lost)
					if min_balance <= amount.saturated_into::<BalancesPallet::Balance>() {
						trap.push(asset);
					}
				}
			}
		}

		// TODO: put real weight of execution up until this point here
		let mut weight = 0;

		if !trap.is_empty() {
			weight += XcmPallet::drop_assets(origin, trap.into());
		}

		weight
	}
}
