use cumulus_primitives_core::ParaId;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};
use base_runtime::{
	constants::currency::EXISTENTIAL_DEPOSIT, AccountId, AssetsConfig, AuraId, BalancesConfig,
	CouncilConfig, GenesisConfig, SessionConfig, SessionKeys, Signature, SudoConfig,
	SystemConfig,
};

const DEFAULT_PROTOCOL_ID: &str = "base";

const ALICE: &str = "Alice";
const BOB: &str = "Bob";
const CHARLIE: &str = "Charlie";
const DAVE: &str = "Dave";
const EVE: &str = "Eve";
const FERDIE: &str = "Ferdie";

const RELAY_CHAIN_NAME: &str = "rococo-local";

const PARACHAIN_ID: u32 = 3000;

const BST_TOKEN_SYMBOL: &str = "BST";
const BST_TOKEN_DECIMALS: u32 = 12;
const SS58_FORMAT: u32 = 42;

const DEV_CHAIN_NAME: &str = "Base Development";
const DEV_CHAIN_ID: &str = "base_dev";

const TESTNET_CHAIN_NAME: &str = "Base Local";
const TESTNET_CHAIN_ID: &str = "base_local";

const B_USD_ASSET_ID: u32 = 1;
const B_USD_INITIAL_BALANCE: u128 = 1_000_000_000_000_000;
const B_USD_IS_SUFFICIENT: bool = true;
const B_USD_MIN_BALANCE: u128 = 1_000_000;
const B_USD_TOKEN_DECIMALS: u8 = 12;
const B_USD_TOKEN_NAME: &str = "bUSD";
const B_USD_TOKEN_SYMBOL: &str = "bUSD";

const LIQUIDITY_TOKEN_ID: u32 = 101;
const EXCHANGE_TOKEN_AMOUNT: u128 = 100_000_000_000_000;

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec = sc_service::GenericChainSpec<base_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

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
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), BST_TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), BST_TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());

	ChainSpec::from_genesis(
		// Name
		DEV_CHAIN_NAME,
		// ID
		DEV_CHAIN_ID,
		ChainType::Development,
		move || {
			testnet_genesis(
				// Initial collators.
				vec![
					(get_account_id(ALICE), get_collator_keys_from_seed(ALICE)),
					(get_account_id(BOB), get_collator_keys_from_seed(BOB)),
				],
				// Sudo account
				get_account_id(ALICE),
				// Pre-funded accounts
				vec![get_account_id(ALICE), get_account_id(BOB), get_account_id(CHARLIE)],
				PARACHAIN_ID.into(),
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
			relay_chain: RELAY_CHAIN_NAME.into(), // You MUST set this to the correct network!
			para_id: PARACHAIN_ID,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), BST_TOKEN_SYMBOL.into());
	properties.insert("tokenDecimals".into(), BST_TOKEN_DECIMALS.into());
	properties.insert("ss58Format".into(), SS58_FORMAT.into());

	ChainSpec::from_genesis(
		// Name
		TESTNET_CHAIN_NAME,
		// ID
		TESTNET_CHAIN_ID,
		ChainType::Local,
		move || {
			testnet_genesis(
				// Initial collators.
				vec![
					(get_account_id(ALICE), get_collator_keys_from_seed(ALICE)),
					(get_account_id(BOB), get_collator_keys_from_seed(BOB)),
				],
				// Sudo account
				get_account_id(ALICE),
				// Pre-funded accounts
				vec![
					get_account_id(ALICE),
					get_account_id(BOB),
					get_account_id(CHARLIE),
					get_account_id(DAVE),
					get_account_id(EVE),
					get_account_id(FERDIE),
				],
				PARACHAIN_ID.into(),
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
			relay_chain: RELAY_CHAIN_NAME.into(), // You MUST set this to the correct network!
			para_id: PARACHAIN_ID,
		},
	)
}

fn get_account_id(name: &str) -> AccountId {
	get_account_id_from_seed::<sr25519::Public>(name)
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
			code: base_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: BalancesConfig {
			// Configure endowed accounts with initial balance of 1 << 60.
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: base_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: base_runtime::CollatorSelectionConfig {
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
		polkadot_xcm: base_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		sudo: SudoConfig {
			// Assign network admin rights.
			key: Some(root_key),
		},
		assets: AssetsConfig {
			assets: vec![(
				B_USD_ASSET_ID,
				get_account_id(ALICE),
				B_USD_IS_SUFFICIENT,
				B_USD_MIN_BALANCE,
			)],
			metadata: vec![(
				B_USD_ASSET_ID,
				B_USD_TOKEN_NAME.into(),
				B_USD_TOKEN_SYMBOL.into(),
				B_USD_TOKEN_DECIMALS,
			)],
			accounts: get_initialized_accounts(
				B_USD_ASSET_ID,
				B_USD_INITIAL_BALANCE,
				vec![
					get_account_id(ALICE),
					get_account_id(BOB),
					get_account_id(CHARLIE),
					get_account_id(DAVE),
					get_account_id(FERDIE),
					get_account_id(EVE),
				],
			),
		},
		council: CouncilConfig {
			members: invulnerables.iter().map(|x| x.0.clone()).collect::<Vec<_>>(),
			phantom: Default::default(),
		},
		// dex: DexConfig {
		// 	exchanges: vec![(
		// 		get_account_id(BOB),
		// 		B_USD_ASSET_ID.into(),
		// 		LIQUIDITY_TOKEN_ID.into(),
		// 		EXCHANGE_TOKEN_AMOUNT.into(),
		// 		EXCHANGE_TOKEN_AMOUNT.into(),
		// 	)],
		// },
	}
}

fn get_initialized_accounts(
	asset_id: u32,
	initial_balance: u128,
	accounts: Vec<AccountId>,
) -> Vec<(u32, AccountId, u128)> {
	accounts
		.iter()
		.map(|account| (asset_id, account.clone(), initial_balance))
		.collect::<Vec<_>>()
}
