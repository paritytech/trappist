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

//! Autogenerated weights for `trappist_runtime_benchmarks`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-08, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `kalan-x1x`, CPU: `12th Gen Intel(R) Core(TM) i7-12800H`
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/trappist-collator
// benchmark
// pallet
// --chain
// dev
// --pallet
// trappist_runtime_benchmarks
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// runtime/trappist/src/weights/trappist_runtime_benchmarks.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `trappist_runtime_benchmarks`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> trappist_runtime_benchmarks::WeightInfo for WeightInfo<T> {
	// Storage: AssetRegistry AssetMultiLocationId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	fn drop_assets_fungible() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `131`
		//  Estimated: `7762`
		// Minimum execution time: 9_674_000 picoseconds.
		Weight::from_parts(9_953_000, 0)
			.saturating_add(Weight::from_parts(0, 7762))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	/// Storage: AssetRegistry AssetMultiLocationId (r:1 w:0)
	/// Proof: AssetRegistry AssetMultiLocationId (max_values: None, max_size: Some(622), added: 3097, mode: MaxEncodedLen)
	fn drop_assets_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `4087`
		// Minimum execution time: 5_313_000 picoseconds.
		Weight::from_parts(5_537_000, 0)
			.saturating_add(Weight::from_parts(0, 4087))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn drop_assets_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_539_000 picoseconds.
		Weight::from_parts(1_632_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
}