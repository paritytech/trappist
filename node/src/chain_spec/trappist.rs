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

use crate::chain_spec::{
	get_account_id_from_seed, get_collator_keys_from_seed, Extensions, SAFE_XCM_VERSION,
};
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_service::ChainType;
use sp_core::{crypto::UncheckedInto, sr25519};
use trappist_runtime::{
	constants::currency::EXISTENTIAL_DEPOSIT, AccountId, AuraId, Balance, SessionKeys,
};

const DEFAULT_PROTOCOL_ID: &str = "hop";

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<trappist_runtime::RuntimeGenesisConfig, Extensions>;

const TRAPPIST_PARA_ID: u32 = 1836;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
fn session_keys(aura: AuraId) -> SessionKeys {
	SessionKeys { aura }
}

pub fn development_config() -> ChainSpec {
	// Give your stout currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "HOP".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		trappist_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: TRAPPIST_PARA_ID,
		},
	)
	.with_name("Trappist Development")
	.with_id("trappist_dev")
	.with_chain_type(ChainType::Development)
	.with_genesis_config_patch(testnet_genesis(
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed::<AuraId>("Bob"),
			),
		],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		2000.into(),
	))
	.with_properties(properties)
	.build()
}

pub fn trappist_local_testnet_config() -> ChainSpec {
	// Give your stout currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "HOP".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		trappist_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: TRAPPIST_PARA_ID,
		},
	)
	.with_name("Trappist Local")
	.with_id("trappist_local")
	.with_chain_type(ChainType::Local)
	.with_genesis_config_patch(testnet_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed::<AuraId>("Bob"),
			),
		],
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
		],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		TRAPPIST_PARA_ID.into(),
	))
	.with_protocol_id(DEFAULT_PROTOCOL_ID)
	.with_properties(properties)
	.build()
}

/// Configure initial storage state for FRAME modules.
pub fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root_key: AccountId,
	id: ParaId,
) -> serde_json::Value {
	let balances: Vec<(sp_runtime::AccountId32, Balance)> = endowed_accounts
		.iter()
		.map(|x| (x.clone(), 1_000_000_000_000_000_000))
		.collect::<Vec<_>>();
	serde_json::json!({
		"balances": {
			"balances": balances
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                // account id
						acc,                        // validator id
						session_keys(aura), // session keys
					)
				})
				.collect::<Vec<_>>(),
		},
		"sudo": { "key": Some(root_key) },
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
	})
}

pub fn trappist_live_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "HOP".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::builder(
		trappist_runtime::WASM_BINARY.expect("WASM binary was not built, please build it!"),
		Extensions {
			relay_chain: "westend".into(), // You MUST set this to the correct network!
			para_id: TRAPPIST_PARA_ID,
		},
	)
	.with_name("Trappist")
	.with_id("trappist")
	.with_chain_type(ChainType::Live)
	.with_genesis_config_patch(trappist_live_genesis(
		// initial collators.
		vec![
			(
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				get_collator_keys_from_seed::<AuraId>("Alice"),
			),
			(
				get_account_id_from_seed::<sr25519::Public>("Bob"),
				get_collator_keys_from_seed::<AuraId>("Bob"),
			),
		],
		vec![
			// This account will have root origin
			hex!("6a3db76f636ce43faaf58dde5a71a8e03b9d4ae3b331cff85c092f5bf98d971b").into(),
			hex!("3e79b5cb39533bd4a20f1a4b8ca5e62d264164cdf1389d568f73bc3932b5144a").into(),
			hex!("d00c901e43ab81cd9f26dc1b0c109a243134c47fee89d897f3fbf03e860c6d45").into(),
			hex!("30a12eef517fb62d993a605bc98183fa9b2336197da9f34414bcbf67839d0b14").into(),
			hex!("c0612ba544f0c34b5b0e102bfa7139e14cc7dc106ba7d34f317adca7fa30bb27").into(),
			hex!("1a2477ef6ea36d70bc6058a97d9bbbdfea103710cf2fbb9586269db72ab98f1a").into(),
		],
		hex!("e40839fde680c01344c20d47b7f08d2926b8a7537697356d416987a04a4453d0").into(),
		TRAPPIST_PARA_ID.into(),
	))
	.with_protocol_id(DEFAULT_PROTOCOL_ID)
	.with_properties(properties)
	.build()
}

fn trappist_live_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	endowed_accounts: Vec<AccountId>,
	root_key: AccountId,
	id: ParaId,
) -> serde_json::Value {
	let balances: Vec<(sp_runtime::AccountId32, Balance)> = endowed_accounts
		.iter()
		.map(|x| (x.clone(), 1_500_000_000_000_000_000))
		.chain(std::iter::once((root_key.clone(), 1_000_000_000_000_000_000)))
		.collect::<Vec<_>>();

	serde_json::json!({
		"balances": {
			"balances": balances
		},
		"parachainInfo": {
			"parachainId": id,
		},
		"collatorSelection": {
			"invulnerables": invulnerables.iter().cloned().map(|(acc, _)| acc).collect::<Vec<_>>(),
			"candidacyBond": EXISTENTIAL_DEPOSIT * 16,
		},
		"session": {
			"keys": invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                // account id
						acc,                        // validator id
						session_keys(aura), // session keys
					)
				})
				.collect::<Vec<_>>(),
		},
		"sudo": { "key": Some(root_key) },
		"polkadotXcm": {
			"safeXcmVersion": Some(SAFE_XCM_VERSION),
		},
	})
}
