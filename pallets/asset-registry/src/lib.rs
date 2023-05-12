#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{pallet_prelude::*, traits::tokens::fungibles::Inspect};
	use frame_system::pallet_prelude::*;

	use xcm::latest::{
		Junction::{GeneralIndex, PalletInstance, Parachain},
		Junctions, MultiLocation,
	};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	type AssetIdOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;

	#[cfg(feature = "runtime-benchmarks")]
	pub trait BenchmarkHelper<AssetId> {
		fn get_registered_asset() -> AssetId;
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type ReserveAssetModifierOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type Assets: Inspect<Self::AccountId>;
		type WeightInfo: WeightInfo;
		/// Helper trait for benchmarks.
		#[cfg(feature = "runtime-benchmarks")]
		type BenchmarkHelper: BenchmarkHelper<AssetIdOf<Self>>;
	}

	#[pallet::storage]
	pub type AssetIdMultiLocation<T: Config> =
		StorageMap<_, Blake2_128Concat, AssetIdOf<T>, MultiLocation>;

	#[pallet::storage]
	pub type AssetMultiLocationId<T: Config> =
		StorageMap<_, Blake2_128Concat, MultiLocation, AssetIdOf<T>>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ReserveAssetRegistered { asset_id: AssetIdOf<T>, asset_multi_location: MultiLocation },
		ReserveAssetUnregistered { asset_id: AssetIdOf<T>, asset_multi_location: MultiLocation },
	}

	#[pallet::error]
	pub enum Error<T> {
		/// The Asset ID is already registered
		AssetAlreadyRegistered,
		/// The Asset ID does not exist
		AssetDoesNotExist,
		/// The Asset ID is not registered
		AssetIsNotRegistered,
		/// Invalid MultiLocation
		WrongMultiLocation,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::register_reserve_asset())]
		pub fn register_reserve_asset(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
			asset_multi_location: MultiLocation,
		) -> DispatchResult {
			T::ReserveAssetModifierOrigin::ensure_origin(origin)?;

			ensure!(T::Assets::asset_exists(asset_id), Error::<T>::AssetDoesNotExist);

			// verify asset is not yet registered
			ensure!(
				!AssetIdMultiLocation::<T>::contains_key(asset_id),
				Error::<T>::AssetAlreadyRegistered
			);

			// verify MultiLocation is valid
			ensure!(
				matches!(
					asset_multi_location,
					MultiLocation {
						parents: 1,
						interior: Junctions::X3(Parachain(_), PalletInstance(_), GeneralIndex(_))
					}
				),
				Error::<T>::WrongMultiLocation
			);

			// register asset
			AssetIdMultiLocation::<T>::insert(asset_id, &asset_multi_location);
			AssetMultiLocationId::<T>::insert(&asset_multi_location, asset_id);

			Self::deposit_event(Event::ReserveAssetRegistered { asset_id, asset_multi_location });
			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::unregister_reserve_asset())]
		pub fn unregister_reserve_asset(
			origin: OriginFor<T>,
			asset_id: AssetIdOf<T>,
		) -> DispatchResult {
			T::ReserveAssetModifierOrigin::ensure_origin(origin)?;

			// unregister asset
			let asset_multi_location =
				AssetIdMultiLocation::<T>::mutate_exists(asset_id, Option::take)
					.ok_or(Error::<T>::AssetIsNotRegistered)?;
			AssetMultiLocationId::<T>::remove(&asset_multi_location);

			Self::deposit_event(Event::ReserveAssetUnregistered { asset_id, asset_multi_location });
			Ok(())
		}
	}

	impl<T: Config> xcm_primitives::AssetMultiLocationGetter<AssetIdOf<T>> for Pallet<T> {
		fn get_asset_multi_location(asset_id: AssetIdOf<T>) -> Option<MultiLocation> {
			AssetIdMultiLocation::<T>::get(asset_id)
		}

		fn get_asset_id(asset_type: MultiLocation) -> Option<AssetIdOf<T>> {
			AssetMultiLocationId::<T>::get(asset_type)
		}
	}
}
