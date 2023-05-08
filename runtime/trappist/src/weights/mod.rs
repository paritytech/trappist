use ::pallet_benchmarks::WeightInfo;
use xcm_primitives::DropAssetsWeigher;

use crate::Runtime;

mod pallet_benchmarks;

pub struct TrappistDropAssetsWeigher();
impl DropAssetsWeigher for TrappistDropAssetsWeigher {
	fn fungible() -> u64 {
		pallet_benchmarks::WeightInfo::<Runtime>::drop_assets_fungible().ref_time()
	}

	fn native() -> u64 {
		pallet_benchmarks::WeightInfo::<Runtime>::drop_assets_native().ref_time()
	}

	fn default() -> u64 {
		pallet_benchmarks::WeightInfo::<Runtime>::drop_assets_default().ref_time()
	}
}
