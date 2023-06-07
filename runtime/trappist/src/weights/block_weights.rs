//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-08 (Y/M/D)
//! HOSTNAME: `kalan-x1x`, CPU: `12th Gen Intel(R) Core(TM) i7-12800H`
//!
//! SHORT-NAME: `block`, LONG-NAME: `BlockExecution`, RUNTIME: `Trappist Development`
//! WARMUPS: `10`, REPEAT: `100`
//! WEIGHT-PATH: `./runtime/trappist/src/weights`
//! WEIGHT-METRIC: `Average`, WEIGHT-MUL: `1.0`, WEIGHT-ADD: `0`

// Executed Command:
//   target/debug/trappist-node
//   benchmark
//   overhead
//   --chain
//   dev
//   --weight-path=./runtime/trappist/src/weights

use sp_core::parameter_types;
use sp_weights::{constants::WEIGHT_REF_TIME_PER_NANOS, Weight};

parameter_types! {
	/// Time to execute an empty block.
	/// Calculated by multiplying the *Average* with `1.0` and adding `0`.
	///
	/// Stats nanoseconds:
	///   Min, Max: 5_120_602, 10_113_574
	///   Average:  6_901_058
	///   Median:   6_276_001
	///   Std-Dev:  1393716.66
	///
	/// Percentiles nanoseconds:
	///   99th: 9_958_901
	///   95th: 8_509_417
	///   75th: 8_216_945
	pub const BlockExecutionWeight: Weight =
		Weight::from_ref_time(WEIGHT_REF_TIME_PER_NANOS.saturating_mul(6_901_058));
}

#[cfg(test)]
mod test_weights {
	use sp_weights::constants;

	/// Checks that the weight exists and is sane.
	// NOTE: If this test fails but you are sure that the generated values are fine,
	// you can delete it.
	#[test]
	fn sane() {
		let w = super::BlockExecutionWeight::get();

		// At least 100 µs.
		assert!(
			w.ref_time() >= 100u64 * constants::WEIGHT_REF_TIME_PER_MICROS,
			"Weight should be at least 100 µs."
		);
		// At most 50 ms.
		assert!(
			w.ref_time() <= 50u64 * constants::WEIGHT_REF_TIME_PER_MILLIS,
			"Weight should be at most 50 ms."
		);
	}
}
