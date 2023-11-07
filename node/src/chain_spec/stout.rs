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

use sc_service::ChainType;

use sp_core::sr25519;
use stout_runtime::{
	constants::currency::EXISTENTIAL_DEPOSIT, AccountId, AssetsConfig, AuraId, BalancesConfig,
	CouncilConfig, ForeignAssetsConfig, PoolAssetsConfig, RuntimeGenesisConfig, SessionConfig,
	SessionKeys, SudoConfig, SystemConfig,
};

const DEFAULT_PROTOCOL_ID: &str = "stout";

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<stout_runtime::RuntimeGenesisConfig, Extensions>;

const STOUT_PARA_ID: u32 = 3000;

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
fn session_keys(aura: AuraId) -> SessionKeys {
	SessionKeys { aura }
}

pub fn stout_local_testnet_config() -> ChainSpec {
	// Give your stout currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "STOUT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

	ChainSpec::from_genesis(
		// Name
		"Stout Local",
		// ID
		"stout_local",
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
				STOUT_PARA_ID.into(),
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
			para_id: STOUT_PARA_ID,
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
			code: stout_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.into_iter().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: stout_runtime::ParachainInfoConfig {
			parachain_id: id,
			..Default::default()
		},
		collator_selection: stout_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().map(|(acc, _)| acc).cloned().collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
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
		polkadot_xcm: stout_runtime::PolkadotXcmConfig {
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
		foreign_assets: ForeignAssetsConfig { assets: vec![], accounts: vec![], metadata: vec![] },
		pool_assets: PoolAssetsConfig { assets: vec![], accounts: vec![], metadata: vec![] },
	}
}
