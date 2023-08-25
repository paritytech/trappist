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

	#[pallet::error]
	pub enum Error<T> {
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
		pub fn proxy_native_teleport(
			origin: OriginFor<T>,
			dest: Box<VersionedMultiLocation>,
			beneficiary: Box<VersionedMultiLocation>,
			native_asset_amount: u128,
			proxy_asset: Box<VersionedMultiAssets>,
			fee_asset_item: u32,
		) -> DispatchResult {
			Self::do_proxy_teleport_assets(
				origin,
				dest,
				beneficiary,
				native_asset_amount,
				proxy_asset,
				fee_asset_item,
			)
		}
	}
}

impl<T: Config> Pallet<T> {
	fn do_proxy_teleport_assets(
		origin: OriginFor<T>,
		dest: Box<VersionedMultiLocation>,
		beneficiary: Box<VersionedMultiLocation>,
		native_asset_amount: u128,
		proxy_asset: Box<VersionedMultiAssets>,
		fee_asset_item: u32,
	) -> DispatchResult {
		//Unbox origin, destination and beneficiary.
		let origin_location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		let dest: MultiLocation =
			(*dest).try_into().map_err(|()| pallet_xcm::Error::<T>::BadVersion)?;
		let beneficiary: MultiLocation =
			(*beneficiary).try_into().map_err(|()| pallet_xcm::Error::<T>::BadVersion)?;

		//Create assets

		// Native from local perspective
		let native_asset = MultiAsset {
			id: AssetId::Concrete(MultiLocation::here()),
			fun: Fungibility::Fungible(native_asset_amount),
		};

		let assets = MultiAssets::from(vec![native_asset]);

		// Native from foreign perspective
		//TODO: Replace ID with parameter
		let localtion_as_foreign =
			MultiLocation { parents: 1, interior: Junctions::X1(Junction::Parachain(1836)) };

		let native_as_foreign = MultiAsset {
			id: AssetId::Concrete(localtion_as_foreign),
			fun: Fungibility::Fungible(native_asset_amount),
		};

		let foreing_assets = MultiAssets::from(vec![native_as_foreign]);

		//Unbox proxy asset
		let proxy_asset: MultiAssets =
			(*proxy_asset).try_into().map_err(|()| pallet_xcm::Error::<T>::BadVersion)?;

		//TeleportFilter check
		let value = (origin_location, assets.into_inner());
		ensure!(T::XcmTeleportFilter::contains(&value), pallet_xcm::Error::<T>::Filtered);
		let (origin_location, assets) = value;

		// Reanchor the proxy asset to the destination chain.
		let context = T::UniversalLocation::get();
		let fees = proxy_asset
			.get(fee_asset_item as usize)
			.ok_or(pallet_xcm::Error::<T>::Empty)?
			.clone()
			.reanchored(&dest, context)
			.map_err(|_| pallet_xcm::Error::<T>::CannotReanchor)?;

		// TODO: Define if Withdrawn proxy assets are deposited or trapped. 
		// Check if there is no vulnerability through RefundSurplus
		//let max_assets = (assets.len() as u32).checked_add(1).ok_or(Error::<T>::TooManyAssets)?;

		//Build the message to execute on origin.
		let assets: MultiAssets = assets.into();
		let message: Xcm<<T as frame_system::Config>::RuntimeCall> = Xcm(vec![
			// Withdraw drops asset so is used as burn mechanism
			WithdrawAsset(assets),
		]);

		// Build the message to send.
		// Set WeightLimit
		// TODO: Implement weight_limit calculation with final instructions.
		let weight_limit: WeightLimit = Unlimited;
		let xcm_message: Xcm<()> = Xcm(vec![
			WithdrawAsset(proxy_asset),
			BuyExecution { fees, weight_limit },
			ReceiveTeleportedAsset(foreing_assets.clone()),
			// Intentionally trap ROC to avoid exploit
			DepositAsset { assets: MultiAssetFilter::Definite(foreing_assets), beneficiary },
		]);

		// Temporarly hardcode weight.
		// TODO: Replace for Weigher.
		let weight: Weight = Weight::from_parts(1_000_000_000, 100_000);
		// 	T::Weigher::weight(&mut message).map_err(|()| Error::<T>::UnweighableMessage)?;

		// Execute Withdraw for trapping assets on origin.
		let hash = message.using_encoded(sp_io::hashing::blake2_256);
		let outcome =
			T::XcmExecutor::execute_xcm_in_credit(origin_location, message, hash, weight, weight);
		Self::deposit_event(Event::Attempted { outcome });

		// Use pallet-xcm send for sending message.
		let root_origin = T::SendXcmOrigin::ensure_origin(frame_system::RawOrigin::Root.into())?;
		let interior: Junctions =
			root_origin.try_into().map_err(|_| pallet_xcm::Error::<T>::InvalidOrigin)?;
		//TODO: Check this Error population
		let message_id = BaseXcm::<T>::send_xcm(interior, dest, xcm_message.clone())
			.map_err(|_| Error::<T>::SendError)?;
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
