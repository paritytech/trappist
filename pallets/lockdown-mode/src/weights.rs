//! Autogenerated weights for `pallet_lockdown_mode`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `vale`, CPU: `11th Gen Intel(R) Core(TM) i7-1165G7 @ 2.80GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/trappist-collator
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_lockdown_mode
// --execution=wasm
// --wasm-execution=compiled
// --extrinsic
// *
// --steps
// 50
// --repeat
// 20
// --output
// pallets/lockdown-mode/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

pub trait WeightInfo {
	fn activate_lockdown_mode() -> Weight;
	fn deactivate_lockdown_mode() -> Weight;
}

/// Weight functions for `pallet_lockdown_mode`.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: LockdownMode LockdownModeStatus (r:1 w:1)
	/// Proof: LockdownMode LockdownModeStatus (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: XcmpQueue QueueSuspended (r:0 w:1)
	/// Proof Skipped: XcmpQueue QueueSuspended (max_values: Some(1), max_size: None, mode: Measured)
	fn activate_lockdown_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `1586`
		// Minimum execution time: 69_552_000 picoseconds.
		Weight::from_parts(75_364_000, 0)
			.saturating_add(Weight::from_parts(0, 1586))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: LockdownMode LockdownModeStatus (r:1 w:1)
	/// Proof: LockdownMode LockdownModeStatus (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: XcmpQueue QueueSuspended (r:0 w:1)
	/// Proof Skipped: XcmpQueue QueueSuspended (max_values: Some(1), max_size: None, mode: Measured)
	fn deactivate_lockdown_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `1586`
		// Minimum execution time: 42_162_000 picoseconds.
		Weight::from_parts(43_321_000, 0)
			.saturating_add(Weight::from_parts(0, 1586))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}