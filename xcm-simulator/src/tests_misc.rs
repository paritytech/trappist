use super::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Once;
    use xcm::prelude::*;
    use xcm_simulator::{TestExt, Weight};
    use frame_support::{assert_ok, log};

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

    #[test]
    fn event_collection_works() {
        init_tracing();
    
        MockNet::reset();
    
        const AMOUNT: u128 = trappist::EXISTENTIAL_DEPOSIT * 10;
        const MAX_WEIGHT: u128 = 1_000_000_000;
    
        Trappist::execute_with(|| {
            assert_ok!(trappist::PolkadotXcm::execute(
                trappist::RuntimeOrigin::signed(ALICE),
                Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
                Weight::from_ref_time(MAX_WEIGHT as u64)
            ));
            output_events::<trappist::Runtime>();
            assert_eq!(3, trappist::System::events().len());
        });
    
        Base::execute_with(|| {
            assert_ok!(base::PolkadotXcm::execute(
                base::RuntimeOrigin::signed(ALICE),
                Box::new(VersionedXcm::from(Xcm(vec![WithdrawAsset(((0, Here), AMOUNT).into())]))),
                Weight::from_ref_time(MAX_WEIGHT as u64)
            ));
            output_events::<base::Runtime>();
            assert_eq!(1, trappist::System::events().len());
        });
    }

    // Helper for outputting events
	fn output_events<Runtime: frame_system::Config>() {
		const TARGET: &str = "system::events";
		let events = frame_system::Pallet::<Runtime>::events();
		log::trace!(target: TARGET, "{} events", events.len());
		for event in events {
			log::trace!(target: TARGET, "{:?}", event)
		}
	}
}