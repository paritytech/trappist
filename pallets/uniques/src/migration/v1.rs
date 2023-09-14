// This file is part of Substrate.

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

//! Various pieces of common functionality.
use super::*;
use crate::*;
use frame_support::{
	traits::{Get, GetStorageVersion, OnRuntimeUpgrade, PalletInfoAccess, StorageVersion},
	weights::Weight,
};
use parity_scale_codec::{Codec, MaxEncodedLen};
use sp_runtime::BoundedVec;

/// Migrate the pallet storage to v1.
pub fn migrate_to_v1<T: Config<I>, I: 'static, P: GetStorageVersion + PalletInfoAccess>(
) -> frame_support::weights::Weight {
	let on_chain_storage_version = <P as GetStorageVersion>::on_chain_storage_version();
	log::info!(
		target: LOG_TARGET,
		"Running migration storage v1 for uniques with storage version {:?}",
		on_chain_storage_version,
	);

	if on_chain_storage_version < 1 {
		let mut count = 0;
		for (collection, detail) in Collection::<T, I>::iter() {
			CollectionAccount::<T, I>::insert(&detail.owner, &collection, ());
			count += 1;
		}
		StorageVersion::new(1).put::<P>();
		log::info!(
			target: LOG_TARGET,
			"Running migration storage v1 for uniques with storage version {:?} was complete",
			on_chain_storage_version,
		);
		// calculate and return migration weights
		T::DbWeight::get().reads_writes(count as u64 + 1, count as u64 + 1)
	} else {
		log::warn!(
			target: LOG_TARGET,
			"Attempted to apply migration to v1 but failed because storage version is {:?}",
			on_chain_storage_version,
		);
		T::DbWeight::get().reads(1)
	}
}

#[derive(Decode)]
pub struct OldItemMetadata<DepositBalance, StringLimit: Get<u32>> {
	/// The balance deposited for this metadata.
	///
	/// This pays for the data stored in this struct.
	pub(super) deposit: DepositBalance,
	/// General information concerning this item. Limited in length by `StringLimit`. This will
	/// generally be either a JSON dump or the hash of some JSON which can be found on a
	/// hash-addressable global publication system such as IPFS.
	pub(super) data: BoundedVec<u8, StringLimit>,
	/// Whether the item metadata may be changed by a non Force origin.
	pub(super) is_frozen: bool,
}

pub mod v1 {
	use super::*;

	pub struct MigrateToV1<T>(sp_std::marker::PhantomData<T>);
	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		fn on_runtime_upgrade() -> Weight {
			if StorageVersion::get::<Pallet<T>>() == 0 {
				let mut weight = T::DbWeight::get().reads(1);

				let mut translated = 0u64;

				ItemMetadataOf::<T>::translate::<
					OldItemMetadata<DepositBalanceOf<T>, T::StringLimit>,
					_,
				>(|_key, _item_id, old_value| {
					translated.saturating_inc();
					let new_value = ItemMetadata::<DepositBalanceOf<T>, T::StringLimit> {
						deposit: old_value.deposit,
						data: old_value.data,
						is_frozen: old_value.is_frozen,
						name: BoundedVec::default(),
					};

					Some(new_value)
				});

				log::info!("v1 applied successfully");
				StorageVersion::new(1).put::<Pallet<T>>();

				T::DbWeight::get().reads_writes(translated + 1, translated + 1)
			} else {
				log::warn!("skipping v1, should be removed");
				T::DbWeight::get().reads(1)
			}
		}
	}
}
