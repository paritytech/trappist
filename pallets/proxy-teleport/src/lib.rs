#![cfg_attr(not(feature = "std"), no_std)]

type BaseXcm<T> = pallet_xcm::Pallet<T>;
pub use pallet::*;
use sp_std::boxed::Box;
pub use xcm::{
	opaque::latest::prelude::{Junction, Junctions, MultiLocation, OriginKind, Transact},
	v3::{
		AssetId, Fungibility,
		Instruction::{
			BuyExecution, DepositAsset, DepositReserveAsset, InitiateReserveWithdraw, WithdrawAsset,
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
	pub trait Config: frame_system::Config + pallet_xcm::Config {}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(0)]
		pub fn teleport_foreign_assets(
			origin: OriginFor<T>,
			dest: Box<VersionedMultiLocation>,
			beneficiary: Box<VersionedMultiLocation>,
			assets: Box<VersionedMultiAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			BaseXcm::<T>::teleport_assets(
			    origin,
			    dest,
			    beneficiary,
			    assets,
			    fee_asset_item,
			);
            Ok(())
		}
	}
}
