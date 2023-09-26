//! Benchmarking setup for pallet-template
#![cfg(feature = "runtime-benchmarks")]
use super::*;

#[allow(unused)]
use crate::Pallet as WithdrawAndTeleport;
use frame_benchmarking::{
	impl_benchmark_test_suite,
	v2::*,
};
use frame_system::RawOrigin;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn withdraw_and_teleport() {
		// TODO: Create caller & assets

        let fee_asset = Concrete(MultiLocation::parent());

		// TODO: Fund caller with native

		// TODO: Fund caller with fee asset

		// TODO: Amount of Native to send

		// TODO: Amount of Fee Asset to send

		// TODO: Call create_game extrinsic
		#[extrinsic_call]
		withdraw_and_teleport(RawOrigin::Signed(caller), bet);

		// TODO: Assert event emited.
	}

	impl_benchmark_test_suite!(WithdrawAndTeleport, crate::mock::new_test_ext(), crate::mock::Test);
}
