
//! Autogenerated weights for `pallet_benchmarks`
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
// pallet_benchmarks
// --extrinsic
// *
// --steps
// 20
// --repeat
// 10
// --output
// runtime/trappist/src/weights/pallet_benchmarks.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_benchmarks`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_benchmarks::WeightInfo for WeightInfo<T> {
	// Storage: AssetRegistry AssetMultiLocationId (r:1 w:0)
	// Storage: Assets Asset (r:1 w:0)
	fn drop_assets_fungible() -> Weight {
		// Minimum execution time: 4_589 nanoseconds.
		Weight::from_ref_time(4_898_000)
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: AssetRegistry AssetMultiLocationId (r:1 w:0)
	fn drop_assets_native() -> Weight {
		// Minimum execution time: 2_157 nanoseconds.
		Weight::from_ref_time(2_314_000)
			.saturating_add(T::DbWeight::get().reads(1))
	}
	fn drop_assets_default() -> Weight {
		// Minimum execution time: 130 nanoseconds.
		Weight::from_ref_time(154_000)
	}
}
