
//! Autogenerated weights for `pallet_collective`
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
// --pallet=pallet_collective
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --output=./runtime/trappist/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_collective`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_collective::WeightInfo for WeightInfo<T> {
	// Storage: Council Members (r:1 w:1)
	// Storage: Council Proposals (r:1 w:0)
	// Storage: Council Prime (r:0 w:1)
	// Storage: Council Voting (r:100 w:100)
	/// The range of component `m` is `[0, 100]`.
	/// The range of component `n` is `[0, 100]`.
	/// The range of component `p` is `[0, 100]`.
	fn set_members(m: u32, _n: u32, p: u32, ) -> Weight {
		// Minimum execution time: 18_127 nanoseconds.
		Weight::from_ref_time(18_564_000)
			// Standard Error: 70_693
			.saturating_add(Weight::from_ref_time(5_011_151).saturating_mul(m.into()))
			// Standard Error: 70_693
			.saturating_add(Weight::from_ref_time(8_579_773).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 24_805 nanoseconds.
		Weight::from_ref_time(26_740_022)
			// Standard Error: 215
			.saturating_add(Weight::from_ref_time(804).saturating_mul(b.into()))
			// Standard Error: 2_216
			.saturating_add(Weight::from_ref_time(6_013).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:0)
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[1, 100]`.
	fn propose_execute(b: u32, m: u32, ) -> Weight {
		// Minimum execution time: 27_531 nanoseconds.
		Weight::from_ref_time(27_089_344)
			// Standard Error: 217
			.saturating_add(Weight::from_ref_time(1_740).saturating_mul(b.into()))
			// Standard Error: 2_246
			.saturating_add(Weight::from_ref_time(29_487).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalCount (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[2, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn propose_proposed(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 30_371 nanoseconds.
		Weight::from_ref_time(22_529_545)
			// Standard Error: 625
			.saturating_add(Weight::from_ref_time(9_161).saturating_mul(b.into()))
			// Standard Error: 6_524
			.saturating_add(Weight::from_ref_time(31_294).saturating_mul(m.into()))
			// Standard Error: 6_441
			.saturating_add(Weight::from_ref_time(326_380).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Voting (r:1 w:1)
	/// The range of component `m` is `[5, 100]`.
	fn vote(m: u32, ) -> Weight {
		// Minimum execution time: 36_809 nanoseconds.
		Weight::from_ref_time(45_585_856)
			// Standard Error: 8_909
			.saturating_add(Weight::from_ref_time(53_758).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 32_907 nanoseconds.
		Weight::from_ref_time(28_925_136)
			// Standard Error: 5_615
			.saturating_add(Weight::from_ref_time(71_454).saturating_mul(m.into()))
			// Standard Error: 5_475
			.saturating_add(Weight::from_ref_time(287_811).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_early_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 47_055 nanoseconds.
		Weight::from_ref_time(40_613_276)
			// Standard Error: 823
			.saturating_add(Weight::from_ref_time(6_077).saturating_mul(b.into()))
			// Standard Error: 8_703
			.saturating_add(Weight::from_ref_time(69_733).saturating_mul(m.into()))
			// Standard Error: 8_483
			.saturating_add(Weight::from_ref_time(367_760).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_disapproved(m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 35_624 nanoseconds.
		Weight::from_ref_time(34_780_020)
			// Standard Error: 5_680
			.saturating_add(Weight::from_ref_time(44_662).saturating_mul(m.into()))
			// Standard Error: 5_539
			.saturating_add(Weight::from_ref_time(274_728).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Voting (r:1 w:1)
	// Storage: Council Members (r:1 w:0)
	// Storage: Council Prime (r:1 w:0)
	// Storage: Council ProposalOf (r:1 w:1)
	// Storage: LockdownMode LockdownModeStatus (r:1 w:0)
	// Storage: Council Proposals (r:1 w:1)
	/// The range of component `b` is `[2, 1024]`.
	/// The range of component `m` is `[4, 100]`.
	/// The range of component `p` is `[1, 100]`.
	fn close_approved(b: u32, m: u32, p: u32, ) -> Weight {
		// Minimum execution time: 50_132 nanoseconds.
		Weight::from_ref_time(42_159_292)
			// Standard Error: 826
			.saturating_add(Weight::from_ref_time(7_410).saturating_mul(b.into()))
			// Standard Error: 8_738
			.saturating_add(Weight::from_ref_time(60_065).saturating_mul(m.into()))
			// Standard Error: 8_518
			.saturating_add(Weight::from_ref_time(366_476).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: Council Proposals (r:1 w:1)
	// Storage: Council Voting (r:0 w:1)
	// Storage: Council ProposalOf (r:0 w:1)
	/// The range of component `p` is `[1, 100]`.
	fn disapprove_proposal(p: u32, ) -> Weight {
		// Minimum execution time: 21_017 nanoseconds.
		Weight::from_ref_time(24_459_889)
			// Standard Error: 4_687
			.saturating_add(Weight::from_ref_time(213_240).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(3))
	}
}
