#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use log;



#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	pub use scale_info::Type;
	use frame_support::{
		dispatch::DispatchResult,
		traits::{
			fungibles::{
				metadata::Mutate as MutateMetadata, Create, Inspect,
				Mutate, Transfer
			},
			tokens::nonfungibles::{Inspect as NonFungiblesInspect, Transfer as NonFungiblesTransfer},
			Currency,
		},
		PalletId,
	};
	use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned, Saturating, Zero, StaticLookup,IntegerSquareRoot};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the collection of item.
		type CollectionId: Member + Parameter + MaxEncodedLen + Copy;

		/// The type used to identify a unique item within a collection.
		type ItemId: Member + Parameter + MaxEncodedLen + Copy;

		type CurrencyBalance: AtLeast32BitUnsigned
			+ codec::FullCodec
			+ Copy
			+ MaybeSerializeDeserialize
			+ sp_std::fmt::Debug
			+ Default
			+ From<u64>
			+ TypeInfo
			+ Saturating
			+ Zero
			+ MaxEncodedLen;

		type AssetBalance: AtLeast32BitUnsigned
			+ codec::FullCodec
			+ Copy
			+ MaybeSerializeDeserialize
			+ sp_std::fmt::Debug
			+ Default
			+ From<u64>
			+ IntegerSquareRoot
			+ Zero
			+ TypeInfo
			+ MaxEncodedLen;

		type AssetId: Member
			+ Parameter
			+ Default
			+ Copy
			+ codec::HasCompact
			+ From<u32>
			+ MaybeSerializeDeserialize
			+ MaxEncodedLen
			+ PartialOrd
			+ TypeInfo;

		type Assets: Inspect<Self::AccountId, AssetId = Self::AssetId, Balance = Self::AssetBalance>
			+ Create<Self::AccountId>
			+ Transfer<Self::AccountId>
			+ Mutate<Self::AccountId>
			+ MutateMetadata<Self::AccountId>;

		type Items: NonFungiblesInspect<Self::AccountId, ItemId = Self::ItemId, CollectionId = Self::CollectionId> 
		+ NonFungiblesTransfer<Self::AccountId>;

		#[pallet::constant]
		type PalletId: Get<PalletId>;
	}

	pub type ItemId = <Type as pallet_uniques::Config>::ItemId;
	pub type CollectionId = <Type as pallet_uniques::Config>::CollectionId;

	pub type AssetIdOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::AssetId;
	pub type AssetBalanceOf<T> =
	<<T as Config>::Assets as Inspect<<T as frame_system::Config>::AccountId>>::Balance;
	pub type AccountIdLookupOf<T> = <<T as frame_system::Config>::Lookup as StaticLookup>::Source;

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		NFTLocked(
			T::CollectionId,
			T::ItemId,
		)
	}

	#[pallet::error]
	pub enum Error<T> {
		AssetAlreadyRegistered,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000 + T::DbWeight::get().writes(2).ref_time())]
		pub fn lock_nft_create_asset(
			origin: OriginFor<T>,
			collection_id: T::CollectionId,
			item_id: T::ItemId,
			asset_id: T::AssetId,
			beneficiary: AccountIdLookupOf<T>,
			amount: AssetBalanceOf<T>,
		) -> DispatchResult {
			let _who = ensure_signed(origin)?;
			let admin_account_id = Self::pallet_account_id();
			log::info!(target: "nft-loan", "admin_account_id: {:?}", admin_account_id);
			let dest = T::Lookup::lookup(beneficiary)?;
			//Self::do_lock_nft(origin, collection_id, item_id);
			T::Items::transfer(&collection_id, &item_id,  &admin_account_id)?;
			Self::deposit_event(Event::NFTLocked(collection_id, item_id));
			T::Assets::transfer(asset_id, &admin_account_id, &dest, amount, true)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		fn pallet_account_id() -> T::AccountId {
			T::PalletId::get().into_account_truncating()
		}
	}
}
