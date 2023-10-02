//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as WithdrawTeleport;
use frame_benchmarking::{impl_benchmark_test_suite, v2::*};
use frame_support::traits::Currency;
use frame_system::RawOrigin;
use sp_std::prelude::*;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn withdraw_and_teleport() -> Result<(), BenchmarkError> {
		let fee_amount = 1_000;
		let asset: MultiAsset = (MultiLocation::new(0, Here), fee_amount.clone()).into();
		let recipient = [0u8; 32];
		let versioned_dest: VersionedMultiLocation = T::ReachableDest::get()
			.ok_or(BenchmarkError::Override(BenchmarkResult::from_weight(Weight::MAX)))?
			.into();
		let versioned_beneficiary: VersionedMultiLocation =
			AccountId32 { network: None, id: recipient.into() }.into();
		let versioned_assets: VersionedMultiAssets = asset.into();
		let amount: u32 = 1_000;
		let caller = whitelisted_caller();
		T::Currency::make_free_balance_be(&caller, 100_000_000u32.into());
		let initial_balance = T::Currency::free_balance(&caller);

		#[extrinsic_call]
		withdraw_and_teleport(
			RawOrigin::Signed(caller.clone()),
			Box::new(versioned_dest),
			Box::new(versioned_beneficiary),
			amount.into(),
			Box::new(versioned_assets),
		);

		let remaining_balance = initial_balance - amount.into() - (fee_amount as u32).into();
		// Send or execution error would derive on balances amounts not being deducted from caller.
		assert_eq!(T::Currency::free_balance(&caller), remaining_balance);
		Ok(())
	}

	impl_benchmark_test_suite!(WithdrawTeleport, crate::mock::new_test_ext(), crate::mock::Test);
}
