use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use trappist_runtime::{
	constants::currency::EXISTENTIAL_DEPOSIT, AccountId, AssetsConfig, AuraId, BalancesConfig,
	CouncilConfig, GenesisConfig, SessionConfig, SessionKeys, Signature, SudoConfig, SystemConfig,
};

const DEFAULT_PROTOCOL_ID: &str = "hop";

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<trappist_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

const TRAPPIST_PARA_ID: u32 = 1836;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
	/// The relay chain of the Parachain.
	pub relay_chain: String,
	/// The id of the Parachain.
	pub para_id: u32,
}

impl Extensions {
	/// Try to get the extension from the given `ChainSpec`.
	pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
		sc_chain_spec::get_extension(chain_spec.extensions())
	}
}

type AccountPublic = <Signature as Verify>::Signer;

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> AuraId {
	get_public_from_seed::<AuraId>(seed)
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

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
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
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
						get_collator_keys_from_seed("Alice"),
					),
					(
						get_account_id_from_seed::<sr25519::Public>("Bob"),
						get_collator_keys_from_seed("Bob"),
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

/// Configure initial storage state for FRAME modules.
fn testnet_genesis(
	invulnerables: Vec<(AccountId, AuraId)>,
	root_key: AccountId,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: trappist_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: trappist_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: trappist_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
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
		polkadot_xcm: trappist_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		assets: AssetsConfig { assets: vec![], accounts: vec![], metadata: vec![] },
		council: CouncilConfig {
			members: invulnerables.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
			phantom: Default::default(),
		},
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
) -> GenesisConfig {
	GenesisConfig {
		system: SystemConfig {
			code: trappist_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: BalancesConfig {
			balances: endowed_accounts
				.iter()
				.cloned()
				.chain(std::iter::once(root_key.clone()))
				.map(|k| {
					if k == root_key {
						(k, 1_000_000_000_000_000_000)
					} else {
						(k, 1_500_000_000_000_000_000)
					}
				})
				.collect(),
		},
		parachain_info: trappist_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: trappist_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
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
		aura: Default::default(),
		aura_ext: Default::default(),
		parachain_system: Default::default(),
		polkadot_xcm: trappist_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		assets: AssetsConfig { assets: vec![], accounts: vec![], metadata: vec![] },
		council: CouncilConfig {
			// We set the endowed accounts with balance as members of the council.
			members: endowed_accounts.iter().map(|x| x.clone()).collect::<Vec<_>>(),
			phantom: Default::default(),
		},
	}
}
