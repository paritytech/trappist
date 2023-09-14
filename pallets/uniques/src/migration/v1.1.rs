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

mod old {
	use super::*;

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

	#[storage_alias]
	pub(super) type ItemMetadataOf<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
		_,
		Blake2_128Concat,
		T::CollectionId,
		Blake2_128Concat,
		T::ItemId,
		ItemMetadata<DepositBalanceOf<T, I>, T::StringLimit>,
		OptionQuery,
	>;
}

#[cfg(feature = "runtime-benchmarks")]
pub fn store_old_metadata<T: Config>(
	collection_id: T::CollectionId,
	item_id: T::ItemId,
	metadata: crate::ItemMetadata<DepositBalanceOf<T, I>, T::StringLimit>,
) {
	let info = old::OldItemMetadata {
		deposit: metadata.deposit.clone(),
		data: metadata.data,
		is_frozen: metadata.data,
	};
	old::ItemMetadataOf::<T>::insert(collection_id, item_id, metadata);
}

#[storage_alias]
pub(super) type ItemMetadataOf<T: Config<I>, I: 'static = ()> = StorageDoubleMap<
	_,
	Blake2_128Concat,
	T::CollectionId,
	Blake2_128Concat,
	T::ItemId,
	ItemMetadata<DepositBalanceOf<T, I>, T::StringLimit>,
	OptionQuery,
>;

#[derive(Encode, Decode, MaxEncodedLen, DefaultNoBound)]
pub struct Migration<T: Config> {
	last_metadata: Option<(T::CollectionId, T::ItemId)>,
}

impl<T: Config> MigrationStep for Migration<T> {
	const VERSION: u16 = 13;

	fn max_step_weight() -> Weight {
		T::WeightInfo::v1_migration_step()
	}

	fn step(&mut self) -> (IsFinished, Weight) {
		let mut iter = if let Some(last_metadata) = self.last_metadata.take() {
			old::ItemMetadataOf::<T>::iter_from(old::ItemMetadataOf::<T>::hashed_key_for(
				last_account.0,
				last_account.1,
			))
		} else {
			old::ItemMetadataOf::<T>::iter()
		};

		if let Some((collection_item, item_id, old)) = iter.next() {
			log::debug!(target: LOG_TARGET, "Migrating item {:?} from collection {:?}", item_id, collection_item);
			let metadata = ItemMetadata::<DepositBalanceOf<T>, T::StringLimit> {
				deposit: old.deposit,
				data: old.data,
				is_frozen: old.is_frozen,
				name: BoundedVec::default(),
			};
			ItemMetadataOf::<T>::insert(collection_item.clone(), item_id.clone(), metadata);
			self.last_metadata = Some((collection_item, item_id));
			(IsFinished::No, T::WeightInfo::v1_migration_step())
		} else {
			log::debug!(target: LOG_TARGET, "No more metadata for items to migrate");
			(IsFinished::Yes, T::WeightInfo::v1_migration_step())
		}
	}
}
