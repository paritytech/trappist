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
	constants::currency::EXISTENTIAL_DEPOSIT, AccountId, AssetsConfig, AuraId, BalancesConfig,
	CouncilConfig, RuntimeGenesisConfig, SessionConfig, SessionKeys, SudoConfig, SystemConfig,
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

	ChainSpec::from_genesis(
		// Name
		"Trappist Development",
		// ID
		"trappist_dev",
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial collators.
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
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
				],
				2000.into(),
			)
		},
		// Bootnodes
		vec![],
		// Telemetry
		None,
		// Protocol ID
		Some(DEFAULT_PROTOCOL_ID),
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: TRAPPIST_PARA_ID,
		},
	)
}

pub fn trappist_local_testnet_config() -> ChainSpec {
	// Give your stout currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "HOP".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Trappist Local",
		// ID
		"trappist_local",
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial collators.
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
				// Sudo account
				get_account_id_from_seed::<sr25519::Public>("Alice"),
				// Pre-funded accounts
				vec![
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					get_account_id_from_seed::<sr25519::Public>("Bob"),
					get_account_id_from_seed::<sr25519::Public>("Charlie"),
					get_account_id_from_seed::<sr25519::Public>("Dave"),
					get_account_id_from_seed::<sr25519::Public>("Eve"),
					get_account_id_from_seed::<sr25519::Public>("Ferdie"),
				],
				TRAPPIST_PARA_ID.into(),
			)
		},
		// Bootnodes
		Vec::new(),
		// Telemetry
		None,
		// Protocol ID
		Some(DEFAULT_PROTOCOL_ID),
		None,
		// Properties
		Some(properties),
		// Extensions
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: TRAPPIST_PARA_ID,
		},
	)
}

/// Configure initial storage state for FRAME modules.
pub fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			code: trappist_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.into_iter().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: trappist_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		collator_selection: trappist_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().map(|(acc, _)| acc).cloned().collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		democracy: Default::default(),
		session: SessionConfig {
			keys: invulnerables
				.iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                // account id
						acc.clone(),                // validator id
						session_keys(aura.clone()), // session keys
					)
				})
				.collect(),
		},
		// no need to pass anything to aura, in fact it will panic if we do. Session will take care
		// of this.
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: trappist_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		assets: AssetsConfig { assets: vec![], accounts: vec![], metadata: vec![] },
		council: CouncilConfig {
			members: invulnerables.into_iter().map(|x| x.0).collect::<Vec<_>>(),
			phantom: Default::default(),
		},
		treasury: Default::default(),
		safe_mode: Default::default(),
		tx_pause: Default::default(),
		transaction_payment: Default::default(),
	}
}

pub fn trappist_live_config() -> ChainSpec {
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "HOP".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Trappist",
		// ID
		"trappist",
		ChainType::Live,
		move || {
			trappist_live_genesis(
				// initial collators.
				vec![
					(
						hex!("56266f110802ee790b5c40f63a0f9cba54d2889b014ea52661745557d09dbc1c")
							.into(),
						hex!("56266f110802ee790b5c40f63a0f9cba54d2889b014ea52661745557d09dbc1c")
							.unchecked_into(),
					),
					(
						hex!("64c2a2b803bdd4dcb88920ff4d56b618b2e5fbede48c4dc7cd78e562ebc06238")
							.into(),
						hex!("64c2a2b803bdd4dcb88920ff4d56b618b2e5fbede48c4dc7cd78e562ebc06238")
							.unchecked_into(),
					),
				],
				hex!("e40839fde680c01344c20d47b7f08d2926b8a7537697356d416987a04a4453d0").into(),
				vec![
					// This account will have root origin
					hex!("6a3db76f636ce43faaf58dde5a71a8e03b9d4ae3b331cff85c092f5bf98d971b").into(),
					hex!("3e79b5cb39533bd4a20f1a4b8ca5e62d264164cdf1389d568f73bc3932b5144a").into(),
					hex!("d00c901e43ab81cd9f26dc1b0c109a243134c47fee89d897f3fbf03e860c6d45").into(),
					hex!("30a12eef517fb62d993a605bc98183fa9b2336197da9f34414bcbf67839d0b14").into(),
					hex!("c0612ba544f0c34b5b0e102bfa7139e14cc7dc106ba7d34f317adca7fa30bb27").into(),
					hex!("1a2477ef6ea36d70bc6058a97d9bbbdfea103710cf2fbb9586269db72ab98f1a").into(),
				],
				TRAPPIST_PARA_ID.into(),
			)
		},
		vec![],
		None,
		None,
		None,
		Some(properties),
		Extensions { relay_chain: "rococo".into(), para_id: TRAPPIST_PARA_ID },
	)
}

fn trappist_live_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> RuntimeGenesisConfig {
	RuntimeGenesisConfig {
		system: SystemConfig {
			code: trappist_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.map(|x| (x.clone(), 1_500_000_000_000_000_000))
				.chain(std::iter::once((root_key.clone(), 1_000_000_000_000_000_000)))
				.collect(),
		},
		parachain_info: trappist_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		collator_selection: trappist_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().map(|(acc, _)| acc).cloned().collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		democracy: Default::default(),
		session: SessionConfig {
			keys: invulnerables
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),        // account id
						acc,                // validator id
						session_keys(aura), // session keys
					)
				})
				.collect(),
		},
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: trappist_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
			..Default::default()
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		assets: AssetsConfig { assets: vec![], accounts: vec![], metadata: vec![] },
		council: CouncilConfig {
			// We set the endowed accounts with balance as members of the council.
			members: endowed_accounts,
			phantom: Default::default(),
		},
		treasury: Default::default(),
		safe_mode: Default::default(),
		tx_pause: Default::default(),
		transaction_payment: Default::default(),
	}
}
