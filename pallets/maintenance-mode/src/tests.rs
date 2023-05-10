use super::*;
use crate::{mock::*, Error, ACTIVATED, DEACTIVATED};
use frame_support::{assert_noop, assert_ok, traits::Contains};
use pallet_balances::{self, Call as BalancesCall};
use pallet_remark::{self, Call as RemarkCall};

#[test]
fn activate_maintenance_mode_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(MaintenanceModeStatus::<Test>::get(), DEACTIVATED);
		assert_ok!(MaintenanceMode::activate_maintenance_mode(RuntimeOrigin::root()));

		let maintenance_mode = MaintenanceModeStatus::<Test>::get();
		assert_eq!(maintenance_mode, ACTIVATED);

		assert_noop!(
			MaintenanceMode::activate_maintenance_mode(RuntimeOrigin::root(),),
			Error::<Test>::MaintenanceModeAlreadyActivated
		);
	});
}

#[test]
fn deactivate_maintenance_mode_works() {
	new_test_ext().execute_with(|| {
		// We activate maintenance mode first so we can deactivate it.
		assert_ok!(MaintenanceMode::activate_maintenance_mode(RuntimeOrigin::root()));

		assert_ok!(MaintenanceMode::deactivate_maintenance_mode(RuntimeOrigin::root()));

		let maintenance_mode = MaintenanceModeStatus::<Test>::get();
		assert_eq!(maintenance_mode, DEACTIVATED);

		assert_noop!(
			MaintenanceMode::deactivate_maintenance_mode(RuntimeOrigin::root(),),
			Error::<Test>::MaintenanceModeAlreadyDeactivated
		);
	});
}

#[test]
fn call_not_filtered_in_maintenance_mode() {
	new_test_ext().execute_with(|| {
		assert_ok!(MaintenanceMode::activate_maintenance_mode(RuntimeOrigin::root()));
		let remark_call = RuntimeCall::Remark(RemarkCall::store { remark: vec![1, 2, 3] });
		let result: bool = MaintenanceMode::contains(&remark_call);
		assert!(result);
	});
}

#[test]
fn call_filtered_in_maintenance_mode() {
	new_test_ext().execute_with(|| {
		assert_ok!(MaintenanceMode::activate_maintenance_mode(RuntimeOrigin::root()));
		let balance_call = RuntimeCall::Balance(BalancesCall::transfer { dest: 1, value: 2 });

		let result: bool = MaintenanceMode::contains(&balance_call);
		assert!(!result);
	});
}

#[test]
fn call_not_filtered_in_normal_mode() {
	new_test_ext().execute_with(|| {
		let maintenance_mode = MaintenanceModeStatus::<Test>::get();
		assert_eq!(maintenance_mode, DEACTIVATED);
		let balance_call = RuntimeCall::Balance(BalancesCall::transfer { dest: 1, value: 2 });
		let result: bool = MaintenanceMode::contains(&balance_call);
		assert!(result);
	});
}
