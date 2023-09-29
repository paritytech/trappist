use frame_support::{sp_io, sp_tracing};
use integration_tests_common::{AccountId, Balance};
use sp_core::{sr25519, storage::Storage, Get};
use sp_runtime::BuildStorage;
use xcm_emulator::{
	bx, decl_test_networks, decl_test_parachains, decl_test_relay_chains, get_account_id_from_seed,
	Ancestor, BridgeMessageHandler, MultiLocation, ParaId, Parachain, Parent, RelayChain, TestExt,
	XcmHash, X1,
};
use xcm_executor::traits::ConvertLocation;

#[cfg(test)]
mod tests;

// PDD: relay chains
decl_test_relay_chains! {
	// Polkadot
	#[api_version(5)]
	pub struct Polkadot {
		genesis = integration_tests_common::constants::polkadot::genesis(),
		on_init = (),
		// PDD: actual Polkadot runtime
		runtime = {
			Runtime: polkadot_runtime::Runtime,
			RuntimeOrigin: polkadot_runtime::RuntimeOrigin,
			RuntimeCall: polkadot_runtime::RuntimeCall,
			RuntimeEvent: polkadot_runtime::RuntimeEvent,
			MessageQueue: polkadot_runtime::MessageQueue,
			XcmConfig: polkadot_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: polkadot_runtime::xcm_config::SovereignAccountOf,
			System: polkadot_runtime::System,
			Balances: polkadot_runtime::Balances,
		},
		// PDD: pallet type helper
		pallets_extra = {}
	},
	// Kusama
	#[api_version(5)]
	pub struct Kusama {
		genesis = integration_tests_common::constants::kusama::genesis(),
		on_init = (),
		runtime = {
			Runtime: kusama_runtime::Runtime,
			RuntimeOrigin: kusama_runtime::RuntimeOrigin,
			RuntimeCall: kusama_runtime::RuntimeCall,
			RuntimeEvent: kusama_runtime::RuntimeEvent,
			MessageQueue: kusama_runtime::MessageQueue,
			XcmConfig: kusama_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: kusama_runtime::xcm_config::SovereignAccountOf,
			System: kusama_runtime::System,
			Balances: kusama_runtime::Balances,
		},
		pallets_extra = {}
	},
	// Rococo
	#[api_version(5)]
	pub struct Rococo {
		genesis = integration_tests_common::constants::rococo::genesis(),
		on_init = (),
		runtime = {
			Runtime: rococo_runtime::Runtime,
			RuntimeOrigin: rococo_runtime::RuntimeOrigin,
			RuntimeCall: rococo_runtime::RuntimeCall,
			RuntimeEvent: rococo_runtime::RuntimeEvent,
			MessageQueue: rococo_runtime::MessageQueue,
			XcmConfig: rococo_runtime::xcm_config::XcmConfig,
			SovereignAccountOf: rococo_runtime::xcm_config::LocationConverter,
			System: rococo_runtime::System,
			Balances: rococo_runtime::Balances,
		},
		pallets_extra = {
			XcmPallet: rococo_runtime::XcmPallet,
			Sudo: rococo_runtime::Sudo,
		}
	}
}

// PDD: parachains
decl_test_parachains! {
	// Parachain A
	pub struct ParaA {
		// PDD: genesis config
		genesis = para_a_genesis(),
		on_init = (),
		// PDD: actual parachain runtime
		runtime = {
			Runtime: trappist_runtime::Runtime,
			RuntimeOrigin: trappist_runtime::RuntimeOrigin,
			RuntimeCall: trappist_runtime::RuntimeCall,
			RuntimeEvent: trappist_runtime::RuntimeEvent,
			XcmpMessageHandler: trappist_runtime::XcmpQueue,
			DmpMessageHandler: trappist_runtime::DmpQueue,
			LocationToAccountId: trappist_runtime::xcm_config::LocationToAccountId,
			System: trappist_runtime::System,
			Balances: trappist_runtime::Balances,
			ParachainSystem: trappist_runtime::ParachainSystem,
			ParachainInfo: trappist_runtime::ParachainInfo,
		},
		pallets_extra = {
			XcmPallet: trappist_runtime::PolkadotXcm,
			Assets: trappist_runtime::Assets,
			Sudo: trappist_runtime::Sudo,
			AssetRegistry: trappist_runtime::AssetRegistry,
		}
	},
	// Parachain B
	pub struct ParaB {
		genesis = para_b_genesis(),
		on_init = (),
		runtime = {
			Runtime: stout_runtime::Runtime,
			RuntimeOrigin: stout_runtime::RuntimeOrigin,
			RuntimeCall: stout_runtime::RuntimeCall,
			RuntimeEvent: stout_runtime::RuntimeEvent,
			XcmpMessageHandler: stout_runtime::XcmpQueue,
			DmpMessageHandler: stout_runtime::DmpQueue,
			LocationToAccountId: stout_runtime::xcm_config::LocationToAccountId,
			System: stout_runtime::System,
			Balances: stout_runtime::Balances,
			ParachainSystem: stout_runtime::ParachainSystem,
			ParachainInfo: stout_runtime::ParachainInfo,
		},
		pallets_extra = {}
	},

	// AssetHub
	pub struct AssetHubRococo {
		genesis = integration_tests_common::constants::asset_hub_polkadot::genesis(),
		on_init = (),
		runtime = {
			Runtime: asset_hub_polkadot_runtime::Runtime,
			RuntimeOrigin: asset_hub_polkadot_runtime::RuntimeOrigin,
			RuntimeCall: asset_hub_polkadot_runtime::RuntimeCall,
			RuntimeEvent: asset_hub_polkadot_runtime::RuntimeEvent,
			XcmpMessageHandler: asset_hub_polkadot_runtime::XcmpQueue,
			DmpMessageHandler: asset_hub_polkadot_runtime::DmpQueue,
			LocationToAccountId: asset_hub_polkadot_runtime::xcm_config::LocationToAccountId,
			System: asset_hub_polkadot_runtime::System,
			Balances: asset_hub_polkadot_runtime::Balances,
			ParachainSystem: asset_hub_polkadot_runtime::ParachainSystem,
			ParachainInfo: asset_hub_polkadot_runtime::ParachainInfo,
		},
		pallets_extra = {
			PolkadotXcm: asset_hub_polkadot_runtime::PolkadotXcm,
			Assets: asset_hub_polkadot_runtime::Assets,
		}
	}
}

// PDD: define network(s)
decl_test_networks! {
	// Polkadot
	pub struct PolkadotMockNet {
		relay_chain = Polkadot,
		parachains = vec![  ],
		bridge = ()
	},
	// Kusama
	pub struct KusamaMockNet {
		relay_chain = Kusama,
		parachains = vec![],
		bridge = ()
	},
	// Rococo
	pub struct RococoMockNet {
		relay_chain = Rococo,
		parachains = vec![ParaA, ParaB, AssetHubRococo,],
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
				.map(|k| (k, ED * 4096))
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
		polkadot_xcm: stout_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(xcm::prelude::XCM_VERSION),
			..Default::default()
		},
		..Default::default()
	};

	genesis_config.build_storage().unwrap()
}
