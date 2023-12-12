// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate fork-aware transaction pool implementation.

//todo:
#![allow(missing_docs)]
#![warn(unused_extern_crates)]
//todo:
#![allow(unused_imports)]
//todo:
#![allow(unused_variables)]
#![allow(dead_code)]

// todo: remove:
// This is cleaned copy of src/lib.rs.

use crate::graph;
pub use crate::{
	api::FullChainApi,
	graph::{
		base_pool::Limit as PoolLimit, watcher::Watcher, ChainApi, Options, Pool, Transaction,
		ValidatedTransaction,
	},
};
use async_trait::async_trait;
use futures::{
	channel::oneshot,
	future::{self, ready},
	prelude::*,
};
use parking_lot::{Mutex, RwLock};
use std::{
	collections::{HashMap, HashSet},
	pin::Pin,
	sync::Arc,
};

use crate::graph::{ExtrinsicHash, IsValidator};
use sc_transaction_pool_api::{
	error::Error as TxPoolError, ChainEvent, ImportNotificationStream, MaintainedTransactionPool,
	PoolFuture, PoolStatus, ReadyTransactions, TransactionFor, TransactionPool, TransactionSource,
	TransactionStatusStreamFor, TxHash,
};
use sp_core::traits::SpawnEssentialNamed;
use sp_runtime::{
	generic::BlockId,
	traits::{AtLeast32Bit, Block as BlockT, Extrinsic, Header as HeaderT, NumberFor, Zero},
};
use std::time::Instant;

use sp_blockchain::{HashAndNumber, TreeRoute};

pub(crate) const LOG_TARGET: &str = "txpool";

//todo: View probably needs a hash? parent hash? number?
pub struct View<PoolApi: graph::ChainApi>(graph::Pool<PoolApi>);

impl<PoolApi> View<PoolApi>
where
	PoolApi: graph::ChainApi,
{
	fn new(api: Arc<PoolApi>) -> Self {
		Self(graph::Pool::new(Default::default(), true.into(), api))
	}
}

pub struct ViewManager<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: graph::ChainApi<Block = Block>,
{
	api: Arc<PoolApi>,
	views: RwLock<HashMap<Block::Hash, Arc<View<PoolApi>>>>,
}

pub enum ViewCreationError {
	AlreadyExists,
	Unknown,
	BlockIdConversion,
}

impl<PoolApi, Block> ViewManager<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: graph::ChainApi<Block = Block>,
{
	fn new(api: Arc<PoolApi>) -> Self {
		Self { api, views: Default::default() }
	}

	// shall be called on block import
	pub async fn create_new_view_at(
		&self,
		event: ChainEvent<Block>,
		xts: Arc<RwLock<Vec<Block::Extrinsic>>>,
	) -> Result<(), ViewCreationError> {
		let hash = match event {
			ChainEvent::Finalized { hash, .. } | ChainEvent::NewBestBlock { hash, .. } => hash,
		};
		if self.views.read().contains_key(&hash) {
			return Err(ViewCreationError::AlreadyExists)
		}

		let view = Arc::new(View::new(self.api.clone()));

		//todo: lock or clone?
		//todo: source?
		let source = TransactionSource::External;

		let number = self
			.api
			.resolve_block_number(hash)
			.map_err(|_| ViewCreationError::BlockIdConversion)?;
		let at = HashAndNumber { hash, number };

		//todo: internal checked banned: not required any more?
		let xts = xts.read().clone();
		let _ = view.0.submit_at(&at, source, xts).await;
		self.views.write().insert(hash, view);

		// brute force: just revalidate all xts against block
		// target: find parent, extract all provided tags on enacted path and recompute graph

		Ok(())
	}

	/// Imports a bunch of unverified extrinsics to every view
	pub async fn submit_at(
		&self,
		at: &HashAndNumber<Block>,
		source: TransactionSource,
		xts: impl IntoIterator<Item = Block::Extrinsic> + Clone,
	) -> HashMap<Block::Hash, Vec<Result<ExtrinsicHash<PoolApi>, PoolApi::Error>>> {
		//todo: Result<Vec,Error> is not really needed. submit_at should take HashAndNumber.
		let futs = {
			let g = self.views.read();
			let futs = g
				.iter()
				.map(|(hash, view)| {
					let view = view.clone();
					//todo: remove this clone (Arc?)
					let xts = xts.clone();
					let hash = hash.clone();
					let at = at.clone();
					async move { (hash, view.0.submit_at(&at, source, xts.clone()).await) }
				})
				.collect::<Vec<_>>();
			futs
		};
		let results = futures::future::join_all(futs).await;

		HashMap::<_, _>::from_iter(results.into_iter())
	}

	/// Imports one unverified extrinsic to every view
	pub async fn submit_one(
		&self,
		at: &HashAndNumber<Block>,
		source: TransactionSource,
		xt: Block::Extrinsic,
	) -> HashMap<Block::Hash, Result<ExtrinsicHash<PoolApi>, PoolApi::Error>> {
		//todo: Result<ExtrinsicHash,Error> is not really needed. submit_at should take
		// HashAndNumber.
		let futs = {
			let g = self.views.read();
			let futs = g
				.iter()
				.map(|(hash, view)| {
					let view = view.clone();
					let at = at.clone();
					let xt = xt.clone();
					let hash = hash.clone();
					async move { (hash, view.0.submit_one(&at, source, xt.clone()).await) }
				})
				.collect::<Vec<_>>();
			futs
		};
		let results = futures::future::join_all(futs).await;

		HashMap::<_, _>::from_iter(results.into_iter())
	}

	/// Import a single extrinsic and starts to watch its progress in the pool.
	pub async fn submit_and_watch(
		&self,
		at: Block::Hash,
		source: TransactionSource,
		xt: Block::Extrinsic,
	) -> Result<Watcher<ExtrinsicHash<PoolApi>, ExtrinsicHash<PoolApi>>, PoolApi::Error> {
		unimplemented!()
	}

	pub fn status(&self) -> HashMap<Block::Hash, PoolStatus> {
		self.views
			.read()
			.iter()
			.map(|(h, v)| (*h, v.0.validated_pool().status()))
			.collect()
	}
}

////////////////////////////////////////////////////////////////////////////////

pub struct ForkAwareTxPool<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: graph::ChainApi<Block = Block>,
{
	api: Arc<PoolApi>,
	xts: Arc<RwLock<Vec<Block::Extrinsic>>>,
	views: Arc<ViewManager<PoolApi, Block>>,
	// todo:
	// map: hash -> view
	// ready_poll: Arc<Mutex<ReadyPoll<ReadyIteratorFor<PoolApi>, Block>>>,
	// current tree? (somehow similar to enactment state?)
	// todo: metrics

	// todo: this are coming from ValidatedPool, some of them maybe needed here
	// is_validator: IsValidator,
	// options: Options,
	// listener: RwLock<Listener<ExtrinsicHash<B>, B>>,
	// import_notification_sinks: Mutex<Vec<Sender<ExtrinsicHash<B>>>>,
	// rotator: PoolRotator<ExtrinsicHash<B>>,
}

impl<PoolApi, Block> ForkAwareTxPool<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: graph::ChainApi<Block = Block> + 'static,
{
	/// Create new fork aware transaction pool with provided api, for tests.
	pub fn new_test(
		pool_api: Arc<PoolApi>,
		best_block_hash: Block::Hash,
		finalized_hash: Block::Hash,
	) -> Self {
		Self {
			api: pool_api.clone(),
			xts: Default::default(),
			views: Arc::new(ViewManager::new(pool_api)),
		}
	}

	/// Get access to the underlying api
	pub fn api(&self) -> &PoolApi {
		&self.api
	}

	pub fn status_all(&self) -> HashMap<Block::Hash, PoolStatus> {
		self.views.status()
	}
}

fn xxxx() {
	// in:
	// HashMap<
	//    Hash,
	//    Result<
	//      Vec<
	//        Result<
	//          ExtrinsicHash<PoolApi>,
	//          PoolApi::Error
	//        >
	//      >,
	//      PoolApi::Error
	//    >
	// >
	//
	// out:
	//  Vec<
	//    Result<
	//      ExtrinsicHash<PoolApi>,
	//      PoolApi::Error
	//    >
	//  >
}

impl<PoolApi, Block> TransactionPool for ForkAwareTxPool<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: 'static + graph::ChainApi<Block = Block>,
{
	type Block = PoolApi::Block;
	type Hash = graph::ExtrinsicHash<PoolApi>;
	type InPoolTransaction = graph::base_pool::Transaction<TxHash<Self>, TransactionFor<Self>>;
	type Error = PoolApi::Error;

	fn submit_at(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xts: Vec<TransactionFor<Self>>,
	) -> PoolFuture<Vec<Result<TxHash<Self>, Self::Error>>, Self::Error> {
		let views = self.views.clone();
		self.xts.write().extend(xts.clone());

		// todo:
		// self.metrics
		// 	.report(|metrics| metrics.submitted_transactions.inc_by(xts.len() as u64));

		let number = self.api.resolve_block_number(at);

		async move {
			let at = HashAndNumber { hash: at, number: number? };
			// HashMap< Hash, Result<Vec<Result<ExtrinsicHash<PoolApi>, PoolApi::Error>>,
			// PoolApi::Error>

			let mut results_map = views.submit_at(&at, source, xts).await;
			//todo: unwrap
			Ok(results_map.remove(&at.hash).unwrap())
		}
		.boxed()
	}

	fn submit_one(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> PoolFuture<TxHash<Self>, Self::Error> {
		// todo:
		// self.metrics.report(|metrics| metrics.submitted_transactions.inc());

		let views = self.views.clone();
		self.xts.write().push(xt.clone());
		let number = self.api.resolve_block_number(at);

		async move {
			let at = HashAndNumber { hash: at, number: number? };
			let mut results = views.submit_one(&at, source, xt).await;
			//todo: unwrap
			results.remove(&at.hash).unwrap()
		}
		.boxed()
	}

	fn submit_and_watch(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> PoolFuture<Pin<Box<TransactionStatusStreamFor<Self>>>, Self::Error> {
		let views = self.views.clone();
		self.xts.write().push(xt.clone());

		// todo:
		// self.metrics.report(|metrics| metrics.submitted_transactions.inc());

		async move {
			let watcher = views.submit_and_watch(at, source, xt).await?;

			Ok(watcher.into_stream().boxed())
		}
		.boxed()
	}

	// todo: api change? we need block hash here (assuming we need it at all).
	fn remove_invalid(&self, hashes: &[TxHash<Self>]) -> Vec<Arc<Self::InPoolTransaction>> {
		// let removed = self.pool.validated_pool().remove_invalid(hashes);
		// removed

		//todo:
		// self.metrics
		// 	.report(|metrics| metrics.validations_invalid.inc_by(removed.len() as u64));

		unimplemented!()
	}

	fn status(&self) -> PoolStatus {
		// self.pool.validated_pool().status()
		unimplemented!()
	}

	fn import_notification_stream(&self) -> ImportNotificationStream<TxHash<Self>> {
		// self.pool.validated_pool().import_notification_stream()
		unimplemented!()
	}

	fn hash_of(&self, xt: &TransactionFor<Self>) -> TxHash<Self> {
		self.api().hash_and_length(xt).0
	}

	fn on_broadcasted(&self, propagations: HashMap<TxHash<Self>, Vec<String>>) {
		// self.pool.validated_pool().on_broadcasted(propagations)
		unimplemented!()
	}

	// todo: api change?
	fn ready_transaction(&self, hash: &TxHash<Self>) -> Option<Arc<Self::InPoolTransaction>> {
		// self.pool.validated_pool().ready_by_hash(hash)
		unimplemented!()
	}

	// todo: API change? ready at hash (not number)?
	fn ready_at(
		&self,
		at: NumberFor<Self::Block>,
	) -> Pin<
		Box<
			dyn Future<
					Output = Box<dyn ReadyTransactions<Item = Arc<Self::InPoolTransaction>> + Send>,
				> + Send,
		>,
	> {
		// -> PolledIterator<PoolApi>
		unimplemented!()
	}

	// todo: API change? ready at block?
	fn ready(&self) -> Box<dyn ReadyTransactions<Item = Arc<Self::InPoolTransaction>> + Send> {
		//originally it was: -> ReadyIteratorFor<PoolApi>
		// Box::new(self.pool.validated_pool().ready())
		unimplemented!()
	}

	// todo: API change? futures at block?
	fn futures(&self) -> Vec<Self::InPoolTransaction> {
		// let pool = self.pool.validated_pool().pool.read();
		// pool.futures().cloned().collect::<Vec<_>>()
		unimplemented!()
	}
}

impl<Block, Client> sc_transaction_pool_api::LocalTransactionPool
	for ForkAwareTxPool<FullChainApi<Client, Block>, Block>
where
	Block: BlockT,
	Client: sp_api::ProvideRuntimeApi<Block>
		+ sc_client_api::BlockBackend<Block>
		+ sc_client_api::blockchain::HeaderBackend<Block>
		+ sp_runtime::traits::BlockIdTo<Block>
		+ sp_blockchain::HeaderMetadata<Block, Error = sp_blockchain::Error>,
	Client: Send + Sync + 'static,
	Client::Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>,
{
	type Block = Block;
	type Hash = graph::ExtrinsicHash<FullChainApi<Client, Block>>;
	type Error = <FullChainApi<Client, Block> as graph::ChainApi>::Error;

	fn submit_local(
		&self,
		at: Block::Hash,
		xt: sc_transaction_pool_api::LocalTransactionFor<Self>,
	) -> Result<Self::Hash, Self::Error> {
		unimplemented!();
	}
}

#[async_trait]
impl<PoolApi, Block> MaintainedTransactionPool for ForkAwareTxPool<PoolApi, Block>
where
	Block: BlockT,
	PoolApi: 'static + graph::ChainApi<Block = Block>,
{
	async fn maintain(&self, event: ChainEvent<Self::Block>) {
		//todo: print error?
		let _ = self.views.create_new_view_at(event, self.xts.clone()).await;
	}
}

/// Inform the transaction pool about imported and finalized blocks.
pub async fn notification_future<Client, Pool, Block>(client: Arc<Client>, txpool: Arc<Pool>)
where
	Block: BlockT,
	Client: sc_client_api::BlockchainEvents<Block>,
	Pool: MaintainedTransactionPool<Block = Block>,
{
	unimplemented!();
}
