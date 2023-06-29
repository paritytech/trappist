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
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner--ss9ysm1-project-647-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("trappist-dev"), DB CACHE: 1024

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
// --execution=wasm
// --wasm-execution=compiled
// --header=./templates/file_header.txt
// --output=./runtime/trappist/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_utility`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_utility::WeightInfo for WeightInfo<T> {
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `c` is `[0, 1000]`.
	fn batch(c: u32, ) -> Weight {
		// Minimum execution time: 14_184 nanoseconds.
		Weight::from_ref_time(19_921_947)
			// Standard Error: 4_638
			.saturating_add(Weight::from_ref_time(5_322_229).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	fn as_derivative() -> Weight {
		// Minimum execution time: 9_510 nanoseconds.
		Weight::from_ref_time(9_788_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `c` is `[0, 1000]`.
	fn batch_all(c: u32, ) -> Weight {
		// Minimum execution time: 13_965 nanoseconds.
		Weight::from_ref_time(27_504_133)
			// Standard Error: 2_740
			.saturating_add(Weight::from_ref_time(5_479_324).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn dispatch_as() -> Weight {
		// Minimum execution time: 16_231 nanoseconds.
		Weight::from_ref_time(16_856_000)
	}
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `c` is `[0, 1000]`.
	fn force_batch(c: u32, ) -> Weight {
		// Minimum execution time: 13_921 nanoseconds.
		Weight::from_ref_time(19_411_547)
			// Standard Error: 1_722
			.saturating_add(Weight::from_ref_time(5_317_525).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
}