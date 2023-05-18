use super::*;
use crate::{mock::*, Error, ACTIVATED, DEACTIVATED};
use frame_support::{assert_noop, assert_ok, traits::Contains};
use pallet_balances::{self, Call as BalancesCall};
use pallet_remark::{self, Call as RemarkCall};

#[test]
fn activate_lockdown_mode_works() {
	new_test_ext().execute_with(|| {
		assert_eq!(LockdownModeStatus::<Test>::get(), DEACTIVATED);
		assert_ok!(LockdownMode::activate_lockdown_mode(RuntimeOrigin::root()));

		let lockdown_mode = LockdownModeStatus::<Test>::get();
		assert_eq!(lockdown_mode, ACTIVATED);

		assert_noop!(
			LockdownMode::activate_lockdown_mode(RuntimeOrigin::root(),),
			Error::<Test>::LockdownModeAlreadyActivated
		);
	});
}

#[test]
fn deactivate_lockdown_mode_works() {
	new_test_ext().execute_with(|| {
		// We activate lockdown mode first so we can deactivate it.
		assert_ok!(LockdownMode::activate_lockdown_mode(RuntimeOrigin::root()));

		assert_ok!(LockdownMode::deactivate_lockdown_mode(RuntimeOrigin::root()));

		let lockdown_mode = LockdownModeStatus::<Test>::get();
		assert_eq!(lockdown_mode, DEACTIVATED);

		assert_noop!(
			LockdownMode::deactivate_lockdown_mode(RuntimeOrigin::root(),),
			Error::<Test>::LockdownModeAlreadyDeactivated
		);
	});
}

#[test]
fn call_not_filtered_in_lockdown_mode() {
	new_test_ext().execute_with(|| {
		assert_ok!(LockdownMode::activate_lockdown_mode(RuntimeOrigin::root()));
		let remark_call = RuntimeCall::Remark(RemarkCall::store { remark: vec![1, 2, 3] });
		let result: bool = LockdownMode::contains(&remark_call);
		assert!(result);
	});
}

#[test]
fn call_filtered_in_lockdown_mode() {
	new_test_ext().execute_with(|| {
		assert_ok!(LockdownMode::activate_lockdown_mode(RuntimeOrigin::root()));
		let balance_call = RuntimeCall::Balance(BalancesCall::transfer { dest: 1, value: 2 });

		let result: bool = LockdownMode::contains(&balance_call);
		assert!(!result);
	});
}

#[test]
fn call_not_filtered_in_normal_mode() {
	new_test_ext().execute_with(|| {
		let lockdown_mode = LockdownModeStatus::<Test>::get();
		assert_eq!(lockdown_mode, DEACTIVATED);
		let balance_call = RuntimeCall::Balance(BalancesCall::transfer { dest: 1, value: 2 });
		let result: bool = LockdownMode::contains(&balance_call);
		assert!(result);
	});
}