//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as WithdrawAndTeleport;
use frame_benchmarking::{impl_benchmark_test_suite, v2::*};
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn withdraw_and_teleport() -> Result<(), BenchmarkError> {
		let asset: MultiAsset = (MultiLocation::new(1, Here), 10).into();
		let caller: T::AccountId = account("caller", 0, 0);

		let recipient = [0u8; 32];
		let versioned_dest: VersionedMultiLocation = MultiLocation::new(1, X1(Parachain(1))).into();
		let versioned_beneficiary: VersionedMultiLocation =
			AccountId32 { network: None, id: recipient.into() }.into();
		let versioned_assets: VersionedMultiAssets = asset.into();

		let amount: u128 = 1_000_000_000;

		#[extrinsic_call]
		withdraw_and_teleport(
			RawOrigin::Signed(caller),
			Box::new(versioned_dest),
			Box::new(versioned_beneficiary),
			amount,
			Box::new(versioned_assets),
		);

        Ok(())

		// TODO: Assert event emited.
	}

	impl_benchmark_test_suite!(WithdrawAndTeleport, crate::mock::new_test_ext(), crate::mock::Test);
}
