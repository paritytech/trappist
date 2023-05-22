use ::trappist_runtime_benchmarks::WeightInfo;
use xcm_primitives::DropAssetsWeigher;

use crate::{Runtime, Weight};

mod trappist_runtime_benchmarks;

pub struct TrappistDropAssetsWeigher();
impl DropAssetsWeigher for TrappistDropAssetsWeigher {
	fn fungible() -> Weight {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_fungible()
	}

	fn native() -> Weight {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_native()
	}

	fn default() -> Weight {
		trappist_runtime_benchmarks::WeightInfo::<Runtime>::drop_assets_default()
	}
}
