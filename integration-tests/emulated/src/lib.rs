use parachains_common::Balance;
use sp_core::{sr25519, storage::Storage};
use sp_runtime::BuildStorage;
use xcm_emulator::{
	decl_test_networks, decl_test_parachains, decl_test_relay_chains,
	helpers::get_account_id_from_seed, DefaultMessageProcessor, Hooks, ParaId,
};

#[cfg(test)]
mod tests;

decl_test_relay_chains! {
	// Rococo
	#[api_version(5)]
	pub struct Rococo {
		genesis = integration_tests_common::constants::rococo::genesis(),
		on_init = (),
		runtime = rococo_runtime,
		core = {
			MessageProcessor: DefaultMessageProcessor<Rococo>,
			SovereignAccountOf: rococo_runtime::xcm_config::LocationConverter,
		},
		pallets = {
			XcmPallet: rococo_runtime::XcmPallet,
			Sudo: rococo_runtime::Sudo,
			Balances: rococo_runtime::Balances,
		}
	}
}

// Declare Parachains
decl_test_parachains! {
	// Parachain A
	pub struct Trappist {
		genesis = para_a_genesis(),
		on_init = {trappist_runtime::AuraExt::on_initialize(1);}
		,
		runtime = trappist_runtime,
		core = {
			XcmpMessageHandler: trappist_runtime::XcmpQueue,
			DmpMessageHandler: trappist_runtime::DmpQueue,
			LocationToAccountId: trappist_runtime::xcm_config::LocationToAccountId,
			ParachainInfo: trappist_runtime::ParachainInfo,
		},
		pallets= {
			XcmPallet: trappist_runtime::PolkadotXcm,
			Assets: trappist_runtime::Assets,
			Sudo: trappist_runtime::Sudo,
			AssetRegistry: trappist_runtime::AssetRegistry,
			Balances: trappist_runtime::Balances,
		}
	},
	// Parachain B
	pub struct Stout {
		genesis = para_b_genesis(),
		on_init = {stout_runtime::AuraExt::on_initialize(1);},
		runtime = stout_runtime,
		core = {
			XcmpMessageHandler: stout_runtime::XcmpQueue,
			DmpMessageHandler: stout_runtime::DmpQueue,
			LocationToAccountId: stout_runtime::xcm_config::LocationToAccountId,
			ParachainInfo: stout_runtime::ParachainInfo,
		},
		pallets= {
			XcmPallet: stout_runtime::PolkadotXcm,
			Assets: stout_runtime::Assets,
			Sudo: stout_runtime::Sudo,
			AssetRegistry: stout_runtime::AssetRegistry,
			Balances: stout_runtime::Balances,
		}
	},

	// AssetHub
	pub struct AssetHubRococo {
		genesis = integration_tests_common::constants::asset_hub_kusama::genesis(),
		on_init = {
			asset_hub_polkadot_runtime::AuraExt::on_initialize(1);
		},
		runtime = asset_hub_kusama_runtime,
		core = {
			XcmpMessageHandler: asset_hub_kusama_runtime::XcmpQueue,
			DmpMessageHandler: asset_hub_kusama_runtime::DmpQueue,
			LocationToAccountId: asset_hub_kusama_runtime::xcm_config::LocationToAccountId,
			ParachainInfo: asset_hub_kusama_runtime::ParachainInfo,
		},
		pallets = {
			PolkadotXcm: asset_hub_kusama_runtime::PolkadotXcm,
			Assets: asset_hub_kusama_runtime::Assets,
			Balances: asset_hub_kusama_runtime::Balances,
		}
	},
}

//Define network(s)
decl_test_networks! {
	// Rococo
	pub struct RococoMockNet {
		relay_chain = Rococo,
		parachains = vec![Trappist, Stout, AssetHubRococo,],
		bridge = ()
	}
}

fn para_a_genesis() -> Storage {
	const PARA_ID: ParaId = ParaId::new(1836);
	const ED: Balance = trappist_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

	let genesis_config = trappist_runtime::RuntimeGenesisConfig {
		system: trappist_runtime::SystemConfig {
			code: trappist_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: trappist_runtime::BalancesConfig {
			balances: integration_tests_common::constants::accounts::init_balances()
				.iter()
				.cloned()
				.map(|k| (k, ED * 1_000_000_000))
				.collect(),
		},
		parachain_info: trappist_runtime::ParachainInfoConfig {
			parachain_id: PARA_ID,
			..Default::default()
		},
		collator_selection: trappist_runtime::CollatorSelectionConfig {
			invulnerables:
				integration_tests_common::constants::collators::invulnerables_asset_hub_polkadot()
					.iter()
					.cloned()
					.map(|(acc, _)| acc)
					.collect(),
			candidacy_bond: ED * 16,
			..Default::default()
		},
		session: trappist_runtime::SessionConfig {
			keys: integration_tests_common::constants::collators::invulnerables()
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                            // account id
						acc,                                    // validator id
						trappist_runtime::SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		sudo: trappist_runtime::SudoConfig {
			key: Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
		},
		polkadot_xcm: trappist_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(xcm::prelude::XCM_VERSION),
			..Default::default()
		},
		..Default::default()
	};

	genesis_config.build_storage().unwrap()
}

fn para_b_genesis() -> Storage {
	const PARA_ID: ParaId = ParaId::new(3000);
	const ED: Balance = stout_runtime::constants::currency::EXISTENTIAL_DEPOSIT;

	let genesis_config = stout_runtime::RuntimeGenesisConfig {
		system: stout_runtime::SystemConfig {
			code: stout_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
			..Default::default()
		},
		balances: stout_runtime::BalancesConfig {
			balances: integration_tests_common::constants::accounts::init_balances()
				.iter()
				.cloned()
				.map(|k| (k, ED * 1_000_000_000))
				.collect(),
		},
		parachain_info: stout_runtime::ParachainInfoConfig {
			parachain_id: PARA_ID,
			..Default::default()
		},
		collator_selection: stout_runtime::CollatorSelectionConfig {
			invulnerables:
				integration_tests_common::constants::collators::invulnerables_asset_hub_polkadot()
					.iter()
					.cloned()
					.map(|(acc, _)| acc)
					.collect(),
			candidacy_bond: ED * 16,
			..Default::default()
		},
		session: stout_runtime::SessionConfig {
			keys: integration_tests_common::constants::collators::invulnerables()
				.into_iter()
				.map(|(acc, aura)| {
					(
						acc.clone(),                         // account id
						acc,                                 // validator id
						stout_runtime::SessionKeys { aura }, // session keys
					)
				})
				.collect(),
		},
		sudo: stout_runtime::SudoConfig {
			key: Some(get_account_id_from_seed::<sr25519::Public>("Alice")),
		},
		polkadot_xcm: stout_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(xcm::prelude::XCM_VERSION),
			..Default::default()
		},
		..Default::default()
	};

	genesis_config.build_storage().unwrap()
}
