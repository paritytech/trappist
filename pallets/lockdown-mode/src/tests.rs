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

use super::{GenesisConfig, *};
use crate::{mock::*, Error, ACTIVATED, DEACTIVATED};
use frame_support::{assert_noop, assert_ok, traits::Contains};
use pallet_balances::{self, Call as BalancesCall};
use pallet_remark::{self, Call as RemarkCall};

#[test]
fn genesis_config_default() {
	let default_genesis = GenesisConfig::<Test>::default();
	assert_eq!(default_genesis.initial_status, ACTIVATED);
}

#[test]
fn genesis_config_initialized() {
	[true, false].into_iter().for_each(|expected| {
		new_test_ext(expected).execute_with(|| {
			let lockdown_mode = LockdownModeStatus::<Test>::get();
			assert_eq!(lockdown_mode, expected);
		});
	});
}

#[test]
fn activate_lockdown_mode_works() {
	new_test_ext(false).execute_with(|| {
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
	new_test_ext(true).execute_with(|| {
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
	new_test_ext(false).execute_with(|| {
		assert_ok!(LockdownMode::activate_lockdown_mode(RuntimeOrigin::root()));
		let remark_call = RuntimeCall::Remark(RemarkCall::store { remark: vec![1, 2, 3] });
		let result: bool = LockdownMode::contains(&remark_call);
		assert!(result);
	});
}

#[test]
fn call_filtered_in_lockdown_mode() {
	new_test_ext(false).execute_with(|| {
		assert_ok!(LockdownMode::activate_lockdown_mode(RuntimeOrigin::root()));
		let balance_call =
			RuntimeCall::Balance(BalancesCall::transfer_allow_death { dest: 1, value: 2 });

		let result: bool = LockdownMode::contains(&balance_call);
		assert!(!result);
	});
}

#[test]
fn call_not_filtered_in_normal_mode() {
	new_test_ext(false).execute_with(|| {
		let lockdown_mode = LockdownModeStatus::<Test>::get();
		assert_eq!(lockdown_mode, DEACTIVATED);
		let balance_call =
			RuntimeCall::Balance(BalancesCall::transfer_allow_death { dest: 1, value: 2 });
		let result: bool = LockdownMode::contains(&balance_call);
		assert!(result);
	});
}
