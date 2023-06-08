// This file is part of Trappist.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frame_benchmarking::benchmarks;
use sp_runtime::SaturatedConversion;
use xcm::prelude::AssetId as XcmAssetId;

use crate::*;

benchmarks! {
	drop_assets_fungible {
		let origin = MultiLocation::default();
		let asset_id = 1;
		let location: MultiLocation = Parachain(asset_id).into();
		T::register_asset(asset_id.into(), location.clone());
		let asset = MultiAsset { id: XcmAssetId::Concrete(location), fun: Fungibility::Fungible(100) };
	} : {
		T::DropAssets::drop_assets(
			&origin,
			asset.into(),
			&XcmContext {
				origin: Some(origin.clone()),
				message_hash: [0; 32],
				topic: None,
			},
		);
	}

	drop_assets_native {
		let origin = MultiLocation::default();
		let location = MultiLocation { parents: 0, interior: Here };
		let amount = T::ExistentialDeposit::get().saturated_into();
		let asset = MultiAsset { id: XcmAssetId::Concrete(location), fun: Fungibility::Fungible(amount) };
	} : {
		T::DropAssets::drop_assets(
			&origin,
			asset.into(),
			&XcmContext {
				origin: Some(origin.clone()),
				message_hash: [0; 32],
				topic: None,
			},
		);
	}

	drop_assets_default {
		let origin = MultiLocation::default();
		let asset = MultiAsset { id: XcmAssetId::Abstract(Default::default()), fun: Fungibility::Fungible(0) };
	} : {
		T::DropAssets::drop_assets(
			&origin,
			asset.into(),
			&XcmContext {
				origin: Some(origin.clone()),
				message_hash: [0; 32],
				topic: None,
			},
		);
	}
}
