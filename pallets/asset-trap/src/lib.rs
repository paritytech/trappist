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
	use frame_support::{
		inherent::Vec,
		pallet_prelude::*,
		traits::{tokens::fungibles::Inspect, Currency},
	};
	use scale_info::prelude::vec;
	use sp_runtime::SaturatedConversion;
	use xcm::{
		latest::{Junction::GeneralIndex, Junctions::*, MultiAssets, MultiLocation},
		opaque::latest::{AssetId::Concrete, Fungibility::Fungible, MultiAsset},
		VersionedMultiAsset, VersionedMultiAssets,
	};
	use xcm_executor::{
		traits::{ClaimAssets, DropAssets},
		Assets,
	};
	use xcm_primitives::AssetMultiLocationGetter;

	#[pallet::pallet]
	#[pallet::without_storage_info]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	type AssetIdOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
	type AssetBalanceOf<T> =
		<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
	type CurrencyBalanceOf<T> =
		<<T as Config>::Balances as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	#[derive(Clone, Debug, PartialEq, Encode, Decode, TypeInfo, MaxEncodedLen)]
	pub enum MultiAssetVersion {
		V0,
		V1,
	}

	/// Keeps track of trapped assets, where n is a counter of
	/// how many times the asset has been trapped
	#[derive(Clone, Debug, Encode, Decode, MaxEncodedLen, TypeInfo, PartialEq)]
	pub struct TrappedAsset {
		pub origin: MultiLocation,
		pub amount: u128,
		pub multi_asset_version: MultiAssetVersion,
		pub n: u32,
	}

	impl TrappedAsset {
		fn increment(&mut self) {
			self.n += 1;
		}

		fn matches(&self, origin: MultiLocation, amount: u128, version: MultiAssetVersion) -> bool {
			self.origin == origin && self.amount == amount && self.multi_asset_version == version
		}

		fn non_zero(&self) -> bool {
			self.n > 0
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type Balances: Currency<Self::AccountId>;
		type Assets: Inspect<Self::AccountId>;
		type AssetRegistry: xcm_primitives::AssetMultiLocationGetter<AssetIdOf<Self>>;
	}

	/// The existing asset traps.
	///
	/// Key is the asset's `MultiLocation`
	/// Value is a `Vec` of `TrappedAsset`
	#[pallet::storage]
	#[pallet::getter(fn asset_trap)]
	pub(super) type AssetTraps<T: Config> =
		StorageMap<_, Identity, MultiLocation, Vec<TrappedAsset>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some asset have been placed in an asset trap.
		///
		/// \[ asset_multi_location, origin, amount, multi_asset_version \]
		AssetTrapped(MultiLocation, MultiLocation, u128, MultiAssetVersion),
		/// Some asset have been claimed from an asset trap.
		///
		/// \[ asset_multi_location, origin, amount, multi_asset_version \]
		AssetClaimed(MultiLocation, MultiLocation, u128, MultiAssetVersion),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> DropAssets for Pallet<T> {
		fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
			let multi_assets: Vec<MultiAsset> = assets.into();

			for asset in multi_assets {
				if let MultiAsset { id: Concrete(location), fun: Fungible(amount) } = asset.clone()
				{
					// is location a fungible on AssetRegistry?
					if let Some(asset_id) = T::AssetRegistry::get_asset_id(location.clone()) {
						let min_balance = T::Assets::minimum_balance(asset_id);

						// only trap if amount is more than min_balance
						// do nothing otherwise (asset is lost)
						if min_balance <= amount.saturated_into::<AssetBalanceOf<T>>() {
							let version = Self::multi_asset_version(asset.clone());
							Self::trap_asset(origin.clone(), location, amount, version);
						}

					// is location the native token?
					} else if location == (MultiLocation { parents: 0, interior: Here }) {
						let min_balance =
							<<T as Config>::Balances as Currency<T::AccountId>>::minimum_balance();

						// only trap if amount is more than min_balance
						// do nothing otherwise (asset is lost)
						if min_balance <= amount.saturated_into::<CurrencyBalanceOf<T>>() {
							let version = Self::multi_asset_version(asset.clone());
							Self::trap_asset(origin.clone(), location, amount, version);
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
			// only supports claiming one asset at a time
			if assets.len() > 1 {
				return false;
			}

			// checks ticket for matching versions
			let version = match (ticket.parents, &ticket.interior) {
				(0, X1(GeneralIndex(i))) => match i {
					0 => MultiAssetVersion::V0,
					1 => MultiAssetVersion::V1,
					_ => return false,
				},
				(0, Here) => match VersionedMultiAssets::from(assets.clone()) {
					VersionedMultiAssets::V0(_) => MultiAssetVersion::V0,
					VersionedMultiAssets::V1(_) => MultiAssetVersion::V1,
				},
				_ => return false,
			};

			let asset: &MultiAsset = &assets.inner()[0];
			if let MultiAsset { id: Concrete(asset_multi_location), fun: Fungible(amount) } = asset
			{
				let mut r = false;
				let trapped_assets: Vec<TrappedAsset> =
					match AssetTraps::<T>::get(asset_multi_location.clone()) {
						Some(v) => v
							.into_iter()
							.map(|trapped_asset| {
								if trapped_asset.matches(
									origin.clone(),
									amount.clone(),
									version.clone(),
								) {
									r = true;
									TrappedAsset {
										origin: origin.clone(),
										amount: amount.clone(),
										multi_asset_version: version.clone(),
										n: trapped_asset.n - 1,
									}
								} else {
									trapped_asset
								}
							})
							.filter(|trapped_asset| trapped_asset.non_zero())
							.collect(),
						None => return false,
					};

				match trapped_assets.len() {
					0 => AssetTraps::<T>::remove(asset_multi_location),
					_ => AssetTraps::<T>::insert(asset_multi_location, trapped_assets),
				}

				Self::deposit_event(Event::AssetClaimed(
					asset_multi_location.clone(),
					origin.clone(),
					amount.clone(),
					version,
				));
				return r;
			}

			// todo: else if let MultiAsset { id: Abstract ... }

			return false;
		}
	}

	impl<T: Config> Pallet<T> {
		fn trap_asset(
			origin: MultiLocation,
			location: MultiLocation,
			amount: u128,
			version: MultiAssetVersion,
		) {
			let trapped_assets = match AssetTraps::<T>::get(location.clone()) {
				// storage map is empty, we just initalize a new Vec<TrappedAsset>
				None => vec![TrappedAsset {
					origin: origin.clone(),
					amount,
					multi_asset_version: version.clone(),
					n: 1,
				}],
				// storage map is not empty, we have to check the vec
				Some(v) => match v.iter().any(|trapped_asset| {
					trapped_asset.matches(origin.clone(), amount, version.clone())
				}) {
					// do we simply increment TrappedAsset?
					true => v
						.into_iter()
						.map(|mut trapped_asset| {
							if trapped_asset.matches(origin.clone(), amount, version.clone()) {
								trapped_asset.increment();
								trapped_asset
							} else {
								trapped_asset
							}
						})
						.collect(),
					// do we append the Vec<TrappedAsset>?
					false => {
						let mut new_v = v.clone();
						new_v.push(TrappedAsset {
							origin: origin.clone(),
							amount,
							multi_asset_version: version.clone(),
							n: 1,
						});
						new_v
					},
				},
			};

			AssetTraps::<T>::insert(location.clone(), trapped_assets);
			Self::deposit_event(Event::AssetTrapped(location, origin, amount, version.clone()));
		}

		fn multi_asset_version(asset: MultiAsset) -> MultiAssetVersion {
			match VersionedMultiAsset::from(asset.clone()) {
				VersionedMultiAsset::V0(_) => MultiAssetVersion::V0,
				VersionedMultiAsset::V1(_) => MultiAssetVersion::V1,
			}
		}
	}
}
