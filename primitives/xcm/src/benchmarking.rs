use super::*;
use frame_benchmarking::{benchmarks};
use sp_std::{prelude::*, vec};
use xcm::v1::{AssetId, Fungibility, MultiAsset};
use frame_system::Pallet;


fn create_and_set_assets(count: u32) -> Assets {
	let mut multi_assets: Vec<MultiAsset> = Vec::new();
	for i in 0..count {
		let multilocation = MultiLocation::new(i.try_into().unwrap(), Here);
		//AssetMultiLocationSetter::set_asset_id(&multilocation, i);
		let asset = MultiAsset::from((Concrete(multilocation), Fungibility::Fungible(1_000_000_u128)));
		multi_assets.push(asset);
	}
	Assets::from(multi_assets)
}

benchmarks! {
  drop_assets {
		let s in 1 .. 10;
		let assets = create_and_set_assets(s);
		let multilocation = MultiLocation::new(1, Here);
		AssetMultiLocationSetter::set_asset_id(&multilocation, 1);

  	}: {
		TrappistDropAssets::drop_assets(&multilocation, assets)
	}

}
