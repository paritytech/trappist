use super::*;

#[allow(unused)]
use crate::Pallet as MaintenanceMode;
use crate::{ACTIVATED, DEACTIVATED};
use frame_benchmarking::benchmarks;
use frame_support::{assert_ok, traits::fungibles::Inspect};
use frame_system::RawOrigin;
use xcm::opaque::latest::{
	Junction::{GeneralIndex, PalletInstance, Parachain},
	Junctions, MultiLocation,
};

benchmarks! {
	activate_maintenance_mode {
		MaintenanceModeStatus::<T>::put(DEACTIVATED);
	}: activate_maintenance_mode(RawOrigin::Root)
	verify {
		assert_eq!(MaintenanceModeStatus::<T>::get(), ACTIVATED);
	}

	 deactivate_maintenance_mode {
		MaintenanceModeStatus::<T>::put(ACTIVATED);
	}: deactivate_maintenance_mode(RawOrigin::Root)
	verify {
		assert_eq!(MaintenanceModeStatus::<T>::get(), DEACTIVATED);
	}

	impl_benchmark_test_suite!(MaintenanceMode, crate::mock::new_test_ext(), crate::mock::Test);
}
