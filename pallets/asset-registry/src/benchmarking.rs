//! Benchmarking setup for pallet-asset-registry

use super::*;

#[allow(unused)]
use crate::Pallet as AssetRegistry;
use frame_benchmarking::benchmarks;
use frame_support::assert_ok;
use frame_system::RawOrigin;
use xcm::opaque::latest::{
	Junction::{GeneralIndex, PalletInstance, Parachain},
	Junctions, MultiLocation,
};

benchmarks! {
	register_reserve_asset {
		let asset_id = T::BenchmarkHelper::get_registered_asset();
		let asset_multi_location = MultiLocation {
			parents: 1,
			interior: Junctions::X3(Parachain(Default::default()), PalletInstance(Default::default()), GeneralIndex(Default::default()))
		};
	}: _(RawOrigin::Root, asset_id, asset_multi_location.clone())
	verify {
		assert_eq!(AssetIdMultiLocation::<T>::get(asset_id), Some(asset_multi_location));
	}

	unregister_reserve_asset {
		let asset_id = T::BenchmarkHelper::get_registered_asset();
		let asset_multi_location = MultiLocation {
			parents: 1,
			interior: Junctions::X3(Parachain(Default::default()), PalletInstance(Default::default()), GeneralIndex(Default::default()))
		};

		assert_ok!(AssetRegistry::<T>::register_reserve_asset(RawOrigin::Root.into(), asset_id, asset_multi_location.clone()));
		assert!(AssetIdMultiLocation::<T>::contains_key(asset_id));
	}: _(RawOrigin::Root, asset_id)
	verify {
		assert_eq!(AssetIdMultiLocation::<T>::get(asset_id), None);
	}

	impl_benchmark_test_suite!(AssetRegistry, crate::mock::new_test_ext(), crate::mock::Test);
}
