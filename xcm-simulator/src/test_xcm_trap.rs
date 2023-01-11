use super::*;

#[cfg(test)]
mod tests {
	use super::*;
	use trappist_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

	use frame_support::assert_ok;
	use sp_runtime::traits::{BlakeTwo256, Hash};
	use std::sync::Once;
	use xcm::prelude::*;
	use xcm_executor::Assets;
	use xcm_simulator::{TestExt, Weight};

	static INIT: Once = Once::new();
	pub fn init_tracing() {
		INIT.call_once(|| {
			// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
			let _ = tracing_subscriber::fmt()
				.with_max_level(tracing::Level::TRACE)
				.with_env_filter("xcm=trace") // Comment out this line to see all traces
				.with_test_writer()
				.init();
		});
	}

	// make sure we can trap and claim a native asset
	#[test]
	fn native_trap_claim_works() {
		init_tracing();

		MockNet::reset();

		const AMOUNT: u128 = EXISTENTIAL_DEPOSIT * 10;
		const MAX_WEIGHT: u128 = 1_000_000_000;

		Trappist::execute_with(|| {
			use trappist::System;

			let origin: MultiLocation =
				Junction::AccountId32 { network: NetworkId::Any, id: ALICE.into() }.into();

			let native_asset: Assets = MultiAsset {
				id: AssetId::Concrete(MultiLocation { parents: 0, interior: Junctions::Here }),
				fun: Fungible((AMOUNT) as u128),
			}
			.into();

			assert_ok!(trappist::PolkadotXcm::execute(
				trappist::RuntimeOrigin::signed(ALICE),
				Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
				Weight::from_ref_time(MAX_WEIGHT as u64)
			));

			let expected_versioned =
				VersionedMultiAssets::from(MultiAssets::from(native_asset.clone()));
			let expected_hash = BlakeTwo256::hash_of(&(&origin, &expected_versioned));

			// we can read the asset trap storage
			let read_asset_trap = trappist::PolkadotXcm::asset_trap(expected_hash);
			// assert_eq!(
			//     read_asset_trap,
			//     1
			// );

			// assert_ok!(trappist::System::remark_with_event(trappist::RuntimeOrigin::signed(ALICE), vec![1]));

			// println!("aaaa {:?}", System::events());

			// assert!(System::events().iter().any(|r| matches!(
			// 	r.event,
			// 	RuntimeEvent::PolkadotXcm(pallet_xcm::Event::AssetsTrapped { .. })
			// )));
		});
	}
}
