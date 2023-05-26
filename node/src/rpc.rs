// Copyright 2021 Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Parachain-specific RPCs implementation.

#![warn(missing_docs)]

use std::sync::Arc;

use parachains_common::{AccountId, Balance, Block, Index as Nonce};
use sc_client_api::AuxStore;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Full client dependencies
pub struct FullDeps<C, P> {
	/// The client instance to use.
	pub client: Arc<C>,
	/// Transaction pool instance.
	pub pool: Arc<P>,
	/// Whether to deny unsafe calls
	pub deny_unsafe: DenyUnsafe,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, B>(
	deps: FullDeps<C, P>,
	backend: Arc<B>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static,
	C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	C::Api: pallet_dex_rpc::DexRuntimeApi<
		trappist_runtime::opaque::Block,
		trappist_runtime::AssetId,
		trappist_runtime::Balance,
		trappist_runtime::AssetBalance,
	>,
	P: TransactionPool + Sync + Send + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	use frame_rpc_system::{System, SystemApiServer};
	use pallet_dex_rpc::{Dex, DexApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

	let mut module = RpcExtension::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;
	module.merge(Dex::new(client).into_rpc())?;

	Ok(module)
}

/* pub trait ClientRequiredTraits {
	type Client: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static;
	type Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ BlockBuilder<Block>;
}

impl<C> ClientRequiredTraits for C
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static,
	C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>
		+ pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>
		+ BlockBuilder<Block>,
{
	type Client = C;
	type Api = C::Api;
}

pub trait PoolRequiredTraits {
	type Pool: TransactionPool + Sync + Send + 'static;
}

impl<T> PoolRequiredTraits for T
where
	T: TransactionPool + Sync + Send + 'static,
{
	type Pool = T;
}

pub trait BackendRequiredTraits {
	type Backend: sc_client_api::Backend<Block> + Send + Sync + 'static;
	type State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>;
}

impl<T> BackendRequiredTraits for T
where
	T: sc_client_api::Backend<Block> + Send + Sync + 'static,
	T::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	type Backend = T;
	type State = T::State;
}

pub fn create_stout_full_bck<C, P, B>(
	deps: FullDeps<C, P>,
	backend: Arc<B>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ClientRequiredTraits,
	P: PoolRequiredTraits,
	B: BackendRequiredTraits,
{
	use frame_rpc_system::{System, SystemApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

	let mut module = RpcExtension::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;

	Ok(module)
} */

pub fn create_stout_full<C, P, B>(
	deps: FullDeps<C, P>,
	backend: Arc<B>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
	C: ProvideRuntimeApi<Block>
		+ HeaderBackend<Block>
		+ AuxStore
		+ HeaderMetadata<Block, Error = BlockChainError>
		+ Send
		+ Sync
		+ 'static,
	C::Api: frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
	C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
	C::Api: BlockBuilder<Block>,
	P: TransactionPool + Sync + Send + 'static,
	B: sc_client_api::Backend<Block> + Send + Sync + 'static,
	B::State: sc_client_api::backend::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
	use frame_rpc_system::{System, SystemApiServer};
	use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
	use substrate_state_trie_migration_rpc::{StateMigration, StateMigrationApiServer};

	let mut module = RpcExtension::new(());
	let FullDeps { client, pool, deny_unsafe } = deps;

	module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
	module.merge(TransactionPayment::new(client.clone()).into_rpc())?;
	module.merge(StateMigration::new(client.clone(), backend, deny_unsafe).into_rpc())?;

	Ok(module)
}
