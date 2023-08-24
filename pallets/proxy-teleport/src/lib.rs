#![cfg_attr(not(feature = "std"), no_std)]

type BaseXcm<T> = pallet_xcm::Pallet<T>;
use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{Contains, EnsureOrigin, Get},
	weights::Weight,
};
use frame_system::pallet_prelude::OriginFor;
pub use pallet::*;
use parity_scale_codec::Encode;
use sp_std::{boxed::Box, vec};
use xcm::v3::WeightLimit::Unlimited;
pub use xcm::{
	opaque::latest::prelude::{Junction, Junctions, MultiLocation, OriginKind},
	v3::{
		AssetId, ExecuteXcm, Fungibility,
		Instruction::{
			BuyExecution, DepositAsset, DepositReserveAsset, InitiateReserveWithdraw,
			ReceiveTeleportedAsset, Transact, WithdrawAsset,
		},
		MultiAsset, MultiAssetFilter, MultiAssets, Parent, SendXcm, WeightLimit, WildMultiAsset,
		Xcm, XcmHash,
	},
	VersionedMultiAssets, VersionedMultiLocation, VersionedResponse, VersionedXcm,
};

// #[cfg(test)]
// mod mock;

// #[cfg(test)]
// mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;
// pub mod weights;
// pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config + pallet_xcm::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn something)]
	pub type Something<T> = StorageValue<_, u32>;

	#[pallet::error]
	pub enum Error<T> {
		/// The version of the `Versioned` value used is not able to be interpreted.
		BadVersion,
		/// Too many assets have been attempted for transfer.
		TooManyAssets,
		/// The message execution fails the filter.
		Filtered,
		/// The assets to be sent are empty.
		Empty,
		/// Could not re-anchor the assets to declare the fees for the destination chain.
		CannotReanchor,
		/// Origin is invalid for sending.
		InvalidOrigin,
		/// An error ocured during send
		SendError,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Execution of an XCM message was attempted.
		Attempted { outcome: xcm::latest::Outcome },
		/// A XCM message was sent.
		Sent {
			origin: MultiLocation,
			destination: MultiLocation,
			message: Xcm<()>,
			message_id: XcmHash,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::call_index(0)]
		#[pallet::weight(10_000)]
		pub fn proxy_teleport(
			origin: OriginFor<T>,
			dest: Box<VersionedMultiLocation>,
			beneficiary: Box<VersionedMultiLocation>,
			assets: Box<VersionedMultiAssets>,
			proxy_asset: Box<VersionedMultiAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			Self::do_proxy_teleport_assets(
				origin,
				dest,
				beneficiary,
				assets,
				proxy_asset,
				fee_asset_item,
				None,
			)
		}
	}
}

/// The maximum number of distinct assets allowed to be transferred in a single helper extrinsic.
const MAX_ASSETS_FOR_TRANSFER: usize = 2;

impl<T: Config> Pallet<T> {
	fn do_proxy_teleport_assets(
		origin: OriginFor<T>,
		dest: Box<VersionedMultiLocation>,
		beneficiary: Box<VersionedMultiLocation>,
		assets: Box<VersionedMultiAssets>,
		proxy_asset: Box<VersionedMultiAssets>,
		fee_asset_item: u32,
		maybe_weight_limit: Option<WeightLimit>,
	) -> DispatchResult {
		//Unbox parameters.
		let origin_location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		let dest: MultiLocation = (*dest).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let beneficiary: MultiLocation =
			(*beneficiary).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let assets: MultiAssets = (*assets).try_into().map_err(|()| Error::<T>::BadVersion)?;
		let proxy_asset: MultiAssets =
			(*proxy_asset).try_into().map_err(|()| Error::<T>::BadVersion)?;

		//Checks
		ensure!(assets.len() <= MAX_ASSETS_FOR_TRANSFER, Error::<T>::TooManyAssets);
		let value = (origin_location, assets.into_inner());
		ensure!(T::XcmTeleportFilter::contains(&value), Error::<T>::Filtered);
		let (origin_location, assets) = value;

		let context = T::UniversalLocation::get();

		// Will only work for caller fees. Proxy asset pays at destination.
		let fees = proxy_asset
			.get(fee_asset_item as usize)
			.ok_or(Error::<T>::Empty)?
			.clone()
			.reanchored(&dest, context)
			.map_err(|_| Error::<T>::CannotReanchor)?;
		let max_assets = assets.len() as u32;
		let assets: MultiAssets = assets.into();

		// For now, ignore weight limit.
		// TODO: Implement weight_limit calculation with final instructions.
		let weight_limit: WeightLimit = Unlimited;
		// let weight_limit = match maybe_weight_limit {
		// 	Some(weight_limit) => weight_limit,
		// 	None => {
		// 		let fees = fees.clone();
		// 		let mut remote_message = Xcm(vec![
		// 			ReceiveTeleportedAsset(assets.clone()),
		// 			ClearOrigin,
		// 			BuyExecution { fees, weight_limit: Limited(Weight::zero()) },
		// 			DepositAsset { assets: Wild(AllCounted(max_assets)), beneficiary },
		// 		]);
		// 		// use local weight for remote message and hope for the best.
		// 		let remote_weight = T::Weigher::weight(&mut remote_message)
		// 			.map_err(|()| Error::<T>::UnweighableMessage)?;
		// 		Limited(remote_weight)
		// 	},
		// };

		let foreign_location =
			MultiLocation { parents: 1, interior: Junctions::X1(Junction::Parachain(1836)) };

		let foreing_asset = MultiAsset {
			id: AssetId::Concrete(foreign_location),
			fun: Fungibility::Fungible(1000000000000_u128),
		};

        let foreing_assets = MultiAssets::from(vec![foreing_asset.clone()]);
		// Build the message to send.
		let xcm_message: Xcm<()> = Xcm(vec![
			WithdrawAsset(proxy_asset),
			BuyExecution { fees, weight_limit },
			ReceiveTeleportedAsset(foreing_assets),
			DepositAsset {
				assets: MultiAssetFilter::Wild(WildMultiAsset::AllCounted(max_assets)),
				beneficiary,
			},
		]);

		//Build the message to execute.
		let message: Xcm<<T as frame_system::Config>::RuntimeCall> = Xcm(vec![
			WithdrawAsset(assets),
			//TODO: Check if needed.
			// SetFeesMode { jit_withdraw: true },
		]);

		// Temporarly hardcode weight.
		// TODO: Replace for Weigher.
		let weight: Weight = Weight::from_parts(1_000_000_000, 100_000);
		// 	T::Weigher::weight(&mut message).map_err(|()| Error::<T>::UnweighableMessage)?;

		// Execute Withdraw for burning assets on origin.
		let hash = message.using_encoded(sp_io::hashing::blake2_256);
		let outcome =
			T::XcmExecutor::execute_xcm_in_credit(origin_location, message, hash, weight, weight);
		Self::deposit_event(Event::Attempted { outcome });

		// Use pallet-xcm send for sending message.
		let root_origin = T::SendXcmOrigin::ensure_origin(frame_system::RawOrigin::Root.into())?;
		let interior: Junctions = root_origin.try_into().map_err(|_| Error::<T>::InvalidOrigin)?;
		let message_id = BaseXcm::<T>::send_xcm(interior, dest, xcm_message.clone())
			.map_err(|_| Error::<T>::SendError)?;
		//TODO: Check this Error population and use the ones from pallet-xcm
		let e = Event::Sent {
			origin: origin_location,
			destination: dest,
			message: xcm_message,
			message_id,
		};
		Self::deposit_event(e);

		// Finish.
		Ok(())
	}
}
