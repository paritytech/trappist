#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/v3/runtime/frame>
pub use pallet::*;

use frame_support::traits::{Currency, ReservableCurrency};

use sp_std::{
	boxed::Box,
	convert::{TryFrom, TryInto},
	marker::PhantomData,
	prelude::*,
	result::Result,
	vec,
};

use cumulus_primitives_core::ParaId;

use xcm::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	use pallet_xcm::{EnsureXcm, IsMajorityOfBody, XcmPassthrough};
	use xcm::{latest::prelude::*, prelude::*};
	use xcm_builder::{
		AccountId32Aliases, AllowKnownQueryResponses, AllowSubscriptionsFrom,
		AllowTopLevelPaidExecutionFrom, AllowUnpaidExecutionFrom, CurrencyAdapter, EnsureXcmOrigin,
		FixedWeightBounds, FungiblesAdapter, IsConcrete, LocationInverter, NativeAsset,
		ParentAsSuperuser, ParentIsPreset, RelayChainAsNative, SiblingParachainAsNative,
		SiblingParachainConvertsVia, SignedAccountId32AsNative, SignedToAccountId32,
		SovereignSignedViaLocation, TakeWeightCredit, UsingComponents,
	};

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_xcm::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		/// The currency trait.
		type Currency: ReservableCurrency<Self::AccountId>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	// The pallet's runtime storage items.
	// https://docs.substrate.io/v3/runtime/storage
	#[pallet::storage]
	#[pallet::getter(fn something)]
	// Learn more about declaring storage items:
	// https://docs.substrate.io/v3/runtime/storage#declaring-storage-items
	pub type Something<T> = StorageValue<_, u32>;

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(u32, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		InvalidAmount,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn do_something(origin: OriginFor<T>, something: u32) -> DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://docs.substrate.io/v3/runtime/origins
			let who = ensure_signed(origin)?;

			// Update storage.
			<Something<T>>::put(something);

			// Emit an event.
			Self::deposit_event(Event::SomethingStored(something, who));
			// Return a successful DispatchResultWithPostInfo
			Ok(())
		}

		/// Teleport the relay chain's native asset to one of its parachain (using DMP).
		///
		/// Fee payment on the destination side is made in the native currency.
		/// Note: fee-weight is calculated locally and thus remote weights are assumed to be
		/// equal to local weights.
		///
		/// - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
		/// - `dest_para_id`: id of the parachain the assets are being teleported to.
		/// - `dest_account`: a beneficiary account of the parachain for the teleported asset.
		/// - `amount`: amount of native currency to be withdrawn.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn xcm_teleport_native_asset_down(
			origin: OriginFor<T>,
			dest_para_id: ParaId,
			dest_account_id: [u8; 32],
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let parachain: VersionedMultiLocation =
				(Parachain(dest_para_id.into()).into() as MultiLocation).into();

			let beneficiary: VersionedMultiLocation =
				(AccountId32 { network: Any, id: dest_account_id }.into() as MultiLocation).into();

			let asset_amount = amount.try_into().map_err(|_| Error::<T>::InvalidAmount)?;
			let assets: Vec<MultiAsset> =
				vec![(AssetId::Concrete(Here.into()), Fungibility::Fungible(asset_amount)).into()];

			pallet_xcm::Pallet::<T>::teleport_assets(
				origin,
				Box::new(parachain),
				Box::new(beneficiary),
				Box::new(assets.into()),
				0,
			)
		}

		/// Teleport a parachain's native asset to a relay chain account (using UMP).
		/// This only works for parachains that are using the relay chain's native currency as their
		/// own currency, typically for common good parachains e.g. Statemine/t, that the relay chain
		/// trusts for upwards teleports (see 'TrustedTeleporters' in Rococo/Kusama/Polakdot's XCM config).
		///
		/// Fee payment on the destination side is made in the native currency.
		/// Note: fee-weight is calculated locally and thus remote weights are assumed to be
		/// equal to local weights.
		///
		/// - `origin`: Must be capable of withdrawing the `assets` and executing XCM.
		/// - `dest_account_id`: A beneficiary `AccountId32` value for the assets on the relay chain.
		/// - `amount`: amount of native currency to be withdrawn.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn xcm_teleport_native_asset_up(
			origin: OriginFor<T>,
			dest_account_id: [u8; 32],
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let relaychain: VersionedMultiLocation = Parent.into();

			let beneficiary: VersionedMultiLocation =
				(AccountId32 { network: Any, id: dest_account_id }.into() as MultiLocation).into();

			let asset_amount = amount.try_into().map_err(|_| Error::<T>::InvalidAmount)?;
			let assets: Vec<MultiAsset> = vec![(
				AssetId::Concrete(Parent.into()),
				Fungibility::Fungible(asset_amount),
			)
				.into()];

			pallet_xcm::Pallet::<T>::teleport_assets(
				origin,
				Box::new(relaychain),
				Box::new(beneficiary),
				Box::new(assets.into()),
				0,
			)
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match <Something<T>>::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue.into()),
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					<Something<T>>::put(new);
					Ok(())
				},
			}
		}
	}
}
