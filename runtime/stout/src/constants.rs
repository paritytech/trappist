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

/// Consensus-related.
pub mod consensus {
	/// Maximum number of blocks simultaneously accepted by the Runtime, not yet included
	/// into the relay chain.
	pub const UNINCLUDED_SEGMENT_CAPACITY: u32 = 1;
	/// How many parachain blocks are processed by the relay chain per parent. Limits the
	/// number of blocks authored per slot.
	pub const BLOCK_PROCESSING_VELOCITY: u32 = 1;
	/// Relay chain slot duration, in milliseconds.
	pub const RELAY_CHAIN_SLOT_DURATION_MILLIS: u32 = 6000;
}

pub mod currency {
	use kusama_runtime_constants as constants;
	use polkadot_core_primitives::Balance;

	/// The existential deposit. Set to 1/10 of its parent Relay Chain.
	pub const EXISTENTIAL_DEPOSIT: Balance = constants::currency::EXISTENTIAL_DEPOSIT / 10;

	pub const UNITS: Balance = constants::currency::UNITS;
	pub const CENTS: Balance = constants::currency::CENTS;
	pub const GRAND: Balance = constants::currency::GRAND;
	pub const MILLICENTS: Balance = constants::currency::MILLICENTS;

	pub const fn deposit(items: u32, bytes: u32) -> Balance {
		// map to 1/10 of what the kusama relay chain charges (v9020)
		constants::currency::deposit(items, bytes) / 10
	}
}

/// Fee-related.
pub mod fee {
	use super::currency::CENTS;
	use frame_support::weights::{
		constants::{ExtrinsicBaseWeight, WEIGHT_REF_TIME_PER_SECOND},
		WeightToFeeCoefficient, WeightToFeeCoefficients, WeightToFeePolynomial,
	};
	use polkadot_core_primitives::Balance;
	use smallvec::smallvec;
	pub use sp_runtime::Perbill;
	use cumulus_primitives_core::Weight;

	/// The block saturation level. Fees will be updates based on this value.
	pub const TARGET_BLOCK_FULLNESS: Perbill = Perbill::from_percent(25);

	pub const MAXIMUM_BLOCK_WEIGHT: Weight = Weight::from_parts(
		WEIGHT_REF_TIME_PER_SECOND.saturating_mul(2),
		cumulus_primitives_core::relay_chain::MAX_POV_SIZE as u64,
	);

	/// Handles converting a weight scalar to a fee value, based on the scale and granularity of the
	/// node's balance type.
	///
	/// This should typically create a mapping between the following ranges:
	///   - [0, MAXIMUM_BLOCK_WEIGHT]
	///   - [Balance::min, Balance::max]
	///
	/// Yet, it can be used for any other sort of change to weight-fee. Some examples being:
	///   - Setting it to `0` will essentially disable the weight fee.
	///   - Setting it to `1` will cause the literal `#[weight = x]` values to be charged.
	pub struct WeightToFee;
	impl WeightToFeePolynomial for WeightToFee {
		type Balance = Balance;
		fn polynomial() -> WeightToFeeCoefficients<Self::Balance> {
			// in Kusama, extrinsic stout weight (smallest non-zero weight) is mapped to 1/10 CENT:
			// in Statemine, we map to 1/10 of that, or 1/100 CENT
			let p = super::currency::CENTS;
			let q = 100 * Balance::from(ExtrinsicBaseWeight::get().ref_time());
			smallvec![WeightToFeeCoefficient {
				degree: 1,
				negative: false,
				coeff_frac: Perbill::from_rational(p % q, q),
				coeff_integer: p / q,
			}]
		}
	}

	pub fn base_tx_fee() -> Balance {
		CENTS / 10
	}

	pub fn default_fee_per_second() -> u128 {
		let base_weight = Balance::from(ExtrinsicBaseWeight::get().ref_time());
		let base_tx_per_second = (WEIGHT_REF_TIME_PER_SECOND as u128) / base_weight;
		base_tx_per_second * base_tx_fee()
	}
}

pub mod time {
	pub const MILLISECS_PER_BLOCK: u64 = 6000;
	pub const SLOT_DURATION: u64 = MILLISECS_PER_BLOCK;
}
