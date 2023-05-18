
use super::*;

#[allow(unused)]
use crate::Pallet as LockdownMode;
use crate::{ACTIVATED, DEACTIVATED};
use frame_benchmarking::benchmarks;
use frame_system::RawOrigin;

benchmarks! {
	activate_lockdown_mode {
		LockdownModeStatus::<T>::put(DEACTIVATED);
	}: activate_lockdown_mode(RawOrigin::Root)
	verify {
		assert_eq!(LockdownModeStatus::<T>::get(), ACTIVATED);
	}

	 deactivate_lockdown_mode {
		LockdownModeStatus::<T>::put(ACTIVATED);
	}: deactivate_lockdown_mode(RawOrigin::Root)
	verify {
		assert_eq!(LockdownModeStatus::<T>::get(), DEACTIVATED);
	}

	impl_benchmark_test_suite!(LockdownMode, crate::mock::new_test_ext(), crate::mock::Test);
}