use super::*;
use frame_support::{assert_ok, instances::Instance1, traits::PalletInfoAccess};
use integration_tests_common::constants::{accounts::ALICE, XCM_V3};
use parity_scale_codec::Encode;
use sp_runtime::traits::{BlakeTwo256, Hash};
use thousands::Separable;
use xcm::{
	opaque::lts::{
		prelude::{
			BuyExecution, DepositAsset, DepositReserveAsset, InitiateReserveWithdraw, Transact,
			UnpaidExecution, WithdrawAsset, Xcm,
		},
		AssetId::Concrete,
		Fungibility::Fungible,
		Junction::{AccountId32, GeneralIndex, PalletInstance, Parachain},
		Junctions::{Here, X1, X2, X3},
		MultiAsset,
		MultiAssetFilter::Wild,
		OriginKind,
		WeightLimit::Unlimited,
		WildMultiAsset::AllCounted,
	},
	VersionedMultiAssets, VersionedMultiLocation, VersionedXcm,
};
use xcm_emulator::{
	assert_expected_events, bx, log, Chain, MultiAssets, MultiLocation, Network,
	Parachain as EmulatorParachain, Parent, RelayChain, TestExt, Weight, WeightLimit,
};
use xcm_executor::Assets;
use xcm_primitives::AssetMultiLocationGetter;

mod misc;
mod reserve_asset_transfer;
mod set_xcm_version;
mod xcm_asset_trap;

#[allow(non_upper_case_globals)]
const xUSD: u32 = 1984;
#[allow(non_upper_case_globals)]
const txUSD: u32 = 10;

const ASSET_HUB_ID: u32 = 1_000;
const TRAPPIST_ID: u32 = 1_836;
const STOUT_ID: u32 = 3_000;

static INIT: std::sync::Once = std::sync::Once::new();
fn init_tracing() {
	INIT.call_once(|| {
		// Add test tracing (from sp_tracing::init_for_tests()) but filtering for xcm logs only
		let _ = tracing_subscriber::fmt()
			.with_max_level(tracing::Level::TRACE)
			// Comment out this line to see all traces
			.with_env_filter(
				vec!["xcm=trace", "events=trace", "hrmp=trace", "dmp=trace", "ump=trace"].join(","),
			)
			.with_test_writer()
			.init();
	});
}

fn output_events<Runtime: frame_system::Config>() {
	const TARGET: &str = "system::events";
	let events = frame_system::Pallet::<Runtime>::events();
	log::trace!(target: TARGET, "{} events", events.len());
	for event in events {
		log::trace!(target: TARGET, "{:?}", event)
	}
}