use ::trappist_runtime_benchmarks::WeightInfo;
use xcm_primitives::DropAssetsWeigher;

use crate::Runtime;

mod trappist_runtime_benchmarks;

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
