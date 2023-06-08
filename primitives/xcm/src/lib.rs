#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{
	sp_runtime::{SaturatedConversion, Saturating},
	traits::{fungibles::Inspect, Currency},
};
#[cfg(not(test))]
use sp_runtime::DispatchResult;
use sp_std::{borrow::Borrow, marker::PhantomData};
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
		AssetIdInfoGetter::get_asset_id(asset_multi_location.borrow()).ok_or(())
	}

	fn reverse_ref(asset_id: impl Borrow<AssetId>) -> Result<MultiLocation, ()> {
		AssetIdInfoGetter::get_asset_multi_location(asset_id.borrow().clone()).ok_or(())
	}
}

pub trait AssetMultiLocationGetter<AssetId> {
	fn get_asset_multi_location(asset_id: AssetId) -> Option<MultiLocation>;
	fn get_asset_id(asset_multi_location: &MultiLocation) -> Option<AssetId>;
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

pub trait DropAssetsWeigher {
	fn fungible() -> u64;
	fn native() -> u64;
	fn default() -> u64;
}

pub struct TrappistDropAssets<
	AssetId,
	AssetIdInfoGetter,
	AssetsPallet,
	BalancesPallet,
	XcmPallet,
	AccountId,
	Weigher,
>(
	PhantomData<(
		AssetId,
		AssetIdInfoGetter,
		AssetsPallet,
		BalancesPallet,
		XcmPallet,
		AccountId,
		Weigher,
	)>,
);

impl<AssetId, AssetIdInfoGetter, AssetsPallet, BalancesPallet, XcmPallet, AccountId, Weigher>
	DropAssets
	for TrappistDropAssets<
		AssetId,
		AssetIdInfoGetter,
		AssetsPallet,
		BalancesPallet,
		XcmPallet,
		AccountId,
		Weigher,
	> where
	AssetIdInfoGetter: AssetMultiLocationGetter<AssetId>,
	AssetsPallet: Inspect<AccountId, AssetId = AssetId>,
	BalancesPallet: Currency<AccountId>,
	XcmPallet: DropAssets,
	Weigher: DropAssetsWeigher,
{
	// assets are whatever the Holding Register had when XCVM halts
	fn drop_assets(origin: &MultiLocation, mut assets: Assets) -> u64 {
		const NATIVE_LOCATION: MultiLocation = MultiLocation { parents: 0, interior: Here };

		let mut weight = {
			assets.non_fungible.clear();
			Weigher::default()
		};

		assets.fungible.retain(|id, &mut amount| {
			if let Concrete(location) = id {
				match AssetIdInfoGetter::get_asset_id(location) {
					Some(asset_id) => {
						weight.saturating_accrue(Weigher::fungible());

						// only trap if amount ≥ min_balance
						// do nothing otherwise (asset is lost)
						amount.saturated_into::<AssetsPallet::Balance>() >=
							AssetsPallet::minimum_balance(asset_id)
					},
					None => {
						weight.saturating_accrue(Weigher::native());

						// only trap if native token and amount ≥ min_balance
						// do nothing otherwise (asset is lost)
						*location == NATIVE_LOCATION &&
							amount.saturated_into::<BalancesPallet::Balance>() >=
								BalancesPallet::minimum_balance()
					},
				}
			} else {
				weight.saturating_accrue(Weigher::default());
				false
			}
		});

		// we have filtered out non-compliant assets
		// insert valid assets into the asset trap implemented by XcmPallet
		weight.saturating_add(XcmPallet::drop_assets(origin, assets))
	}
}

/// Pause and resume execution of XCM
#[cfg(not(test))]
pub trait PauseXcmExecution {
	fn suspend_xcm_execution() -> DispatchResult;
	fn resume_xcm_execution() -> DispatchResult;
}
#[cfg(not(test))]
impl PauseXcmExecution for () {
	fn suspend_xcm_execution() -> DispatchResult {
		Ok(())
	}
	fn resume_xcm_execution() -> DispatchResult {
		Ok(())
	}
}
