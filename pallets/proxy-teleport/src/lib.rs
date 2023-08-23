#![cfg_attr(not(feature = "std"), no_std)]

type BaseXcm<T> = pallet_xcm::Pallet<T>;
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{Contains, EnsureOrigin, Get},
	weights::Weight,
};
use frame_system::pallet_prelude::OriginFor;
pub use pallet::*;
use parity_scale_codec::Encode;
use sp_std::{boxed::Box, vec};
use xcm::v3::WeightLimit::Unlimited;
pub use xcm::{
	opaque::latest::prelude::{Junction, Junctions, MultiLocation, OriginKind},
	v3::{
		AssetId, ExecuteXcm, Fungibility,
		Instruction::{
			BuyExecution, DepositAsset, DepositReserveAsset, InitiateReserveWithdraw, Transact,
			WithdrawAsset,
		},
		MultiAsset, MultiAssetFilter, MultiAssets, Parent, WeightLimit, WildMultiAsset, Xcm,
	},
	VersionedMultiAssets, VersionedMultiLocation, VersionedResponse, VersionedXcm,
};

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
// pub mod weights;
// pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_xcm::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::error]
	pub enum Error<T> {
		/// The version of the `Versioned` value used is not able to be interpreted.
		BadVersion,
		/// Too many assets have been attempted for transfer.
		TooManyAssets,
		/// The message execution fails the filter.
		Filtered,
		/// The assets to be sent are empty.
		Empty,
		/// Could not re-anchor the assets to declare the fees for the destination chain.
		CannotReanchor,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Execution of an XCM message was attempted.
		Attempted { outcome: xcm::latest::Outcome },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn proxy_teleport(
			origin: OriginFor<T>,
			dest: Box<VersionedMultiLocation>,
			beneficiary: Box<VersionedMultiLocation>,
			assets: Box<VersionedMultiAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			Self::do_proxy_teleport_assets(origin, dest, beneficiary, assets, fee_asset_item, None)
		}
	}
}

/// The maximum number of distinct assets allowed to be transferred in a single helper extrinsic.
const MAX_ASSETS_FOR_TRANSFER: usize = 2;

impl<T: Config> Pallet<T> {
	fn do_proxy_teleport_assets(
		origin: OriginFor<T>,
		dest: Box<VersionedMultiLocation>,
		beneficiary: Box<VersionedMultiLocation>,
		assets: Box<VersionedMultiAssets>,
		fee_asset_item: u32,
		maybe_weight_limit: Option<WeightLimit>,
	) -> DispatchResult {
		let origin_location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		let dest: MultiLocation = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let beneficiary: MultiLocation =
			(*beneficiary).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let assets: MultiAssets = (*assets).try_into().map_err(|()| Error::<T>::BadVersion)?;

		ensure!(assets.len() <= MAX_ASSETS_FOR_TRANSFER, Error::<T>::TooManyAssets);
		let value = (origin_location, assets.into_inner());
		ensure!(T::XcmTeleportFilter::contains(&value), Error::<T>::Filtered);
		let (origin_location, assets) = value;
		let context = T::UniversalLocation::get();
		let fees = assets
			.get(fee_asset_item as usize)
			.ok_or(Error::<T>::Empty)?
			.clone()
			.reanchored(&dest, context)
			.map_err(|_| Error::<T>::CannotReanchor)?;
		let max_assets = assets.len() as u32;
		let assets: MultiAssets = assets.into();
		let weight_limit: WeightLimit = Unlimited;
		// let weight_limit = match maybe_weight_limit {
		// 	Some(weight_limit) => weight_limit,
		// 	None => {
		// 		let fees = fees.clone();
		// 		let mut remote_message = Xcm(vec![
		// 			ReceiveTeleportedAsset(assets.clone()),
		// 			ClearOrigin,
		// 			BuyExecution { fees, weight_limit: Limited(Weight::zero()) },
		// 			DepositAsset { assets: Wild(AllCounted(max_assets)), beneficiary },
		// 		]);
		// 		// use local weight for remote message and hope for the best.
		// 		let remote_weight = T::Weigher::weight(&mut remote_message)
		// 			.map_err(|()| Error::<T>::UnweighableMessage)?;
		// 		Limited(remote_weight)
		// 	},
		// };
		let xcm: Xcm<()> = Xcm(vec![
			BuyExecution { fees, weight_limit },
			DepositAsset {
				assets: MultiAssetFilter::Wild(WildMultiAsset::AllCounted(max_assets)),
				beneficiary,
			},
		]);
		let mut message: Xcm<<T as frame_system::Config>::RuntimeCall> = Xcm(vec![
			WithdrawAsset(assets),
			// SetFeesMode { jit_withdraw: true },
            //TODO: Send 
		]);
		let weight: Weight = Weight::from_parts(1_000_000_000, 100_000);
		// 	T::Weigher::weight(&mut message).map_err(|()| Error::<T>::UnweighableMessage)?;
		let hash = message.using_encoded(sp_io::hashing::blake2_256);
		let outcome =
			T::XcmExecutor::execute_xcm_in_credit(origin_location, message, hash, weight, weight);
		Self::deposit_event(Event::Attempted { outcome });
		Ok(())
	}
}
