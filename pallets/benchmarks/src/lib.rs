//! Pallet for benchmarking.

#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use frame_support::{pallet_prelude::*, traits::tokens::AssetId};
use sp_runtime::traits::AtLeast32BitUnsigned;
use xcm::prelude::*;
use xcm_executor::traits::DropAssets;

pub use pallet::*;
pub use weights::*;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;
pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	use super::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Identifier for the class of asset.
		type AssetId: AssetId + From<u32>;

		/// The balance of an account.
		type Balance: Parameter + Member + AtLeast32BitUnsigned + Codec + TypeInfo;

		/// The minimum amount required to keep an account open.
		#[pallet::constant]
		type ExistentialDeposit: Get<Self::Balance>;

		/// Handler for when some non-empty `Assets` value should be dropped.
		type DropAssets: DropAssets;

		/// Handler to register an asset.
		fn register_asset(asset_id: Self::AssetId, location: MultiLocation);
	}

	#[pallet::pallet]
	pub struct Pallet<T>(_);
}
