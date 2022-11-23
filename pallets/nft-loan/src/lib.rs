#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

use log;

pub use xcm::opaque::latest::prelude::{
	Junction, Junctions, MultiLocation,
	Transact, OriginKind
};

pub use sp_std::{vec, boxed::Box};

use frame_system::{pallet_prelude::OriginFor};
use frame_support::traits::{Get};
use sp_runtime::traits::{AccountIdConversion};


pub use xcm::{VersionedMultiAsset, VersionedMultiLocation, VersionedResponse, VersionedXcm, v3::{Xcm,WeightLimit,Fungibility,AssetId,Parent,WildMultiAsset,MultiAsset,MultiAssets,MultiAssetFilter,Instruction::{DepositReserveAsset,InitiateReserveWithdraw,BuyExecution,DepositAsset,WithdrawAsset}}};
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
				Mutate, Transfer, 
			},
			tokens::nonfungibles::{Inspect as NonFungiblesInspect, Transfer as NonFungiblesTransfer},
		},
		PalletId,
	};
	use sp_runtime::traits::{AtLeast32BitUnsigned, Zero, StaticLookup,IntegerSquareRoot};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config:
		frame_system::Config + pallet_xcm::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Identifier for the collection of item.
		type CollectionId: Member + Parameter + MaxEncodedLen + Copy;

		/// The type used to identify a unique item within a collection.
		type ItemId: Member + Parameter + MaxEncodedLen + Copy;

		type AssetBalance: AtLeast32BitUnsigned
			+ codec::FullCodec
			+ Copy
			+ MaybeSerializeDeserialize
			+ sp_std::fmt::Debug
			+ Default
			+ From<u64>
			+ Into<u128>
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
			amount: AssetBalanceOf<T>,
			// amount: u128,
		) -> DispatchResult {
			let _who = ensure_signed(origin.clone())?;
			let admin_account_id = Self::pallet_account_id();
			let value: u128 = amount.into();
			let asset_id: T::AssetId =  1u32.into();
			T::Items::transfer(&collection_id, &item_id,  &admin_account_id)?;
			Self::deposit_event(Event::NFTLocked(collection_id, item_id));
			T::Assets::transfer(asset_id, &admin_account_id, &_who, amount, true)?;

			Self::xcm_transfer(origin, value);


			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
		
	pub fn pallet_account_id() -> T::AccountId {
		T::PalletId::get().into_account_truncating()
	}

	pub fn xcm_transfer(origin: OriginFor<T>, amount: u128) {
		let account: [u8; 32]= [0,68,31,24,120,181,36,104,199,244,218,65,206,194,125,193,62,104,67,194,131,165,141,51,72,82,18,174,72,207,148,251];
		let para_1000 = Junctions::X1(Junction::Parachain(1000));
		let para_2000 = Junctions::X1(Junction::Parachain(2000));
		let account_dest = Junctions::X1(Junction::AccountId32 { network: None, id: account });
		let reserved_asset = Junctions::X3(Junction::Parachain(1000), Junction::PalletInstance(50), Junction::GeneralIndex(1));
		let buy_execution_asset = Junctions::X2(Junction::PalletInstance(50), Junction::GeneralIndex(1));

		let reserve =MultiLocation {
			parents: 1,
			interior: para_1000,
		};
		let dest = MultiLocation {
			parents: 1,
			interior: para_2000,
		};
		let beneficiary = MultiLocation {
			parents: 0,
			interior: account_dest,
		};
		let reserved_location = MultiLocation {
			parents: 1,
			interior: reserved_asset,
		};

		let buy_asset_location = MultiLocation {
			parents: 0,
			interior: buy_execution_asset,
		};
		
		let fees = MultiAsset {
			id: AssetId::Concrete(buy_asset_location),
			fun: Fungibility::Fungible(1000000000000_u128)
		};
		let assets = MultiAssetFilter::Wild(WildMultiAsset::All);
		let mut multi_assets = MultiAssets::new();
		multi_assets.push(
			MultiAsset {
				id: AssetId::Concrete(reserved_location),
				fun: Fungibility::Fungible(amount)
			} 
		);

		let versioned_xcm = Box::new(VersionedXcm::from(Xcm(vec![
			WithdrawAsset(multi_assets),
			InitiateReserveWithdraw {
				assets: assets.clone(),
				reserve,
				xcm: Xcm(vec![
					BuyExecution { fees, weight_limit: WeightLimit::Unlimited},
					DepositReserveAsset {
						assets: assets.clone(),
						dest,
						xcm: Xcm([DepositAsset { assets: MultiAssetFilter::Wild(WildMultiAsset::All), beneficiary } ].into())
					}
				])
			}
		])));
		<pallet_xcm::Pallet<T>>::execute(origin, versioned_xcm, 500000000000).unwrap();

	}

	pub fn add_liquidity_remote(origin: OriginFor<T>) {
		let para_1000 = Junctions::X1(Junction::Parachain(1000));
		let para_2000 = Junctions::X1(Junction::Parachain(2000));
		let buy_execution_asset = Junctions::X2(Junction::PalletInstance(50), Junction::GeneralIndex(1));
		let reserved_asset = Junctions::X3(Junction::Parachain(1000), Junction::PalletInstance(50), Junction::GeneralIndex(1));
		let dest = MultiLocation {
			parents: 1,
			interior: para_2000,
		};
		let buy_asset_location = MultiLocation {
			parents: 0,
			interior: buy_execution_asset,
		};
		let fees = MultiAsset {
			id: AssetId::Concrete(buy_asset_location),
			fun: Fungibility::Fungible(1000000000000_u128)
		};
		let reserved_location = MultiLocation {
			parents: 1,
			interior: reserved_asset,
		};


		let assets = MultiAssetFilter::Wild(WildMultiAsset::All);
		let mut multi_assets = MultiAssets::new();
		multi_assets.push(
			MultiAsset {
				id: AssetId::Concrete(reserved_location),
				fun: Fungibility::Fungible(1000000000000_u128)
			} 
		);

		let versioned_xcm = Box::new(VersionedXcm::from(Xcm(vec![
			WithdrawAsset(multi_assets),
			BuyExecution { fees, weight_limit: WeightLimit::Unlimited},
			Transact {
				origin_kind: OriginKind::Native,
				require_weight_at_most: 500000000000 as u64,
				call: call.into()
			}
		])));
		let destination = Box::new(VersionedMultiLocation::V3(dest));
		<pallet_xcm::Pallet<T>>::send(origin, destination, versioned_xcm ).unwrap();

	}
}