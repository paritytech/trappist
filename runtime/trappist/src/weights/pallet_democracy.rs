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

//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 29.0.0
//! DATE: 2023-12-21, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-q7z7ruxr-project-647-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("trappist-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/trappist-node
// benchmark
// pallet
// --chain=trappist-dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_democracy
// --extrinsic=*
// --wasm-execution=compiled
// --header=./templates/file_header.txt
// --output=./runtime/trappist/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: `Democracy::PublicPropCount` (r:1 w:1)
	/// Proof: `Democracy::PublicPropCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:0 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4768`
		//  Estimated: `18187`
		// Minimum execution time: 40_972_000 picoseconds.
		Weight::from_parts(42_138_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3523`
		//  Estimated: `6695`
		// Minimum execution time: 35_712_000 picoseconds.
		Weight::from_parts(37_434_000, 0)
			.saturating_add(Weight::from_parts(0, 6695))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3399`
		//  Estimated: `7260`
		// Minimum execution time: 51_250_000 picoseconds.
		Weight::from_parts(52_541_000, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3421`
		//  Estimated: `7260`
		// Minimum execution time: 55_239_000 picoseconds.
		Weight::from_parts(56_536_000, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Cancellations` (r:1 w:1)
	/// Proof: `Democracy::Cancellations` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `333`
		//  Estimated: `3666`
		// Minimum execution time: 22_455_000 picoseconds.
		Weight::from_parts(22_815_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:3 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:0 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6216`
		//  Estimated: `18187`
		// Minimum execution time: 100_501_000 picoseconds.
		Weight::from_parts(103_091_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:0)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3383`
		//  Estimated: `6703`
		// Minimum execution time: 11_810_000 picoseconds.
		Weight::from_parts(12_287_000, 0)
			.saturating_add(Weight::from_parts(0, 6703))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_395_000 picoseconds.
		Weight::from_parts(2_652_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:0 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_480_000 picoseconds.
		Weight::from_parts(2_629_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:1)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:2)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `3518`
		// Minimum execution time: 22_330_000 picoseconds.
		Weight::from_parts(22_994_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:1)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::Blacklist` (r:1 w:1)
	/// Proof: `Democracy::Blacklist` (`max_values`: None, `max_size`: Some(3238), added: 5713, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3486`
		//  Estimated: `6703`
		// Minimum execution time: 25_117_000 picoseconds.
		Weight::from_parts(25_933_000, 0)
			.saturating_add(Weight::from_parts(0, 6703))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::DepositOf` (r:1 w:1)
	/// Proof: `Democracy::DepositOf` (`max_values`: None, `max_size`: Some(3230), added: 5705, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6127`
		//  Estimated: `18187`
		// Minimum execution time: 83_653_000 picoseconds.
		Weight::from_parts(85_540_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:0 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `3518`
		// Minimum execution time: 16_551_000 picoseconds.
		Weight::from_parts(16_969_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + r * (86 ±0)`
		//  Estimated: `1489 + r * (2676 ±0)`
		// Minimum execution time: 5_760_000 picoseconds.
		Weight::from_parts(6_548_332, 0)
			.saturating_add(Weight::from_parts(0, 1489))
			// Standard Error: 6_429
			.saturating_add(Weight::from_parts(3_113_108, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::LowestUnbaked` (r:1 w:1)
	/// Proof: `Democracy::LowestUnbaked` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumCount` (r:1 w:0)
	/// Proof: `Democracy::ReferendumCount` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::LastTabledWasExternal` (r:1 w:0)
	/// Proof: `Democracy::LastTabledWasExternal` (`max_values`: Some(1), `max_size`: Some(1), added: 496, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + r * (86 ±0)`
		//  Estimated: `18187 + r * (2676 ±0)`
		// Minimum execution time: 7_755_000 picoseconds.
		Weight::from_parts(8_289_081, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			// Standard Error: 7_137
			.saturating_add(Weight::from_parts(3_150_826, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:3 w:3)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `763 + r * (108 ±0)`
		//  Estimated: `19800 + r * (2676 ±0)`
		// Minimum execution time: 37_621_000 picoseconds.
		Weight::from_parts(44_024_497, 0)
			.saturating_add(Weight::from_parts(0, 19800))
			// Standard Error: 7_418
			.saturating_add(Weight::from_parts(3_985_526, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::VotingOf` (r:2 w:2)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::ReferendumInfoOf` (r:99 w:99)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `460 + r * (108 ±0)`
		//  Estimated: `13530 + r * (2676 ±0)`
		// Minimum execution time: 17_673_000 picoseconds.
		Weight::from_parts(18_961_817, 0)
			.saturating_add(Weight::from_parts(0, 13530))
			// Standard Error: 7_027
			.saturating_add(Weight::from_parts(3_954_685, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2676).saturating_mul(r.into()))
	}
	/// Storage: `Democracy::PublicProps` (r:0 w:1)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_523_000 picoseconds.
		Weight::from_parts(2_647_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `455`
		//  Estimated: `7260`
		// Minimum execution time: 21_890_000 picoseconds.
		Weight::from_parts(34_973_026, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 3_336
			.saturating_add(Weight::from_parts(107_477, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Locks` (r:1 w:1)
	/// Proof: `Balances::Locks` (`max_values`: None, `max_size`: Some(1299), added: 3774, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Freezes` (r:1 w:0)
	/// Proof: `Balances::Freezes` (`max_values`: None, `max_size`: Some(49), added: 2524, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456 + r * (22 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 30_723_000 picoseconds.
		Weight::from_parts(34_125_910, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 1_969
			.saturating_add(Weight::from_parts(123_637, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `695 + r * (26 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 15_031_000 picoseconds.
		Weight::from_parts(18_291_876, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 1_713
			.saturating_add(Weight::from_parts(121_953, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:1)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::VotingOf` (r:1 w:1)
	/// Proof: `Democracy::VotingOf` (`max_values`: None, `max_size`: Some(3795), added: 6270, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `695 + r * (26 ±0)`
		//  Estimated: `7260`
		// Minimum execution time: 15_353_000 picoseconds.
		Weight::from_parts(18_559_799, 0)
			.saturating_add(Weight::from_parts(0, 7260))
			// Standard Error: 1_795
			.saturating_add(Weight::from_parts(120_727, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `323`
		//  Estimated: `3556`
		// Minimum execution time: 17_357_000 picoseconds.
		Weight::from_parts(17_738_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::NextExternal` (r:1 w:0)
	/// Proof: `Democracy::NextExternal` (`max_values`: Some(1), `max_size`: Some(132), added: 627, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `3518`
		// Minimum execution time: 14_238_000 picoseconds.
		Weight::from_parts(14_728_000, 0)
			.saturating_add(Weight::from_parts(0, 3518))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4855`
		//  Estimated: `18187`
		// Minimum execution time: 39_685_000 picoseconds.
		Weight::from_parts(40_500_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::PublicProps` (r:1 w:0)
	/// Proof: `Democracy::PublicProps` (`max_values`: Some(1), `max_size`: Some(16702), added: 17197, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4789`
		//  Estimated: `18187`
		// Minimum execution time: 35_795_000 picoseconds.
		Weight::from_parts(36_564_000, 0)
			.saturating_add(Weight::from_parts(0, 18187))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Preimage::StatusFor` (r:1 w:0)
	/// Proof: `Preimage::StatusFor` (`max_values`: None, `max_size`: Some(91), added: 2566, mode: `MaxEncodedLen`)
	/// Storage: `Preimage::RequestStatusFor` (r:1 w:0)
	/// Proof: `Preimage::RequestStatusFor` (`max_values`: None, `max_size`: Some(75), added: 2550, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:0 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `3556`
		// Minimum execution time: 13_951_000 picoseconds.
		Weight::from_parts(14_402_000, 0)
			.saturating_add(Weight::from_parts(0, 3556))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Democracy::ReferendumInfoOf` (r:1 w:0)
	/// Proof: `Democracy::ReferendumInfoOf` (`max_values`: None, `max_size`: Some(201), added: 2676, mode: `MaxEncodedLen`)
	/// Storage: `Democracy::MetadataOf` (r:1 w:1)
	/// Proof: `Democracy::MetadataOf` (`max_values`: None, `max_size`: Some(53), added: 2528, mode: `MaxEncodedLen`)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `3666`
		// Minimum execution time: 16_635_000 picoseconds.
		Weight::from_parts(16_960_000, 0)
			.saturating_add(Weight::from_parts(0, 3666))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
