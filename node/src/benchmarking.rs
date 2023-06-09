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

use std::{sync::Arc, time::Duration};

use cumulus_primitives_parachain_inherent::MockValidationDataInherentDataProvider;
use parity_scale_codec::Encode;
use sc_client_api::BlockBackend;
use sp_core::Pair;
use sp_inherents::{InherentData, InherentDataProvider};
use sp_keyring::Sr25519Keyring;
use sp_runtime::{generic, OpaqueExtrinsic, SaturatedConversion};

use crate::service::ParachainClient;

/// Generates `System::Remark` extrinsics for the benchmarks.
///
/// Note: Should only be used for benchmarking.
pub struct RemarkBuilder<RuntimeApi> {
	client: Arc<ParachainClient<RuntimeApi>>,
}

impl<RuntimeApi> RemarkBuilder<RuntimeApi> {
	/// Creates a new [`Self`] from the given client.
	pub fn new(client: Arc<ParachainClient<RuntimeApi>>) -> Self {
		Self { client }
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder<trappist_runtime::RuntimeApi> {
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> Result<OpaqueExtrinsic, &'static str> {
		use trappist_runtime as runtime;

		let call: runtime::RuntimeCall = runtime::SystemCall::remark { remark: vec![] }.into();
		let period = runtime::BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let best_block = self.client.chain_info().best_number;
		let tip = 0;
		let extra: runtime::SignedExtra = (
			frame_system::CheckNonZeroSender::<runtime::Runtime>::new(),
			frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
			frame_system::CheckTxVersion::<runtime::Runtime>::new(),
			frame_system::CheckGenesis::<runtime::Runtime>::new(),
			frame_system::CheckEra::<runtime::Runtime>::from(generic::Era::mortal(
				period,
				best_block.saturated_into(),
			)),
			frame_system::CheckNonce::<runtime::Runtime>::from(nonce),
			frame_system::CheckWeight::<runtime::Runtime>::new(),
			pallet_asset_tx_payment::ChargeAssetTxPayment::<runtime::Runtime>::from(tip, None),
		);

		let genesis_hash = self.client.block_hash(0).ok().flatten().expect("Genesis block exists");
		let best_hash = self.client.chain_info().best_hash;
		let payload = runtime::SignedPayload::from_raw(
			call.clone(),
			extra.clone(),
			(
				(),
				runtime::VERSION.spec_version,
				runtime::VERSION.transaction_version,
				genesis_hash,
				best_hash,
				(),
				(),
				(),
			),
		);

		let sender = Sr25519Keyring::Bob.pair();
		let signature = payload.using_encoded(|x| sender.sign(x));
		let extrinsic = runtime::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::AccountId32::from(sender.public()).into(),
			runtime::Signature::Sr25519(signature),
			extra,
		);

		Ok(extrinsic.into())
	}
}

impl frame_benchmarking_cli::ExtrinsicBuilder for RemarkBuilder<stout_runtime::RuntimeApi> {
	fn pallet(&self) -> &str {
		"system"
	}

	fn extrinsic(&self) -> &str {
		"remark"
	}

	fn build(&self, nonce: u32) -> Result<OpaqueExtrinsic, &'static str> {
		use stout_runtime as runtime;

		let call: runtime::RuntimeCall = runtime::SystemCall::remark { remark: vec![] }.into();
		let period = runtime::BlockHashCount::get()
			.checked_next_power_of_two()
			.map(|c| c / 2)
			.unwrap_or(2) as u64;
		let best_block = self.client.chain_info().best_number;
		let tip = 0;
		let extra: runtime::SignedExtra = (
			frame_system::CheckNonZeroSender::<runtime::Runtime>::new(),
			frame_system::CheckSpecVersion::<runtime::Runtime>::new(),
			frame_system::CheckTxVersion::<runtime::Runtime>::new(),
			frame_system::CheckGenesis::<runtime::Runtime>::new(),
			frame_system::CheckEra::<runtime::Runtime>::from(generic::Era::mortal(
				period,
				best_block.saturated_into(),
			)),
			frame_system::CheckNonce::<runtime::Runtime>::from(nonce),
			frame_system::CheckWeight::<runtime::Runtime>::new(),
			pallet_asset_tx_payment::ChargeAssetTxPayment::<runtime::Runtime>::from(tip, None),
		);

		let genesis_hash = self.client.block_hash(0).ok().flatten().expect("Genesis block exists");
		let best_hash = self.client.chain_info().best_hash;
		let payload = runtime::SignedPayload::from_raw(
			call.clone(),
			extra.clone(),
			(
				(),
				runtime::VERSION.spec_version,
				runtime::VERSION.transaction_version,
				genesis_hash,
				best_hash,
				(),
				(),
				(),
			),
		);

		let sender = Sr25519Keyring::Bob.pair();
		let signature = payload.using_encoded(|x| sender.sign(x));
		let extrinsic = runtime::UncheckedExtrinsic::new_signed(
			call,
			sp_runtime::AccountId32::from(sender.public()).into(),
			runtime::Signature::Sr25519(signature),
			extra,
		);

		Ok(extrinsic.into())
	}
}

/// Generates inherent data for the `benchmark overhead` command.
pub fn inherent_benchmark_data() -> sc_cli::Result<InherentData> {
	let mut inherent_data = InherentData::new();

	let timestamp = sp_timestamp::InherentDataProvider::new(Duration::ZERO.into());
	futures::executor::block_on(timestamp.provide_inherent_data(&mut inherent_data))
		.map_err(|e| format!("creating inherent data: {e:?}"))?;

	let parachain_inherent = MockValidationDataInherentDataProvider {
		current_para_block: 1,
		relay_offset: 0,
		relay_blocks_per_para_block: 1,
		para_blocks_per_relay_epoch: 0,
		relay_randomness_config: (),
		xcm_config: Default::default(),
		raw_downward_messages: Default::default(),
		raw_horizontal_messages: Default::default(),
	};

	futures::executor::block_on(parachain_inherent.provide_inherent_data(&mut inherent_data))
		.map_err(|e| format!("creating inherent data: {e:?}"))?;

	Ok(inherent_data)
}
