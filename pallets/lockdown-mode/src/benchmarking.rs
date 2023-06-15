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

	impl_benchmark_test_suite!(LockdownMode, crate::mock::new_test_ext(true), crate::mock::Test);
}
