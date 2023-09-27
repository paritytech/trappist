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
		let asset: MultiAsset = (MultiLocation::new(1, Here), 10).into();
		let caller: T::AccountId = account("caller", 0, 0);
		let initial_balance: u32 = 1_000_000_000;
		T::Currency::make_free_balance_be(&caller, initial_balance.into());

		let recipient = [0u8; 32];
		let versioned_dest: VersionedMultiLocation = T::ReachableDest::get()
			.ok_or(BenchmarkError::Override(BenchmarkResult::from_weight(Weight::MAX)))?
			.into();
		let versioned_beneficiary: VersionedMultiLocation =
			AccountId32 { network: None, id: recipient.into() }.into();
		let versioned_assets: VersionedMultiAssets = asset.into();

		let amount: u32 = 500_000_000;

		#[extrinsic_call]
		withdraw_and_teleport(
			RawOrigin::Signed(caller.clone()),
			Box::new(versioned_dest),
			Box::new(versioned_beneficiary),
			amount.into(),
			Box::new(versioned_assets),
		);
		// TODO: Change to asset check as SetFeesMode might impact the native balance
		let remaining_balance = initial_balance - amount;
		//assert_eq!(T::Currency::free_balance(&caller), remaining_balance.into());
		Ok(())
	}

	impl_benchmark_test_suite!(WithdrawTeleport, crate::mock::new_test_ext(), crate::mock::Test);
}
