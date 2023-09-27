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
		let asset: MultiAsset = (MultiLocation::new(0, Here), 10_000).into();
		let send_origin =
			T::ExecuteXcmOrigin::try_successful_origin().map_err(|_| BenchmarkError::Weightless)?;
		let recipient = [0u8; 32];
		let versioned_dest: VersionedMultiLocation = T::ReachableDest::get()
			.ok_or(BenchmarkError::Override(BenchmarkResult::from_weight(Weight::MAX)))?
			.into();
		let versioned_beneficiary: VersionedMultiLocation =
			AccountId32 { network: None, id: recipient.into() }.into();
		let versioned_assets: VersionedMultiAssets = asset.into();
		let amount: u32 = 50_000_000;

		#[extrinsic_call]
		withdraw_and_teleport(
			send_origin as <T as frame_system::Config>::RuntimeOrigin,
			Box::new(versioned_dest),
			Box::new(versioned_beneficiary),
			amount.into(),
			Box::new(versioned_assets),
		);
		Ok(())
	}

	impl_benchmark_test_suite!(WithdrawTeleport, crate::mock::new_test_ext(), crate::mock::Test);
}
