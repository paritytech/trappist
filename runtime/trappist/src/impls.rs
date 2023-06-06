// Copyright (C) 2021 Parity Technologies (UK) Ltd.
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

//! Auxiliary struct/enums for parachain runtimes.
//! Taken from polkadot/runtime/common (at a21cd64) and adapted for parachains.

use super::*;
use cumulus_primitives_core::{relay_chain::BlockNumber as RelayBlockNumber, DmpMessageHandler};
use frame_support::{
	traits::{Contains, Currency, Imbalance, OnUnbalanced},
	weights::{Weight, WeightToFeeCoefficient},
};
pub use log;
use sp_runtime::{DispatchResult, SaturatedConversion};
use sp_arithmetic::traits::{BaseArithmetic, Unsigned};

use sp_std::marker::PhantomData;

/// Type alias to conveniently refer to the `Currency::NegativeImbalance` associated type.
pub type NegativeImbalance<T> = <pallet_balances::Pallet<T> as Currency<
	<T as frame_system::Config>::AccountId,
>>::NegativeImbalance;

/// Type alias to conveniently refer to `frame_system`'s `Config::AccountId`.
pub type AccountIdOf<R> = <R as frame_system::Config>::AccountId;

/// Logic for the author to get a portion of fees.
pub struct ToAuthor<R>(PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for ToAuthor<R>
where
	R: pallet_balances::Config + pallet_collator_selection::Config,
	AccountIdOf<R>: From<polkadot_core_primitives::v2::AccountId>
		+ Into<polkadot_core_primitives::v2::AccountId>,
	<R as frame_system::Config>::RuntimeEvent: From<pallet_balances::Event<R>>,
{
	fn on_nonzero_unbalanced(amount: NegativeImbalance<R>) {
		let author = <pallet_collator_selection::Pallet<R>>::account_id();
		<pallet_balances::Pallet<R>>::resolve_creating(&author, amount);
	}
}

pub struct DealWithFees<R>(PhantomData<R>);
impl<R> OnUnbalanced<NegativeImbalance<R>> for DealWithFees<R>
where
	R: pallet_balances::Config + pallet_collator_selection::Config + pallet_treasury::Config,
	pallet_treasury::Pallet<R>: OnUnbalanced<NegativeImbalance<R>>,
	AccountIdOf<R>: From<polkadot_core_primitives::v2::AccountId>
		+ Into<polkadot_core_primitives::v2::AccountId>,
	<R as frame_system::Config>::RuntimeEvent: From<pallet_balances::Event<R>>,
{
	fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item = NegativeImbalance<R>>) {
		if let Some(fees) = fees_then_tips.next() {
			// for fees, 80% to treasury, 20% to author
			let mut split = fees.ration(80, 20);
			if let Some(tips) = fees_then_tips.next() {
				// for tips, if any, 100% to author
				tips.merge_into(&mut split.1);
			}
			use pallet_treasury::Pallet as Treasury;
			<Treasury<R> as OnUnbalanced<_>>::on_unbalanced(split.0);
			<ToAuthor<R> as OnUnbalanced<_>>::on_unbalanced(split.1);
		}
	}
}

pub struct RuntimeBlackListedCalls;
impl Contains<RuntimeCall> for RuntimeBlackListedCalls {
	fn contains(call: &RuntimeCall) -> bool {
		match call {
			RuntimeCall::Balances(_) => false,
			RuntimeCall::Assets(_) => false,
			RuntimeCall::Dex(_) => false,
			RuntimeCall::PolkadotXcm(_) => false,
			RuntimeCall::Treasury(_) => false,
			RuntimeCall::Chess(_) => false,
			RuntimeCall::Contracts(_) => false,
			RuntimeCall::Uniques(_) => false,
			RuntimeCall::AssetRegistry(_) => false,
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
		DmpQueue::handle_dmp_messages(_iter, limit)
	}
}

pub struct XcmExecutionManager {}
impl xcm_primitives::PauseXcmExecution for XcmExecutionManager {
	fn suspend_xcm_execution() -> DispatchResult {
		XcmpQueue::suspend_xcm_execution(RuntimeOrigin::root())
	}
	fn resume_xcm_execution() -> DispatchResult {
		XcmpQueue::resume_xcm_execution(RuntimeOrigin::root())
	}
}

pub trait WeightCoefficientCalc<Balance> {
	fn saturating_eval(&self, result: Balance, x: Balance) -> Balance;
}

impl<Balance> WeightCoefficientCalc<Balance> for WeightToFeeCoefficient<Balance>
where
	Balance: BaseArithmetic + From<u32> + Copy + Unsigned + SaturatedConversion,
{
	fn saturating_eval(&self, mut result: Balance, x: Balance) -> Balance {
		let power = x.saturating_pow(self.degree.into());

		let frac = self.coeff_frac * power; // Overflow safe.
		let integer = self.coeff_integer.saturating_mul(power);
		// Do not add them together here to avoid an underflow.

		if self.negative {
			result = result.saturating_sub(frac);
			result = result.saturating_sub(integer);
		} else {
			result = result.saturating_add(frac);
			result = result.saturating_add(integer);
		}

		result
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use frame_support::{
		parameter_types,
		traits::{FindAuthor, ValidatorRegistration},
		PalletId,
	};
	use frame_system::{limits, EnsureRoot};
	use pallet_collator_selection::IdentityCollator;
	use polkadot_core_primitives::v2::AccountId;
	use sp_core::H256;
	use sp_runtime::{
		testing::Header,
		traits::{BlakeTwo256, ConstU32, ConstU64, IdentityLookup},
		Perbill, Permill,
	};

	type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
	type Block = frame_system::mocking::MockBlock<Test>;
	const TEST_ACCOUNT: AccountId = AccountId::new([1; 32]);

	frame_support::construct_runtime!(
		pub enum Test where
			Block = Block,
			NodeBlock = Block,
			UncheckedExtrinsic = UncheckedExtrinsic,
		{
			System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
			Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
			CollatorSelection: pallet_collator_selection::{Pallet, Call, Storage, Event<T>},
			Treasury: pallet_treasury::{Pallet, Call, Storage, Event<T>},
		}
	);

	parameter_types! {
		pub const BlockHashCount: u64 = 250;
		pub BlockLength: limits::BlockLength = limits::BlockLength::max(2 * 1024);
		pub const AvailableBlockRatio: Perbill = Perbill::one();
		pub const MaxReserves: u32 = 50;
	}

	impl frame_system::Config for Test {
		type BaseCallFilter = frame_support::traits::Everything;
		type RuntimeOrigin = RuntimeOrigin;
		type Index = u64;
		type BlockNumber = u64;
		type RuntimeCall = RuntimeCall;
		type Hash = H256;
		type Hashing = BlakeTwo256;
		type AccountId = AccountId;
		type Lookup = IdentityLookup<Self::AccountId>;
		type Header = Header;
		type RuntimeEvent = RuntimeEvent;
		type BlockHashCount = BlockHashCount;
		type BlockLength = BlockLength;
		type BlockWeights = ();
		type DbWeight = ();
		type Version = ();
		type PalletInfo = PalletInfo;
		type AccountData = pallet_balances::AccountData<u64>;
		type OnNewAccount = ();
		type OnKilledAccount = ();
		type SystemWeightInfo = ();
		type SS58Prefix = ();
		type OnSetCode = ();
		type MaxConsumers = frame_support::traits::ConstU32<16>;
	}

	impl pallet_balances::Config for Test {
		type Balance = u64;
		type RuntimeEvent = RuntimeEvent;
		type DustRemoval = ();
		type ExistentialDeposit = ();
		type AccountStore = System;
		type MaxLocks = ();
		type WeightInfo = ();
		type MaxReserves = MaxReserves;
		type ReserveIdentifier = [u8; 8];
	}

	pub struct OneAuthor;
	impl FindAuthor<AccountId> for OneAuthor {
		fn find_author<'a, I>(_: I) -> Option<AccountId>
		where
			I: 'a,
		{
			Some(TEST_ACCOUNT)
		}
	}

	pub struct IsRegistered;
	impl ValidatorRegistration<AccountId> for IsRegistered {
		fn is_registered(_id: &AccountId) -> bool {
			true
		}
	}

	parameter_types! {
		pub const PotId: PalletId = PalletId(*b"PotStake");
		pub const MaxCandidates: u32 = 20;
		pub const MaxInvulnerables: u32 = 20;
		pub const MinCandidates: u32 = 1;
	}

	impl pallet_collator_selection::Config for Test {
		type RuntimeEvent = RuntimeEvent;
		type Currency = Balances;
		type UpdateOrigin = EnsureRoot<AccountId>;
		type PotId = PotId;
		type MaxCandidates = MaxCandidates;
		type MinCandidates = MinCandidates;
		type MaxInvulnerables = MaxInvulnerables;
		type ValidatorId = <Self as frame_system::Config>::AccountId;
		type ValidatorIdOf = IdentityCollator;
		type ValidatorRegistration = IsRegistered;
		type KickThreshold = ();
		type WeightInfo = ();
	}

	impl pallet_authorship::Config for Test {
		type FindAuthor = OneAuthor;
		type EventHandler = ();
	}

	parameter_types! {
		pub const ProposalBond: Permill = Permill::from_percent(5);
		pub const Burn: Permill = Permill::from_percent(50);
		pub const TreasuryPalletId: PalletId = PalletId(*b"py/trsry");
	}
	pub struct TestSpendOrigin;
	impl frame_support::traits::EnsureOrigin<RuntimeOrigin> for TestSpendOrigin {
		type Success = u64;
		fn try_origin(o: RuntimeOrigin) -> Result<Self::Success, RuntimeOrigin> {
			Result::<frame_system::RawOrigin<_>, RuntimeOrigin>::from(o).and_then(|o| match o {
				frame_system::RawOrigin::Root => Ok(u64::max_value()),
				r => Err(RuntimeOrigin::from(r)),
			})
		}
		#[cfg(feature = "runtime-benchmarks")]
		fn try_successful_origin() -> Result<RuntimeOrigin, ()> {
			Ok(RuntimeOrigin::root())
		}
	}

	impl pallet_treasury::Config for Test {
		type PalletId = TreasuryPalletId;
		type Currency = pallet_balances::Pallet<Test>;
		type ApproveOrigin = EnsureRoot<AccountId>;
		type RejectOrigin = EnsureRoot<AccountId>;
		type RuntimeEvent = RuntimeEvent;
		type OnSlash = ();
		type ProposalBond = ProposalBond;
		type ProposalBondMinimum = ConstU64<1>;
		type ProposalBondMaximum = ();
		type SpendPeriod = ConstU64<2>;
		type Burn = Burn;
		type BurnDestination = (); // Just gets burned.
		type WeightInfo = ();
		type SpendFunds = ();
		type MaxApprovals = ConstU32<100>;
		type SpendOrigin = TestSpendOrigin;
	}

	pub fn new_test_ext() -> sp_io::TestExternalities {
		let mut t = frame_system::GenesisConfig::default().build_storage::<Test>().unwrap();
		// We use default for brevity, but you can configure as desired if needed.
		pallet_balances::GenesisConfig::<Test>::default()
			.assimilate_storage(&mut t)
			.unwrap();
		t.into()
	}

	#[test]
	fn test_fees_and_tip_split() {
		new_test_ext().execute_with(|| {
			let fee = Balances::issue(100);
			let tip = Balances::issue(30);

			assert_eq!(Treasury::pot(), 0);

			DealWithFees::on_unbalanceds(vec![fee, tip].into_iter());

			// Author should get 20% of the fee + the 100% of the tip. (50)
			assert_eq!(Balances::free_balance(CollatorSelection::account_id()), 50);
			// Treasury should get 80% of the fee. (80)
			assert_eq!(Treasury::pot(), 80);
		});
	}
}
