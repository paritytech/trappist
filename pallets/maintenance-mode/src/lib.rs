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
	use cumulus_primitives_core::{
		relay_chain::BlockNumber as RelayBlockNumber, DmpMessageHandler,
	};
	use frame_support::{
		pallet_prelude::{ValueQuery, *},
		traits::Contains,
	};
	use frame_system::pallet_prelude::*;
	pub use log;
	use sp_std::vec::Vec;
	use xcm_primitives::PauseXcmExecution;
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
		type MaintenanceDmpHandler: DmpMessageHandler;
		type XcmExecutorManager: PauseXcmExecution;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type MaintenanceModeOnOff<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MaintenanceModeActivated,
		MaintenanceModeDeactivated,
		/// The call to suspend on_idle XCM execution failed with inner error
		FailedToSuspendIdleXcmExecution {
			error: DispatchError,
		},
		/// The call to resume on_idle XCM execution failed with inner error
		FailedToResumeIdleXcmExecution {
			error: DispatchError,
		},
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

			if let Err(error) = T::XcmExecutorManager::suspend_xcm_execution() {
				<Pallet<T>>::deposit_event(Event::FailedToSuspendIdleXcmExecution { error });
			}

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

			if let Err(error) = T::XcmExecutorManager::resume_xcm_execution() {
				<Pallet<T>>::deposit_event(Event::FailedToResumeIdleXcmExecution { error });
			}

			Self::deposit_event(Event::MaintenanceModeDeactivated);

			Ok(())
		}
	}

	impl<T: Config> Contains<T::RuntimeCall> for Pallet<T> {
		fn contains(call: &T::RuntimeCall) -> bool {
			log::info!("Pallet Contains: {:?}", call);
			if MaintenanceModeOnOff::<T>::get() {
				T::FilteredCalls::contains(call)
			} else {
				log::info!("Maintenance Mode is off, all calls are allowed");
				return true
			}
		}
	}

	impl<T: Config> DmpMessageHandler for Pallet<T> {
		fn handle_dmp_messages(
			iter: impl Iterator<Item = (RelayBlockNumber, Vec<u8>)>,
			limit: Weight,
		) -> Weight {
			if MaintenanceModeOnOff::<T>::get() {
				T::MaintenanceDmpHandler::handle_dmp_messages(iter, Weight::zero())
			} else {
				// Normal path, everything should pass through
				T::MaintenanceDmpHandler::handle_dmp_messages(iter, limit)
			}
		}
	}
}
