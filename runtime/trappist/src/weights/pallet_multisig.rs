
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-24, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner-sgmchhtv-project-647-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/trappist-collator
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --pallet=pallet_multisig
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./runtime/trappist/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Minimum execution time: 20_844 nanoseconds.
		Weight::from_ref_time(22_212_196)
			// Standard Error: 8
			.saturating_add(Weight::from_ref_time(645).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 50_261 nanoseconds.
		Weight::from_ref_time(36_607_611)
			// Standard Error: 1_287
			.saturating_add(Weight::from_ref_time(157_811).saturating_mul(s.into()))
			// Standard Error: 12
			.saturating_add(Weight::from_ref_time(1_885).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 38_545 nanoseconds.
		Weight::from_ref_time(26_306_532)
			// Standard Error: 941
			.saturating_add(Weight::from_ref_time(140_661).saturating_mul(s.into()))
			// Standard Error: 9
			.saturating_add(Weight::from_ref_time(1_886).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Minimum execution time: 59_315 nanoseconds.
		Weight::from_ref_time(42_448_959)
			// Standard Error: 1_255
			.saturating_add(Weight::from_ref_time(198_696).saturating_mul(s.into()))
			// Standard Error: 12
			.saturating_add(Weight::from_ref_time(2_005).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	// Storage: unknown [0x3a65787472696e7369635f696e646578] (r:1 w:0)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Minimum execution time: 34_126 nanoseconds.
		Weight::from_ref_time(34_982_646)
			// Standard Error: 1_119
			.saturating_add(Weight::from_ref_time(159_055).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Minimum execution time: 24_353 nanoseconds.
		Weight::from_ref_time(24_961_276)
			// Standard Error: 1_035
			.saturating_add(Weight::from_ref_time(137_965).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Multisig Multisigs (r:1 w:1)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Minimum execution time: 35_413 nanoseconds.
		Weight::from_ref_time(36_582_377)
			// Standard Error: 1_154
			.saturating_add(Weight::from_ref_time(154_343).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
