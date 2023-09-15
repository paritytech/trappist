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
use crate::{
	migration::{IsFinished, MigrationStep},
	types::*,
	weights::WeightInfo,
	Config, Pallet, LOG_TARGET,
};
use frame_support::{
	pallet_prelude::*, storage_alias, traits::Get, weights::Weight, DefaultNoBound,
};
use parity_scale_codec::MaxEncodedLen;
use sp_runtime::BoundedVec;
use sp_std::prelude::*;

pub(crate) mod old {
	use super::*;

	#[derive(Encode, Decode)]
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

	impl<DepositBalance, StringLimit: Get<u32>> From<ItemMetadata<DepositBalance, StringLimit>>
		for OldItemMetadata<DepositBalance, StringLimit>
	{
		fn from(value: ItemMetadata<DepositBalance, StringLimit>) -> Self {
			Self { deposit: value.deposit, data: value.data, is_frozen: value.is_frozen }
		}
	}

	#[storage_alias]
	// Use storage_prefix name
	pub(crate) type InstanceMetadataOf<T: Config<I>, I: 'static> = StorageDoubleMap<
		Pallet<T, I>,
		Blake2_128Concat,
		<T as Config<I>>::CollectionId,
		Blake2_128Concat,
		<T as Config<I>>::ItemId,
		OldItemMetadata<DepositBalanceOf<T, I>, <T as Config<I>>::StringLimit>,
		OptionQuery,
	>;
}

#[cfg(feature = "runtime-benchmarks")]
pub fn store_old_metadata<T: Config<I>, I: 'static>(
	collection_id: <T as Config<I>>::CollectionId,
	item_id: <T as Config<I>>::ItemId,
	metadata: old::OldItemMetadata<DepositBalanceOf<T, I>, <T as Config<I>>::StringLimit>,
) {
	old::InstanceMetadataOf::<T, I>::insert(collection_id, item_id, metadata);
}

#[derive(Encode, Decode, MaxEncodedLen, DefaultNoBound)]
pub struct Migration<T: Config<I>, I: 'static = ()> {
	last_metadata: Option<(T::CollectionId, T::ItemId)>,
}

impl<T: Config<I>, I: 'static> MigrationStep for Migration<T, I> {
	const VERSION: u16 = 1;

	fn max_step_weight() -> Weight {
		T::WeightInfo::v1_migration_step()
	}

	fn step(&mut self) -> (IsFinished, Weight) {
		let mut iter = if let Some(last_metadata) = self.last_metadata.take() {
			old::InstanceMetadataOf::<T, I>::iter_from(
				old::InstanceMetadataOf::<T, I>::hashed_key_for(last_metadata.0, last_metadata.1),
			)
		} else {
			old::InstanceMetadataOf::<T, I>::iter()
		};

		if let Some((collection_item, item_id, old)) = iter.next() {
			log::debug!(target: LOG_TARGET, "Migrating item {:?} from collection {:?}", item_id, collection_item);
			let metadata = ItemMetadata::<DepositBalanceOf<T, I>, T::StringLimit> {
				deposit: old.deposit,
				data: old.data,
				is_frozen: old.is_frozen,
				name: BoundedVec::truncate_from(b"Polkadot Deep Dive".to_vec()),
			};
			crate::ItemMetadataOf::<T, I>::insert(
				collection_item.clone(),
				item_id.clone(),
				metadata,
			);
			self.last_metadata = Some((collection_item, item_id));
			(IsFinished::No, T::WeightInfo::v1_migration_step())
		} else {
			log::debug!(target: LOG_TARGET, "No more metadata for items to migrate");
			(IsFinished::Yes, T::WeightInfo::v1_migration_step())
		}
	}
}
