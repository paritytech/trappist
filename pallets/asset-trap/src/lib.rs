#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::*, traits::{Currency, tokens::fungibles::Inspect}};
	use sp_runtime::{traits::{BlakeTwo256, Hash}, SaturatedConversion};
	use sp_core::H256;
	use xcm::{opaque::latest::{MultiAsset, AssetId::Concrete, Fungibility::Fungible}, IntoVersion, VersionedMultiAssets, latest::{MultiAssets, MultiLocation, Junctions::*, Junction::GeneralIndex}};
	use xcm_executor::{Assets, traits::{DropAssets, ClaimAssets}};
	use xcm_primitives::AssetMultiLocationGetter;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	type AssetIdOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
	type AssetBalanceOf<T> = <<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
	type CurrencyBalanceOf<T> = <<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Balances: Currency<Self::AccountId>;
		type Assets: Inspect<Self::AccountId>;
		type AssetRegistry: xcm_primitives::AssetMultiLocationGetter<AssetIdOf<Self>>;
	}

	/// The existing asset traps.
	///
	/// Key is the blake2 256 hash of (origin, `VersionedMultiAssets`) pair. 
	/// Value is a tuple containing:
	/// - `MultiLocation` of the asset that is trapped
	/// - the number of times this (origin, `VersionedMultiAssets`) pair has been trapped (usually just 1 if it exists at all).
	/// 
	/// In case multiple assets are to be trapped, they are stored separately (multiple hashes)
	#[pallet::storage]
	#[pallet::getter(fn asset_trap)]
	pub(super) type AssetTraps<T: Config> = StorageMap<_, Identity, H256, (MultiLocation, u32), OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some asset have been placed in an asset trap.
		///
		/// \[ hash, origin, assets \]
		AssetTrapped(H256, MultiLocation, VersionedMultiAssets),
		/// Some asset have been claimed from an asset trap.
		///
		/// \[ hash, origin, assets \]
		AssetClaimed(H256, MultiLocation, VersionedMultiAssets)
	}

	#[pallet::error]
	pub enum Error<T> {
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
	}

	impl<T: Config> DropAssets for Pallet<T> {
		fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
			let multi_assets: Vec<MultiAsset> = assets.into();

			for asset in multi_assets {
				if let MultiAsset {
					id: Concrete(location),
					fun: Fungible(amount),
				} = asset.clone()
				{
					// is location a fungible on AssetRegistry?
					if let Some(asset_id) = T::AssetRegistry::get_asset_id(location.clone()) {
						let min_balance = T::Assets::minimum_balance(asset_id);

						// burn asset (do nothing) if amount is less than min_balance
						if min_balance <= amount.saturated_into::<AssetBalanceOf<T>>() {
							Self::trap_asset(origin, asset, location);
						}
					
					// is location the native token?
					} else if location == (MultiLocation { parents: 0, interior: Here }) {
						let min_balance = <<T as Config>::Balances as Currency<T::AccountId>>::minimum_balance();

						// burn asset (do nothing) if amount is less than min_balance
						if min_balance <= amount.saturated_into::<CurrencyBalanceOf<T>>() {
							Self::trap_asset(origin, asset, location);
						}
						
					}
				}
			}

			// TODO #3735: Put the real weight in there.
			0
		}
	}

	impl<T: Config> ClaimAssets for Pallet<T> {
		fn claim_assets(
			origin: &MultiLocation,
			ticket: &MultiLocation,
			assets: &MultiAssets,
		) -> bool {
			let mut versioned = VersionedMultiAssets::from(assets.clone());
			// checks ticket for matching versions
			match (ticket.parents, &ticket.interior) {
				(0, X1(GeneralIndex(i))) =>
					versioned = match versioned.into_version(*i as u32) {
						Ok(v) => v,
						Err(()) => return false,
					},
				(0, Here) => (),
				_ => return false,
			};

			let hash = BlakeTwo256::hash_of(&(origin, versioned.clone()));
			match AssetTraps::<T>::get(hash) {
				None => return false,
				Some((_, 1)) => { 
					AssetTraps::<T>::remove(hash); 
					Self::deposit_event(Event::AssetClaimed(hash, origin.clone(), versioned));
				},
				Some((ml, n)) => {
					AssetTraps::<T>::insert(hash, (ml, n - 1));
					Self::deposit_event(Event::AssetClaimed(hash, origin.clone(), versioned));
				},
			}
			return true
		}
	}

	impl<T: Config> Pallet<T> {
		fn trap_asset(origin: &MultiLocation, asset: MultiAsset, location: MultiLocation) {
			let versioned = VersionedMultiAssets::from(MultiAssets::from(asset));
			let hash = BlakeTwo256::hash_of(&(&origin, &versioned));

			let trap = match AssetTraps::<T>::get(hash) {
				Some((_, n)) => (location, n+1),
				None => (location, 1)
			};

			AssetTraps::<T>::insert(hash, trap);

			Self::deposit_event(Event::AssetTrapped(hash, origin.clone(), versioned));
		}
	}
}
