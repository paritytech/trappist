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

use ::trappist_runtime_benchmarks::WeightInfo;
use xcm_primitives::DropAssetsWeigher;

use crate::Runtime;

pub mod block_weights;
pub mod extrinsic_weights;
pub mod trappist_runtime_benchmarks;
pub mod xcm;

pub struct TrappistDropAssetsWeigher();
impl DropAssetsWeigher for TrappistDropAssetsWeigher {
	fn fungible() -> u64 {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_fungible().ref_time()
	}

	fn native() -> u64 {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_native().ref_time()
	}

	fn default() -> u64 {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_default().ref_time()
	}
}
