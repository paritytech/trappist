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

// Disclaimer:
// This is an experimental implementation of a pallet-xcm extension.
// This module is not audited and should not be used in production.

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
pub use xcm::{
	latest::prelude::{
		AssetId, ExecuteXcm, Fungibility,
		Instruction::{
			BurnAsset, BuyExecution, DepositAsset, DepositReserveAsset, InitiateReserveWithdraw,
			ReceiveTeleportedAsset, Transact, WithdrawAsset,
		},
		Junction, Junctions, MultiAsset, MultiAssetFilter, MultiAssets, MultiLocation, OriginKind,
		Parent, SendXcm, WeightLimit,
		WeightLimit::Unlimited,
		WildMultiAsset, Xcm, XcmHash,
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
		/// Failed to execute
		FailedToExecute,
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

	/// Teleport native asset from a parachain to another.
	/// This function is called by the parachain that wants to teleport native assets to another
	/// parachain but needs to buy execution on the destination parachain with an asset that is not
	/// being teleported. We call this asset the proxy asset.
	/// The parachain that wants to teleport native assets to another parachain with this method
	/// need to fund its Sovereign Account with the proxy asset on the destination parachain.
	/// If multiple proxy assets are included in the message, only the first one is used to buy
	/// execution. Proxy assets are trapped on the destination parachain.
	/// Parameters:
	/// - `origin`: The origin of the call.
	/// - `dest`: The destination chain of the teleport.
	/// - `beneficiary`: The beneficiary of the teleport from the perspective of the destination
	///   chain.
	/// - `native_asset_amount`: The amount of native asset to teleport.
	/// - `proxy_asset`: The proxy asset to buy execution on the destination chain.

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
		) -> DispatchResult {
			Self::do_proxy_teleport_assets(
				origin,
				dest,
				beneficiary,
				native_asset_amount,
				proxy_asset,
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
	) -> DispatchResult {
		//Unbox origin, destination and beneficiary.
		let origin_location = T::ExecuteXcmOrigin::ensure_origin(origin)?;
		let dest: MultiLocation =
			(*dest).try_into().map_err(|()| pallet_xcm::Error::<T>::BadVersion)?;
		let beneficiary: MultiLocation =
			(*beneficiary).try_into().map_err(|()| pallet_xcm::Error::<T>::BadVersion)?;
		//Unbox proxy asset
		let proxy_asset: MultiAssets =
			(*proxy_asset).try_into().map_err(|()| pallet_xcm::Error::<T>::BadVersion)?;

		//Create assets

		// Native from local perspective
		let native_asset = MultiAsset {
			id: AssetId::Concrete(MultiLocation::here()),
			fun: Fungibility::Fungible(native_asset_amount),
		};
		let assets = MultiAssets::from(vec![native_asset.clone()]);

		// Native from foreign perspective
		let context = T::UniversalLocation::get();
		let native_as_foreign = native_asset
			.reanchored(&dest, context)
			.map_err(|_| pallet_xcm::Error::<T>::CannotReanchor)?;
		let foreing_assets = MultiAssets::from(vec![native_as_foreign]);

		// TeleportFilter check
		let value = (origin_location, assets.into_inner());
		ensure!(T::XcmTeleportFilter::contains(&value), pallet_xcm::Error::<T>::Filtered);
		let (origin_location, assets) = value;

		// Reanchor the proxy asset to the destination chain.
		let fee_asset_item: usize = 0;
		let fees = proxy_asset
			.get(fee_asset_item as usize)
			.ok_or(pallet_xcm::Error::<T>::Empty)?
			.clone()
			.reanchored(&dest, context)
			.map_err(|_| pallet_xcm::Error::<T>::CannotReanchor)?;

		// TODO: Define if Withdrawn proxy assets are deposited or trapped.
		// Check if there is no vulnerability through RefundSurplus
		// let max_assets = (assets.len() as u32).checked_add(1).ok_or(Error::<T>::TooManyAssets)?;

		// DISCLAIMER: Splitting the instructions to be executed on origin and destination is
		// discouraged. Due to current limitations, we need to generate a message
		// to be executed on origin and another message to be sent to be executed on destination in
		// two different steps as:
		// - We cannot buy execution on Asset Hub with foreign assets.
		// - We cannot send arbitrary instructions from a local XCM execution.
		// - InitiateTeleport prepends unwanted instructions to the message.
		// - Asset Hub does not recognize Sibling chains as trusted teleporters of ROC.

		//Build the message to execute on origin.
		let assets: MultiAssets = assets.into();
		let message: Xcm<<T as frame_system::Config>::RuntimeCall> =
			Xcm(vec![WithdrawAsset(assets.clone()), BurnAsset(assets)]);

		// Build the message to send to be executed.
		// Set WeightLimit
		// TODO: Implement weight_limit calculation with final instructions.
		let weight_limit: WeightLimit = Unlimited;
		let xcm_to_send: Xcm<()> = Xcm(vec![
			// There are currently no limitations on the amount of proxy assets to withdraw.
			// Since funds are withdrawn from the Sovereign Account of the origin, chains must be aware of this
			// and implement a mechanism to prevent draining.
			WithdrawAsset(proxy_asset),
			BuyExecution { fees, weight_limit },
			ReceiveTeleportedAsset(foreing_assets.clone()),
			// Intentionally trap ROC to avoid exploit of draining Sovereing Account
			// by depositing withdrawn ROC on beneficiary.
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
		outcome.clone().ensure_complete().map_err(|_| Error::<T>::FailedToExecute)?;
		Self::deposit_event(Event::Attempted { outcome });

		// Use pallet-xcm send for sending message.
		// Origin is set to Root so it is interpreted as Sovereign Account.
		let root_origin = T::SendXcmOrigin::ensure_origin(frame_system::RawOrigin::Root.into())?;
		let interior: Junctions =
			root_origin.try_into().map_err(|_| pallet_xcm::Error::<T>::InvalidOrigin)?;
		//TODO: Check this Error population
		let message_id = BaseXcm::<T>::send_xcm(interior, dest, xcm_to_send.clone())
			.map_err(|_| Error::<T>::SendError)?;
		let e = Event::Sent {
			origin: origin_location,
			destination: dest,
			message: xcm_to_send,
			message_id,
		};
		Self::deposit_event(e);

		// Finish.
		Ok(())
	}
}
