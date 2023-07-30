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

use crate as pallet_lockdown_mode;
use cumulus_primitives_core::{relay_chain::BlockNumber as RelayBlockNumber, DmpMessageHandler};
use frame_support::{
	traits::{ConstU16, ConstU64, Contains, GenesisBuild},
	weights::Weight,
};
use frame_system as system;
use sp_core::H256;
use sp_runtime::{
	testing::Header,
	traits::{BlakeTwo256, ConstU32, IdentityLookup},
	DispatchResult,
};

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::parameter_types! {
	pub const StatemineParaIdInfo: u32 = 1000u32;
	pub const StatemineAssetsInstanceInfo: u8 = 50u8;
	pub const StatemineAssetIdInfo: u128 = 1u128;
}

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
	pub enum Test where
		Block = Block,
		NodeBlock = Block,
		UncheckedExtrinsic = UncheckedExtrinsic,
	{
		System: frame_system,
		LockdownMode: pallet_lockdown_mode::{Pallet, Call, Storage, Event<T>},
		Balance: pallet_balances::{Pallet, Call, Storage, Event<T>},
		Remark: pallet_remark::{Pallet, Call, Storage, Event<T>},
	}
);

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Index = u64;
	type BlockNumber = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Header = Header;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type DbWeight = ();
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = ConstU32<16>;
}

impl pallet_balances::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type Balance = u64;
	type DustRemoval = ();
	type ExistentialDeposit = ConstU64<1>;
	type AccountStore = System;
	type ReserveIdentifier = [u8; 8];
	type HoldIdentifier = ();
	type FreezeIdentifier = ();
	type MaxLocks = ();
	type MaxReserves = ();
	type MaxHolds = ConstU32<0>;
	type MaxFreezes = ConstU32<0>;
}

pub struct RuntimeBlackListedCalls;
impl Contains<RuntimeCall> for RuntimeBlackListedCalls {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::Balance(_) => false,
			_ => true,
		}
	}
}

pub struct LockdownDmpHandler;
impl DmpMessageHandler for LockdownDmpHandler {
	fn handle_dmp_messages(
		_iter: impl Iterator<Item = (RelayBlockNumber, Vec<u8>)>,
		limit: Weight,
	) -> Weight {
		limit
	}
}

pub struct XcmExecutionManager {}

impl xcm_primitives::PauseXcmExecution for XcmExecutionManager {
	fn suspend_xcm_execution() -> DispatchResult {
		Ok(())
	}
	fn resume_xcm_execution() -> DispatchResult {
		Ok(())
	}
}

impl pallet_remark::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
}

impl pallet_lockdown_mode::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type LockdownModeOrigin = frame_system::EnsureRoot<Self::AccountId>;
	type BlackListedCalls = RuntimeBlackListedCalls;
	type LockdownDmpHandler = LockdownDmpHandler;
	type XcmExecutorManager = XcmExecutionManager;
	type WeightInfo = pallet_lockdown_mode::weights::SubstrateWeight<Test>;
}

pub fn new_test_ext(initial_status: bool) -> sp_io::TestExternalities {
	let mut storage = system::GenesisConfig::default().build_storage::<Test>().unwrap();
	GenesisBuild::<Test>::assimilate_storage(
		&pallet_lockdown_mode::GenesisConfig { initial_status },
		&mut storage,
	)
	.unwrap();
	storage.into()
}
