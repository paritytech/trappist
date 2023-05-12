use frame_benchmarking::benchmarks;
use sp_runtime::SaturatedConversion;
use xcm::prelude::AssetId as XcmAssetId;

use crate::*;

benchmarks! {
	drop_assets_fungible {
		let origin = MultiLocation::default();
		let asset_id = 1;
		let location = Parachain(asset_id).into();
		T::register_asset(asset_id.into(), location.clone());
		let asset = MultiAsset { id: XcmAssetId::Concrete(location), fun: Fungibility::Fungible(100) };
	} : {
		T::DropAssets::drop_assets(&origin, asset.into());
	}

	drop_assets_native {
		let origin = MultiLocation::default();
		let location = MultiLocation { parents: 0, interior: Here };
		let amount = T::ExistentialDeposit::get().saturated_into();
		let asset = MultiAsset { id: XcmAssetId::Concrete(location), fun: Fungibility::Fungible(amount) };
	} : {
		T::DropAssets::drop_assets(&origin, asset.into());
	}

	drop_assets_default {
		let origin = MultiLocation::default();
		let asset = MultiAsset { id: XcmAssetId::Abstract(Default::default()), fun: Fungibility::Fungible(0) };
	} : {
		T::DropAssets::drop_assets(&origin, asset.into());
	}
}
