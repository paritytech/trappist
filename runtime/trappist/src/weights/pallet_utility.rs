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

//! Autogenerated weights for `pallet_utility`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 29.0.0
//! DATE: 2023-12-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_utility
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

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3997`
		// Minimum execution time: 4_717_000 picoseconds.
		Weight::from_parts(21_214_914, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			// Standard Error: 4_259
			.saturating_add(Weight::from_parts(5_603_835, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn as_derivative() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3997`
		// Minimum execution time: 10_006_000 picoseconds.
		Weight::from_parts(10_522_000, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3997`
		// Minimum execution time: 4_800_000 picoseconds.
		Weight::from_parts(19_175_889, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			// Standard Error: 3_990
			.saturating_add(Weight::from_parts(5_895_665, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	fn dispatch_as() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 7_262_000 picoseconds.
		Weight::from_parts(7_512_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: `TxPause::PausedCalls` (r:1 w:0)
	/// Proof: `TxPause::PausedCalls` (`max_values`: None, `max_size`: Some(532), added: 3007, mode: `MaxEncodedLen`)
	/// Storage: `SafeMode::EnteredUntil` (r:1 w:0)
	/// Proof: `SafeMode::EnteredUntil` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `80`
		//  Estimated: `3997`
		// Minimum execution time: 4_676_000 picoseconds.
		Weight::from_parts(16_042_876, 0)
			.saturating_add(Weight::from_parts(0, 3997))
			// Standard Error: 3_898
			.saturating_add(Weight::from_parts(5_624_346, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
}
