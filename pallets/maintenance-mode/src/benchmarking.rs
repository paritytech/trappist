//! Benchmarking setup for pallet-asset-registry

use super::*;

#[allow(unused)]
use crate::Pallet as MaintenanceMode;
use crate::{ACTIVATED, DEACTIVATEDD};
use frame_benchmarking::benchmarks;
use frame_support::{assert_ok, traits::fungibles::Inspect};
use frame_system::RawOrigin;
use xcm::opaque::latest::{
	Junction::{GeneralIndex, PalletInstance, Parachain},
	Junctions, MultiLocation,
};

benchmarks! {
	activate_maintenance_mode {
		MaintenanceMode::<T>::activate_maintenance_mode(RawOrigin::Root.into())?;
	}: _(RawOrigin::Root)
	verify {
		assert_eq!(MaintenanceModeStatus::<T>::get(), ACTIVATED);
	}

	deactivate_maintenance_mode {
		MaintenanceMode::<T>::deactivate_maintenance_mode(RawOrigin::Root.into())?;
	}: _(RawOrigin::Root)
	verify {
		assert_eq!(MaintenanceModeStatus::<T>::get(), DEACTIVATEDD);
	}

	impl_benchmark_test_suite!(MaintenanceMode, crate::mock::new_test_ext(), crate::mock::Test);
}
