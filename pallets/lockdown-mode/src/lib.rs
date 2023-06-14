// This file is part of Trappist.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

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

pub const ACTIVATED: bool = true;
pub const DEACTIVATED: bool = false;

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
	use sp_std::vec::Vec;
	use xcm_primitives::PauseXcmExecution;
	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::genesis_config]
	pub struct GenesisConfig {
		pub initial_status: bool,
	}

	#[cfg(feature = "std")]
	impl Default for GenesisConfig {
		fn default() -> Self {
			Self { initial_status: ACTIVATED }
		}
	}

	#[pallet::genesis_build]
	impl<T: Config> GenesisBuild<T> for GenesisConfig {
		fn build(&self) {
			LockdownModeStatus::<T>::put(self.initial_status);
		}
	}

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type LockdownModeOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type BlackListedCalls: Contains<Self::RuntimeCall>;
		type LockdownDmpHandler: DmpMessageHandler;
		type XcmExecutorManager: PauseXcmExecution;
		type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	pub type LockdownModeStatus<T: Config> = StorageValue<_, bool, ValueQuery>;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		LockdownModeActivated,
		LockdownModeDeactivated,
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
		/// Lockdown mode was already activated
		LockdownModeAlreadyActivated,
		/// Lockdown mode was already deactivated
		LockdownModeAlreadyDeactivated,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::activate_lockdown_mode())]
		pub fn activate_lockdown_mode(origin: OriginFor<T>) -> DispatchResult {
			T::LockdownModeOrigin::ensure_origin(origin)?;

			ensure!(!LockdownModeStatus::<T>::get(), Error::<T>::LockdownModeAlreadyActivated);

			LockdownModeStatus::<T>::put(ACTIVATED);

			if let Err(error) = T::XcmExecutorManager::suspend_xcm_execution() {
				log::error!("Failed to suspend idle XCM execution {:?}", error);
				Self::deposit_event(Event::FailedToSuspendIdleXcmExecution { error });
			}

			Self::deposit_event(Event::LockdownModeActivated);

			Ok(())
		}

		#[pallet::call_index(1)]
		#[pallet::weight(<T as pallet::Config>::WeightInfo::deactivate_lockdown_mode())]
		pub fn deactivate_lockdown_mode(origin: OriginFor<T>) -> DispatchResult {
			T::LockdownModeOrigin::ensure_origin(origin)?;
			ensure!(LockdownModeStatus::<T>::get(), Error::<T>::LockdownModeAlreadyDeactivated);

			LockdownModeStatus::<T>::put(DEACTIVATED);

			if let Err(error) = T::XcmExecutorManager::resume_xcm_execution() {
				log::error!("Failed to resume idle XCM execution {:?}", error);
				Self::deposit_event(Event::FailedToResumeIdleXcmExecution { error });
			}

			Self::deposit_event(Event::LockdownModeDeactivated);

			Ok(())
		}
	}

	impl<T: Config> Contains<T::RuntimeCall> for Pallet<T> {
		fn contains(call: &T::RuntimeCall) -> bool {
			!LockdownModeStatus::<T>::get() || T::BlackListedCalls::contains(call)
		}
	}

	impl<T: Config> DmpMessageHandler for Pallet<T> {
		fn handle_dmp_messages(
			iter: impl Iterator<Item = (RelayBlockNumber, Vec<u8>)>,
			limit: Weight,
		) -> Weight {
			if LockdownModeStatus::<T>::get() {
				T::LockdownDmpHandler::handle_dmp_messages(iter, Weight::zero())
			} else {
				// Normal path, everything should pass through
				T::LockdownDmpHandler::handle_dmp_messages(iter, limit)
			}
		}
	}
}
