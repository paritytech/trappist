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

use std::{net::SocketAddr, path::PathBuf};

use cumulus_client_cli::generate_genesis_block;
use cumulus_primitives_core::ParaId;
use frame_benchmarking_cli::{BenchmarkCmd, SUBSTRATE_REFERENCE_HARDWARE};
use log::info;
use parachains_common::AuraId;
use parity_scale_codec::Encode;
use sc_cli::{
	ChainSpec, CliConfiguration, DefaultConfigurationValues, ImportParams, KeystoreParams,
	NetworkParams, Result, SharedParams, SubstrateCli,
};
use sc_service::config::{BasePath, PrometheusConfig};
use sp_core::hexdisplay::HexDisplay;
use sp_runtime::traits::{AccountIdConversion, Block as BlockT};

use crate::{
	benchmarking::{inherent_benchmark_data, RemarkBuilder},
	chain_spec,
	cli::{Cli, RelayChainCli, Subcommand},
	service::{new_partial, Block},
};

/// Dispatches the code to the currently selected runtime.
macro_rules! dispatch_runtime {
	($runtime:expr, |$alias: ident| $code:expr) => {
		match $runtime {
			#[cfg(feature = "trappist-runtime")]
			Runtime::Trappist => {
				#[allow(unused_imports)]
				use trappist_runtime as $alias;

				$code
			},
			#[cfg(feature = "stout-runtime")]
			Runtime::Stout => {
				#[allow(unused_imports)]
				use stout_runtime as $alias;

				$code
			},
		}
	};
	($runtime:expr, $code:expr) => {
		dispatch_runtime!($runtime, |rt| $code)
	};
}

/// Generates boilerplate code for constructing partial node for the runtimes that are supported
/// by the benchmarks.
macro_rules! construct_partial {
	($config:expr, |$partial:ident, $runtime:ident| $code:expr) => {
		dispatch_runtime!($config.chain_spec.runtime(), |$runtime| {
			let $partial = new_partial::<$runtime::RuntimeApi, _>(
				&$config,
				crate::service::aura_build_import_queue::<_, AuraId>,
			)?;

			$code
		})
	};
	($config:expr, |$partial:ident| $code:expr) => {
		construct_partial!($config, |$partial, rt| $code)
	};
}

/// Generates boilerplate code for async run on partial node.
macro_rules! construct_async_run {
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident, $runtime:ident| $( $code:tt )* ) => {{
		let runner = $cli.create_runner($cmd)?;
		construct_partial!(runner.config(), |$components, $runtime| {
			runner.async_run(|$config| {
				let task_manager = $components.task_manager;
				{ $( $code )* }.map(|v| (v, task_manager))
			})
		})
	}};
	(|$components:ident, $cli:ident, $cmd:ident, $config:ident| $( $code:tt )* ) => {{
		construct_async_run!(|$components, $cli, $cmd, $config, rt| { $( $code )* })
	}};
}

/// Helper enum that is used for better distinction of different parachain/runtime configuration
/// (it is based/calculated on ChainSpec's ID attribute)
#[derive(Debug, PartialEq)]
enum Runtime {
	#[cfg(feature = "trappist-runtime")]
	Trappist,
	#[cfg(feature = "stout-runtime")]
	Stout,
}

#[cfg(feature = "trappist-runtime")]
impl Default for Runtime {
	fn default() -> Self {
		Runtime::Trappist
	}
}

#[cfg(all(feature = "stout-runtime", not(feature = "trappist-runtime")))]
impl Default for Runtime {
	fn default() -> Self {
		Runtime::Stout
	}
}

impl From<&str> for Runtime {
	fn from(value: &str) -> Self {
		#[cfg(feature = "trappist-runtime")]
		if value.starts_with("trappist") {
			return Runtime::Trappist;
		}

		#[cfg(feature = "stout-runtime")]
		if value.starts_with("stout") {
			return Runtime::Stout;
		}

		let fallback = Runtime::default();
		log::warn!("No specific runtime was recognized for ChainSpec's id: '{value}', so `{fallback:?}` will be used as default.");
		fallback
	}
}

trait RuntimeResolver {
	fn runtime(&self) -> Runtime;
}

impl RuntimeResolver for dyn ChainSpec {
	fn runtime(&self) -> Runtime {
		self.id().into()
	}
}

/// Implementation, that can resolve [`Runtime`] from any json configuration file
impl RuntimeResolver for PathBuf {
	fn runtime(&self) -> Runtime {
		#[derive(Debug, serde::Deserialize)]
		struct EmptyChainSpecWithId {
			id: String,
		}

		let file = std::fs::File::open(self).expect("Failed to open file");
		let reader = std::io::BufReader::new(file);
		let chain_spec: EmptyChainSpecWithId = serde_json::from_reader(reader)
			.expect("Failed to read 'json' file with ChainSpec configuration");

		chain_spec.id.as_str().into()
	}
}

fn load_spec(id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
	Ok(match id {
		#[cfg(feature = "trappist-runtime")]
		"dev" | "trappist-dev" => Box::new(chain_spec::trappist::development_config()),
		#[cfg(feature = "trappist-runtime")]
		"trappist-local" => Box::new(chain_spec::trappist::trappist_local_testnet_config()),
		// Live chain spec for Rococo - Trappist
		#[cfg(feature = "trappist-runtime")]
		"" | "trappist-rococo" => Box::new(chain_spec::trappist::trappist_live_config()),
		#[cfg(feature = "stout-runtime")]
		"stout-dev" => unimplemented!("stout-dev chain spec is not available yet"),
		#[cfg(feature = "stout-runtime")]
		"stout-local" => Box::new(chain_spec::stout::stout_local_testnet_config()),
		// -- Loading a specific spec from disk
		path => {
			let path: PathBuf = path.into();
			match path.runtime() {
				#[cfg(feature = "trappist-runtime")]
				Runtime::Trappist => Box::new(chain_spec::trappist::ChainSpec::from_json_file(path)?),
				#[cfg(feature = "stout-runtime")]
				Runtime::Stout => Box::new(chain_spec::stout::ChainSpec::from_json_file(path)?),
			}
		},
	})
}

impl SubstrateCli for Cli {
	fn impl_name() -> String {
		"Trappist Node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Trappist collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/TrappistNetwork/trappist/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2021
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		load_spec(id)
	}
}

impl SubstrateCli for RelayChainCli {
	fn impl_name() -> String {
		"Trappist node".into()
	}

	fn impl_version() -> String {
		env!("SUBSTRATE_CLI_IMPL_VERSION").into()
	}

	fn description() -> String {
		format!(
			"Trappist collator\n\nThe command-line arguments provided first will be \
		passed to the parachain node, while the arguments provided after -- will be passed \
		to the relay chain node.\n\n\
		{} <parachain-args> -- <relay-chain-args>",
			Self::executable_name()
		)
	}

	fn author() -> String {
		env!("CARGO_PKG_AUTHORS").into()
	}

	fn support_url() -> String {
		"https://github.com/TrappistNetwork/trappist/issues/new".into()
	}

	fn copyright_start_year() -> i32 {
		2021
	}

	fn load_spec(&self, id: &str) -> std::result::Result<Box<dyn ChainSpec>, String> {
		polkadot_cli::Cli::from_iter([RelayChainCli::executable_name()].iter()).load_spec(id)
	}
}

/// Parse command line arguments into service configuration.
pub fn run() -> Result<()> {
	let cli = Cli::from_args();

	match &cli.subcommand {
		Some(Subcommand::BuildSpec(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| cmd.run(config.chain_spec, config.network))
		},
		Some(Subcommand::CheckBlock(cmd)) => {
			construct_async_run!(|components, cli, cmd, _config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::ExportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.database))
			})
		},
		Some(Subcommand::ExportState(cmd)) => {
			construct_async_run!(|components, cli, cmd, config| {
				Ok(cmd.run(components.client, config.chain_spec))
			})
		},
		Some(Subcommand::ImportBlocks(cmd)) => {
			construct_async_run!(|components, cli, cmd, _config| {
				Ok(cmd.run(components.client, components.import_queue))
			})
		},
		Some(Subcommand::Revert(cmd)) => construct_async_run!(|components, cli, cmd, _config| {
			Ok(cmd.run(components.client, components.backend, None))
		}),
		Some(Subcommand::PurgeChain(cmd)) => {
			let runner = cli.create_runner(cmd)?;

			runner.sync_run(|config| {
				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relaychain_args.iter()),
				);

				let polkadot_config = SubstrateCli::create_configuration(
					&polkadot_cli,
					&polkadot_cli,
					config.tokio_handle.clone(),
				)
				.map_err(|err| format!("Relay chain argument error: {err}"))?;

				cmd.run(config, polkadot_config)
			})
		},
		Some(Subcommand::ExportGenesisState(cmd)) => {
			construct_async_run!(|components, cli, cmd, _config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				Ok(async move { cmd.run::<crate::service::Block>(&*spec, &*components.client) })
			})
		},
		Some(Subcommand::ExportGenesisWasm(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|_config| {
				let spec = cli.load_spec(&cmd.shared_params.chain.clone().unwrap_or_default())?;
				cmd.run(&*spec)
			})
		},
		Some(Subcommand::Benchmark(cmd)) => {
			let runner = cli.create_runner(cmd)?;
			runner.sync_run(|config| {
				// Switch on the concrete benchmark sub-command
				match cmd {
					BenchmarkCmd::Pallet(cmd) => {
						if !cfg!(feature = "runtime-benchmarks") {
							return Err("Benchmarking wasn't enabled when building the node. \
							You can enable it with `--features runtime-benchmarks`."
								.into());
						}

						dispatch_runtime!(config.chain_spec.runtime(), |runtime| {
							cmd.run::<Block, ()>(config)
						})
					},
					BenchmarkCmd::Block(cmd) => {
						construct_partial!(config, |partial| cmd.run(partial.client))
					},
					#[cfg(not(feature = "runtime-benchmarks"))]
					BenchmarkCmd::Storage(_) => Err(sc_cli::Error::Input(
						"Compile with --features=runtime-benchmarks \
						to enable storage benchmarks."
							.into(),
					)),
					#[cfg(feature = "runtime-benchmarks")]
					BenchmarkCmd::Storage(cmd) => {
						construct_partial!(config, |partial| {
							let db = partial.backend.expose_db();
							let storage = partial.backend.expose_storage();

							cmd.run(config, partial.client, db, storage)
						})
					},
					BenchmarkCmd::Machine(cmd) => {
						cmd.run(&config, SUBSTRATE_REFERENCE_HARDWARE.clone())
					},
					BenchmarkCmd::Overhead(cmd) => {
						construct_partial!(config, |partial| {
							let ext_builder = RemarkBuilder::new(partial.client.clone());

							cmd.run(
								config,
								partial.client,
								inherent_benchmark_data()?,
								Vec::new(),
								&ext_builder,
							)
						})
					},
					// NOTE: this allows the Client to leniently implement
					// new benchmark commands without requiring a companion MR.
					#[allow(unreachable_patterns)]
					_ => Err("Benchmarking sub-command unsupported".into()),
				}
			})
		},
		Some(Subcommand::Key(cmd)) => Ok(cmd.run(&cli)?),
		None => {
			let runner = cli.create_runner(&cli.run.normalize())?;
			let collator_options = cli.run.collator_options();

			runner.run_node_until_exit(|config| async move {
				let hwbench = if !cli.no_hardware_benchmarks {
					config.database.path().map(|database_path| {
						let _ = std::fs::create_dir_all(database_path);
						sc_sysinfo::gather_hwbench(Some(database_path))
					})
				} else {
					None
				};

				let para_id = chain_spec::Extensions::try_get(&*config.chain_spec)
					.map(|e| e.para_id)
					.ok_or("Could not find parachain extension in chain-spec.")?;

				let polkadot_cli = RelayChainCli::new(
					&config,
					[RelayChainCli::executable_name()].iter().chain(cli.relaychain_args.iter()),
				);

				let id = ParaId::from(para_id);

				let parachain_account =
					AccountIdConversion::<polkadot_primitives::AccountId>::into_account_truncating(
						&id,
					);

				let block: crate::service::Block =
					generate_genesis_block(&*config.chain_spec, sp_runtime::StateVersion::V1)
						.map_err(|e| format!("{e:?}"))?;
				let genesis_state = format!("0x{:?}", HexDisplay::from(&block.header().encode()));

				let tokio_handle = config.tokio_handle.clone();
				let polkadot_config =
					SubstrateCli::create_configuration(&polkadot_cli, &polkadot_cli, tokio_handle)
						.map_err(|err| format!("Relay chain argument error: {err}"))?;

				info!("Parachain id: {:?}", id);
				info!("Parachain Account: {}", parachain_account);
				info!("Parachain genesis state: {}", genesis_state);
				info!("Is collating: {}", if config.role.is_authority() { "yes" } else { "no" });

				dispatch_runtime!(config.chain_spec.runtime(), |runtime| {
					crate::service::start_aura_node::<runtime::RuntimeApi, AuraId>(
						config,
						polkadot_config,
						collator_options,
						id,
						hwbench,
					)
					.await
					.map(|r| r.0)
					.map_err(Into::into)
				})
			})
		},
	}
}

impl DefaultConfigurationValues for RelayChainCli {
	fn p2p_listen_port() -> u16 {
		30334
	}

	fn rpc_listen_port() -> u16 {
		9945
	}

	fn prometheus_listen_port() -> u16 {
		9616
	}
}

impl CliConfiguration<Self> for RelayChainCli {
	fn shared_params(&self) -> &SharedParams {
		self.base.base.shared_params()
	}

	fn import_params(&self) -> Option<&ImportParams> {
		self.base.base.import_params()
	}

	fn network_params(&self) -> Option<&NetworkParams> {
		self.base.base.network_params()
	}

	fn keystore_params(&self) -> Option<&KeystoreParams> {
		self.base.base.keystore_params()
	}

	fn base_path(&self) -> Result<Option<BasePath>> {
		Ok(self
			.shared_params()
			.base_path()?
			.or_else(|| self.base_path.clone().map(Into::into)))
	}

	fn rpc_addr(&self, default_listen_port: u16) -> Result<Option<SocketAddr>> {
		self.base.base.rpc_addr(default_listen_port)
	}

	fn prometheus_config(
		&self,
		default_listen_port: u16,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<PrometheusConfig>> {
		self.base.base.prometheus_config(default_listen_port, chain_spec)
	}

	fn init<F>(
		&self,
		_support_url: &String,
		_impl_version: &String,
		_logger_hook: F,
		_config: &sc_service::Configuration,
	) -> Result<()>
	where
		F: FnOnce(&mut sc_cli::LoggerBuilder, &sc_service::Configuration),
	{
		unreachable!("PolkadotCli is never initialized; qed");
	}

	fn chain_id(&self, is_dev: bool) -> Result<String> {
		let chain_id = self.base.base.chain_id(is_dev)?;

		Ok(if chain_id.is_empty() { self.chain_id.clone().unwrap_or_default() } else { chain_id })
	}

	fn role(&self, is_dev: bool) -> Result<sc_service::Role> {
		self.base.base.role(is_dev)
	}

	fn transaction_pool(&self, is_dev: bool) -> Result<sc_service::config::TransactionPoolOptions> {
		self.base.base.transaction_pool(is_dev)
	}

	fn trie_cache_maximum_size(&self) -> Result<Option<usize>> {
		self.base.base.trie_cache_maximum_size()
	}

	fn rpc_methods(&self) -> Result<sc_service::config::RpcMethods> {
		self.base.base.rpc_methods()
	}

	fn rpc_max_connections(&self) -> Result<u32> {
		self.base.base.rpc_max_connections()
	}

	fn rpc_cors(&self, is_dev: bool) -> Result<Option<Vec<String>>> {
		self.base.base.rpc_cors(is_dev)
	}

	fn default_heap_pages(&self) -> Result<Option<u64>> {
		self.base.base.default_heap_pages()
	}

	fn force_authoring(&self) -> Result<bool> {
		self.base.base.force_authoring()
	}

	fn disable_grandpa(&self) -> Result<bool> {
		self.base.base.disable_grandpa()
	}

	fn max_runtime_instances(&self) -> Result<Option<usize>> {
		self.base.base.max_runtime_instances()
	}

	fn announce_block(&self) -> Result<bool> {
		self.base.base.announce_block()
	}

	fn telemetry_endpoints(
		&self,
		chain_spec: &Box<dyn ChainSpec>,
	) -> Result<Option<sc_telemetry::TelemetryEndpoints>> {
		self.base.base.telemetry_endpoints(chain_spec)
	}

	fn node_name(&self) -> Result<String> {
		self.base.base.node_name()
	}
}

#[cfg(test)]
mod tests {
	use std::path::PathBuf;

	use cumulus_primitives_core::ParaId;
	use parachains_common::{AccountId, AuraId};
	use sc_chain_spec::{
		ChainSpec, ChainSpecExtension, ChainSpecGroup, ChainType, Extension, GenericChainSpec,
	};
	use serde::{Deserialize, Serialize};
	use sp_core::sr25519;
	use tempfile::TempDir;

	use crate::{
		chain_spec::{get_account_id_from_seed, get_collator_keys_from_seed},
		command::{Runtime, RuntimeResolver},
	};

	#[derive(
		Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension, Default,
	)]
	#[serde(deny_unknown_fields)]
	pub struct Extensions1 {
		pub attribute1: String,
		pub attribute2: u32,
	}

	#[derive(
		Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension, Default,
	)]
	#[serde(deny_unknown_fields)]
	pub struct Extensions2 {
		pub attribute_x: String,
		pub attribute_y: String,
		pub attribute_z: u32,
	}

	pub fn create_default_with_extensions<G: 'static + Send + Sync, E: Extension>(
		id: &str,
		extension: E,
		constructor: fn(Vec<(AccountId, AuraId)>, AccountId, Vec<AccountId>, ParaId) -> G,
	) -> GenericChainSpec<G, E> {
		GenericChainSpec::<G, E>::from_genesis(
			"Dummy local testnet",
			id,
			ChainType::Local,
			move || {
				constructor(
					vec![(
						get_account_id_from_seed::<sr25519::Public>("Alice"),
						get_collator_keys_from_seed::<AuraId>("Alice"),
					)],
					get_account_id_from_seed::<sr25519::Public>("Alice"),
					Vec::new(),
					1000.into(),
				)
			},
			Vec::new(),
			None,
			None,
			None,
			None,
			extension,
		)
	}

	fn assert_resolved_runtime(runtime: Runtime, specs: Vec<Box<dyn ChainSpec>>) {
		fn store_configuration(dir: &TempDir, spec: Box<dyn ChainSpec>) -> PathBuf {
			let raw_output = true;
			let json = sc_service::chain_ops::build_spec(&*spec, raw_output)
				.expect("Failed to build json string");
			let mut cfg_file_path = dir.path().to_path_buf();
			cfg_file_path.push(spec.id());
			cfg_file_path.set_extension("json");
			std::fs::write(&cfg_file_path, json).expect("Failed to write to json file");
			cfg_file_path
		}

		let temp_dir = tempfile::tempdir().expect("Failed to access tempdir");

		specs.into_iter().for_each(|spec| {
			let path = store_configuration(&temp_dir, spec);
			assert_eq!(runtime, path.runtime());
		});
	}

	#[test]
	#[cfg(feature = "trappist-runtime")]
	fn test_resolve_trappist_runtime_for_different_configuration_files() {
		assert_resolved_runtime(
			Runtime::Trappist,
			vec![
				Box::new(
					create_default_with_extensions::<trappist_runtime::RuntimeGenesisConfig, _>(
						"trappist-1",
						Extensions1::default(),
						crate::chain_spec::trappist::testnet_genesis,
					),
				),
				Box::new(
					create_default_with_extensions::<trappist_runtime::RuntimeGenesisConfig, _>(
						"trappist-2",
						Extensions2::default(),
						crate::chain_spec::trappist::testnet_genesis,
					),
				),
				Box::new(crate::chain_spec::trappist::trappist_local_testnet_config()),
			],
		)
	}

	#[test]
	#[cfg(feature = "stout-runtime")]
	fn test_resolve_stout_runtime_for_different_configuration_files() {
		assert_resolved_runtime(
			Runtime::Stout,
			vec![
				Box::new(create_default_with_extensions(
					"stout-1",
					Extensions1::default(),
					crate::chain_spec::stout::testnet_genesis,
				)),
				Box::new(create_default_with_extensions(
					"stout-2",
					Extensions2::default(),
					crate::chain_spec::stout::testnet_genesis,
				)),
			],
		)
	}
}
