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
		IntoVersion, VersionedMultiAssets,
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
	type VecTrappedAssetsOf<T> = BoundedVec<TrappedAssets, <T as Config>::MaxTrapsPerOrigin>;

	/// Keeps track of trapped `MultiAssets`, where n is a counter of
	/// how many times the `MultiAssets` has been trapped under some specific origin
	#[derive(Clone, Debug, Encode, Decode, TypeInfo, PartialEq)]
	pub struct TrappedAssets {
		pub multi_assets: VersionedMultiAssets,
		pub n: u32,
	}

	impl TrappedAssets {
		fn increment(&mut self) {
			self.n += 1;
		}

		fn decrement(&mut self) {
			self.n -= 1;
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

		#[pallet::constant]
		type MaxTrapsPerOrigin: Get<u32>;
	}

	/// The existing asset traps.
	///
	/// Key is the asset's `MultiLocation`
	/// Value is a `Vec` of `TrappedAssets`
	#[pallet::storage]
	#[pallet::getter(fn asset_trap)]
	pub(super) type AssetTraps<T: Config> =
		StorageMap<_, Identity, MultiLocation, VecTrappedAssetsOf<T>, OptionQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Some asset have been placed in an asset trap.
		///
		/// \[ origin, assets \]
		AssetsTrapped(MultiLocation, VersionedMultiAssets),
		/// Some asset have been claimed from an asset trap.
		///
		/// \[ origin, assets \]
		AssetsClaimed(MultiLocation, VersionedMultiAssets),
	}

	#[pallet::error]
	pub enum Error<T> {}

	#[pallet::call]
	impl<T: Config> Pallet<T> {}

	impl<T: Config> DropAssets for Pallet<T> {
		fn drop_assets(origin: &MultiLocation, assets: Assets) -> u64 {
			let multi_assets: Vec<MultiAsset> = assets.into();
			let mut trap: Vec<MultiAsset> = Vec::new();

			for asset in multi_assets {
				if let MultiAsset { id: Concrete(location), fun: Fungible(amount) } = asset.clone()
				{
					// is location a fungible on AssetRegistry?
					if let Some(asset_id) = T::AssetRegistry::get_asset_id(location.clone()) {
						let min_balance = T::Assets::minimum_balance(asset_id);

						// only trap if amount ≥ min_balance
						// do nothing otherwise (asset is lost)
						if min_balance <= amount.saturated_into::<AssetBalanceOf<T>>() {
							trap.push(asset);
						}

					// is location the native token?
					} else if location == (MultiLocation { parents: 0, interior: Here }) {
						let min_balance =
							<<T as Config>::Balances as Currency<T::AccountId>>::minimum_balance();

						// only trap if amount ≥ min_balance
						// do nothing otherwise (asset is lost)
						if min_balance <= amount.saturated_into::<CurrencyBalanceOf<T>>() {
							trap.push(asset);
						}
					}
				}
			}

			if !trap.is_empty() {
				let versioned: VersionedMultiAssets = VersionedMultiAssets::from(trap);
				Self::trap(origin.clone(), versioned);
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
			// checks ticket for matching versions
			let mut versioned = VersionedMultiAssets::from(assets.clone());
			match (ticket.parents, &ticket.interior) {
				(0, X1(GeneralIndex(i))) => {
					versioned = match versioned.into_version(*i as u32) {
						Ok(v) => v,
						Err(()) => return false,
					}
				},
				(0, Here) => (),
				_ => return false,
			};

			let trapped_assets_inner_vec: Vec<TrappedAssets> =
				match AssetTraps::<T>::get(origin.clone()) {
					Some(v) => v
						.into_inner()
						.into_iter()
						.map(|mut t| {
							if t.multi_assets == versioned {
								t.decrement();
								t
							} else {
								t
							}
						})
						.filter(|t| t.non_zero())
						.collect(),
					None => return false,
				};

			match trapped_assets_inner_vec.len() {
				0 => AssetTraps::<T>::remove(origin),
				_ => {
					let bounded_trapped_assets: VecTrappedAssetsOf<T> = trapped_assets_inner_vec.try_into().expect("inner vec len is either equal or smaller than bound, therefore try_into can never fail");
					AssetTraps::<T>::insert(origin, bounded_trapped_assets)
				},
			}

			Self::deposit_event(Event::AssetsClaimed(origin.clone(), versioned));

			return false;
		}
	}

	impl<T: Config> Pallet<T> {
		fn trap(origin: MultiLocation, trap: VersionedMultiAssets) {
			let trapped_assets = match AssetTraps::<T>::get(origin.clone()) {
				// storage map is empty, we just initalize a new Vec<TrappedAssets>
				None => {
					let v = vec![TrappedAssets { multi_assets: trap.clone(), n: 1 }];
					v.try_into().expect("v has only 1 item, therefore try_into can never fail")
				},
				// storage map is not empty, we have to check the vec
				Some(v) => match v.clone().into_inner().iter().any(|t| t.multi_assets == trap) {
					// do we simply increment TrappedAssets?
					true => v
						.into_inner()
						.into_iter()
						.map(|mut t| {
							if t.multi_assets == trap {
								t.increment();
								t
							} else {
								t
							}
						})
						.collect::<Vec<TrappedAssets>>()
						.try_into()
						.expect("len stayed the same, therefore try_into can never fail"),
					// do we append the BoundedVec<TrappedAssets>?
					false => {
						let mut new_v = v.clone();
						let t = TrappedAssets { multi_assets: trap.clone(), n: 1 };

						// is the BoundedVec full? remove 0th element first
						if new_v.len() as u32 == T::MaxTrapsPerOrigin::get() {
							new_v.remove(0);
							new_v
								.try_push(t)
								.expect("we just decreased len, therefore try_push can never fail");
						// we can simply push
						} else {
							new_v
								.try_push(t)
								.expect("we just checked len, therefore try_push can never fail");
						}
						new_v
					},
				},
			};

			AssetTraps::<T>::insert(origin.clone(), trapped_assets);
			Self::deposit_event(Event::AssetsTrapped(origin, trap));
		}
	}
}
