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

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{inherent::Vec, pallet_prelude::*, PalletId};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::AccountIdConversion;
	use xcm::latest::MultiLocation;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	pub const PALLET_ID: PalletId = PalletId(*b"asstrgty");

	#[derive(Clone, Debug, Eq, PartialEq, Encode, Decode, TypeInfo)]
	pub struct ForeignAssetMetadata {
		pub name: Vec<u8>,
		pub symbol: Vec<u8>,
		pub decimals: u8,
		pub is_frozen: bool,
	}

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_assets::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		type ForeignAssetModifierOrigin: EnsureOrigin<Self::Origin>;
	}

	#[pallet::storage]
	#[pallet::getter(fn asset_id_multilocation)]
	pub type AssetIdMultiLocation<T: Config> =
		StorageMap<_, Blake2_128Concat, T::AssetId, MultiLocation>;

	#[pallet::storage]
	#[pallet::getter(fn asset_multilocation_id)]
	pub type AssetMultiLocationId<T: Config> =
		StorageMap<_, Blake2_128Concat, MultiLocation, T::AssetId>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ForeignAssetRegistered {
			asset_id: T::AssetId,
			asset: MultiLocation,
			metadata: ForeignAssetMetadata,
		},
		ForeignAssetMultiLocationChanged {
			asset_id: T::AssetId,
			new_asset_multi_location: MultiLocation,
		},
		ForeignAssetRemoved {
			asset_id: T::AssetId,
			asset_multi_location: MultiLocation,
		},
		ForeignAssetDestroyed {
			asset_id: T::AssetId,
			asset_multi_location: MultiLocation,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		ErrorCreatingAsset,
		AssetAlreadyExists,
		AssetDoesNotExist,
		ErrorDestroyingAsset,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn register_foreign_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			asset_multi_location: MultiLocation,
			metadata: ForeignAssetMetadata,
			min_amount: T::Balance,
			is_sufficient: bool,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			ensure!(
				AssetIdMultiLocation::<T>::get(&asset_id).is_none(),
				Error::<T>::AssetAlreadyExists
			);

			Self::do_create_foreign_asset(asset_id, min_amount, metadata.clone(), is_sufficient)
				.map_err(|_| Error::<T>::ErrorCreatingAsset)?;

			AssetIdMultiLocation::<T>::insert(&asset_id, &asset_multi_location);
			AssetMultiLocationId::<T>::insert(&asset_multi_location, &asset_id);

			Self::deposit_event(Event::ForeignAssetRegistered {
				asset_id,
				asset: asset_multi_location,
				metadata,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn change_foreign_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			new_asset_multi_location: MultiLocation,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			let previous_asset_multi_location =
				AssetIdMultiLocation::<T>::get(&asset_id).ok_or(Error::<T>::AssetDoesNotExist)?;

			AssetIdMultiLocation::<T>::insert(&asset_id, &new_asset_multi_location);
			AssetMultiLocationId::<T>::insert(&new_asset_multi_location, &asset_id);

			AssetMultiLocationId::<T>::remove(&previous_asset_multi_location);

			Self::deposit_event(Event::ForeignAssetMultiLocationChanged {
				asset_id,
				new_asset_multi_location,
			});

			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn destroy_foreign_asset(
			origin: OriginFor<T>,
			asset_id: T::AssetId,
			destroy_asset_witness: pallet_assets::DestroyWitness,
		) -> DispatchResult {
			T::ForeignAssetModifierOrigin::ensure_origin(origin)?;

			Self::do_destroy_foreign_asset(asset_id, destroy_asset_witness)
				.map_err(|_| Error::<T>::ErrorDestroyingAsset)?;

			let asset_multi_location =
				AssetIdMultiLocation::<T>::get(&asset_id).ok_or(Error::<T>::AssetDoesNotExist)?;

			AssetIdMultiLocation::<T>::remove(&asset_id);
			AssetMultiLocationId::<T>::remove(&asset_multi_location);

			Self::deposit_event(Event::ForeignAssetDestroyed { asset_id, asset_multi_location });
			Ok(())
		}
	}

	impl<T: Config> xcm_primitives::AssetMultiLocationGetter<T::AssetId> for Pallet<T> {
		fn get_asset_multi_location(asset_id: T::AssetId) -> Option<MultiLocation> {
			AssetIdMultiLocation::<T>::get(asset_id)
		}

		fn get_asset_id(asset_type: MultiLocation) -> Option<T::AssetId> {
			AssetMultiLocationId::<T>::get(asset_type)
		}
	}

	impl<T: Config> Pallet<T> {
		fn pallet_account_id() -> T::AccountId {
			PALLET_ID.into_account_truncating()
		}

		fn do_create_foreign_asset(
			asset: T::AssetId,
			min_balance: T::Balance,
			metadata: ForeignAssetMetadata,
			is_sufficient: bool,
		) -> DispatchResult {
			pallet_assets::Pallet::<T>::force_create(
				frame_system::RawOrigin::Root.into(),
				asset,
				<T::Lookup as sp_runtime::traits::StaticLookup>::unlookup(Self::pallet_account_id()),
				is_sufficient,
				min_balance,
			)?;

			pallet_assets::Pallet::<T>::force_set_metadata(
				frame_system::RawOrigin::Root.into(),
				asset,
				metadata.name,
				metadata.symbol,
				metadata.decimals,
				metadata.is_frozen,
			)
		}

		fn do_destroy_foreign_asset(
			asset: T::AssetId,
			witness: pallet_assets::DestroyWitness,
		) -> DispatchResult {
			pallet_assets::Pallet::<T>::destroy(
				frame_system::RawOrigin::Root.into(),
				asset,
				witness,
			)
			.map_err(|info| info.error)?;
			Ok(())
		}
	}
}
