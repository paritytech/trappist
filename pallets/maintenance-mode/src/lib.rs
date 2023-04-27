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

pub const ACTIVATE: bool = true;
pub const DEACTIVATE: bool = false;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::{
		pallet_prelude::{ValueQuery, *},
		traits::Contains,
	};
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(Default)]
	#[pallet::genesis_config]
	/// Genesis config for maintenance mode pallet
	pub struct GenesisConfig {}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			MaintenanceModeOnOff::<T>::put(ACTIVATE);
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type MaintenanceModeOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type FilteredCalls: Contains<Self::RuntimeCall>;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type MaintenanceModeOnOff<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MaintenanceModeActivated,
		MaintenanceModeDeactivated,
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Maintenance mode was already activated
		MaintenanceModeAlreadyActivated,
		/// Maintenance mode was already deactivated
		MaintenanceModeAlreadyDeactivated,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::activate_maintenance_mode())]
		pub fn activate_maintenance_mode(origin: OriginFor<T>) -> DispatchResult {
			T::MaintenanceModeOrigin::ensure_origin(origin)?;

			ensure!(!MaintenanceModeOnOff::<T>::get(), Error::<T>::MaintenanceModeAlreadyActivated);

			MaintenanceModeOnOff::<T>::put(ACTIVATE);

			// check xcm execution
			Self::deposit_event(Event::MaintenanceModeActivated);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::deactivate_maintenance_mode())]
		pub fn deactivate_maintenance_mode(origin: OriginFor<T>) -> DispatchResult {
			T::MaintenanceModeOrigin::ensure_origin(origin)?;

			ensure!(
				MaintenanceModeOnOff::<T>::get(),
				Error::<T>::MaintenanceModeAlreadyDeactivated
			);

			MaintenanceModeOnOff::<T>::put(DEACTIVATE);

			// check xcm execution
			Self::deposit_event(Event::MaintenanceModeDeactivated);

			Ok(())
		}
	}

	impl<T: Config> Contains<T::RuntimeCall> for Pallet<T> {
		fn contains(call: &T::RuntimeCall) -> bool {
			if MaintenanceModeOnOff::<T>::get() {
				T::FilteredCalls::contains(call)
			} else {
				return false
			}
		}
	}
}
