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

use super::{
	import_notification_sink::MultiViewImportNotificationSink,
	metrics::MetricsLink as PrometheusMetrics,
	multi_view_listener::MultiViewListener,
	tx_mem_pool::{TxMemPool, TXMEMPOOL_TRANSACTION_LIMIT_MULTIPLIER},
	view::View,
	view_store::ViewStore,
};
use crate::{
	api::FullChainApi,
	enactment_state::{EnactmentAction, EnactmentState},
	fork_aware_txpool::view_revalidation,
	graph::{self, base_pool::Transaction, ExtrinsicFor, ExtrinsicHash, IsValidator, Options},
	log_xt_debug, PolledIterator, ReadyIteratorFor, LOG_TARGET,
};
use async_trait::async_trait;
use futures::{
	channel::oneshot,
	future::{self},
	prelude::*,
	FutureExt,
};
use parking_lot::Mutex;
use prometheus_endpoint::Registry as PrometheusRegistry;
use sc_transaction_pool_api::{
	error::{Error, IntoPoolError},
	ChainEvent, ImportNotificationStream, MaintainedTransactionPool, PoolFuture, PoolStatus,
	TransactionFor, TransactionPool, TransactionSource, TransactionStatusStreamFor, TxHash,
};
use sp_blockchain::{HashAndNumber, TreeRoute};
use sp_core::traits::SpawnEssentialNamed;
use sp_runtime::{
	generic::BlockId,
	traits::{Block as BlockT, Extrinsic, NumberFor},
	transaction_validity::{InvalidTransaction, UnknownTransaction},
};
use std::{
	collections::{HashMap, HashSet},
	pin::Pin,
	sync::{
		atomic::{AtomicUsize, Ordering},
		Arc,
	},
	time::Instant,
};
use tokio::select;

pub use super::import_notification_sink::ImportNotificationTask;
pub type FullPool<Block, Client> = ForkAwareTxPool<FullChainApi<Client, Block>, Block>;

struct ReadyPoll<T, Block>
where
	Block: BlockT,
{
	pollers: HashMap<<Block as BlockT>::Hash, Vec<oneshot::Sender<T>>>,
}

impl<T, Block> ReadyPoll<T, Block>
where
	Block: BlockT,
{
	fn new() -> Self {
		Self { pollers: Default::default() }
	}

	fn add(&mut self, at: <Block as BlockT>::Hash) -> oneshot::Receiver<T> {
		let (s, r) = oneshot::channel();
		self.pollers.entry(at).or_default().push(s);
		r
	}

	fn trigger(&mut self, at: <Block as BlockT>::Hash, ready_iterator: impl Fn() -> T) {
		log::debug!(target: LOG_TARGET, "fatp::trigger {at:?} pending keys: {:?}", self.pollers.keys());
		let Some(pollers) = self.pollers.remove(&at) else { return };
		pollers.into_iter().for_each(|p| {
			log::info!(target: LOG_TARGET, "trigger ready signal at block {}", at);
			let _ = p.send(ready_iterator());
		});
	}

	fn remove_cancelled(&mut self) {
		self.pollers.retain(|_, v| v.iter().any(|sender| !sender.is_canceled()));
	}
}

/// The fork-aware transaction pool.
///
/// It keeps track of every fork and provides the set of transactions that is valid for every fork.
pub struct ForkAwareTxPool<ChainApi, Block>
where
	Block: BlockT,
	ChainApi: graph::ChainApi<Block = Block> + 'static,
	<Block as BlockT>::Hash: Unpin,
{
	api: Arc<ChainApi>,
	mempool: Arc<TxMemPool<ChainApi, Block>>,

	view_store: Arc<ViewStore<ChainApi, Block>>,
	ready_poll: Arc<Mutex<ReadyPoll<ReadyIteratorFor<ChainApi>, Block>>>,
	metrics: PrometheusMetrics,
	enactment_state: Arc<Mutex<EnactmentState<Block>>>,
	revalidation_queue: Arc<view_revalidation::RevalidationQueue<ChainApi, Block>>,

	import_notification_sink: MultiViewImportNotificationSink<Block::Hash, ExtrinsicHash<ChainApi>>,
	options: Options,
	is_validator: IsValidator,
	// todo: this are coming from ValidatedPool, some of them maybe needed here
	// rotator: PoolRotator<ExtrinsicHash<B>>,
}

impl<ChainApi, Block> ForkAwareTxPool<ChainApi, Block>
where
	Block: BlockT,
	ChainApi: graph::ChainApi<Block = Block> + 'static,
	<Block as BlockT>::Hash: Unpin,
{
	/// Create new fork aware transaction pool with provided api, for tests.
	pub fn new_test(
		pool_api: Arc<ChainApi>,
		best_block_hash: Block::Hash,
		finalized_hash: Block::Hash,
	) -> (Self, ImportNotificationTask) {
		let listener = Arc::from(MultiViewListener::new());
		let (import_notification_sink, import_notification_sink_task) =
			MultiViewImportNotificationSink::new_with_worker();

		(
			Self {
				mempool: Arc::from(TxMemPool::new(
					pool_api.clone(),
					listener.clone(),
					Default::default(),
					usize::MAX,
				)),
				api: pool_api.clone(),
				view_store: Arc::new(ViewStore::new(pool_api, listener)),
				ready_poll: Arc::from(Mutex::from(ReadyPoll::new())),
				enactment_state: Arc::new(Mutex::new(EnactmentState::new(
					best_block_hash,
					finalized_hash,
				))),
				revalidation_queue: Arc::from(view_revalidation::RevalidationQueue::new()),
				import_notification_sink,
				options: Options::default(),
				is_validator: false.into(),
				metrics: Default::default(),
			},
			import_notification_sink_task,
		)
	}

	/// Create new fork aware transaction pool with provided api.
	///
	/// The txpool essential tasks are spawned using provided spawner.
	pub fn new_with_background_queue(
		options: Options,
		is_validator: IsValidator,
		pool_api: Arc<ChainApi>,
		prometheus: Option<&PrometheusRegistry>,
		spawner: impl SpawnEssentialNamed,
		_best_block_number: NumberFor<Block>,
		best_block_hash: Block::Hash,
		finalized_hash: Block::Hash,
	) -> Self {
		let metrics = PrometheusMetrics::new(prometheus);
		let listener = Arc::from(MultiViewListener::new());
		let (revalidation_queue, revalidation_task) =
			view_revalidation::RevalidationQueue::new_with_worker();

		let (import_notification_sink, import_notification_sink_task) =
			MultiViewImportNotificationSink::new_with_worker();

		let combined_tasks = async move {
			tokio::select! {
				_ = revalidation_task => {},
				_ = import_notification_sink_task => {},
			}
		}
		.boxed();
		spawner.spawn_essential("txpool-background", Some("transaction-pool"), combined_tasks);

		Self {
			mempool: Arc::from(TxMemPool::new(
				pool_api.clone(),
				listener.clone(),
				metrics.clone(),
				TXMEMPOOL_TRANSACTION_LIMIT_MULTIPLIER *
					(options.ready.count + options.future.count),
			)),
			api: pool_api.clone(),
			view_store: Arc::new(ViewStore::new(pool_api, listener)),
			ready_poll: Arc::from(Mutex::from(ReadyPoll::new())),
			enactment_state: Arc::new(Mutex::new(EnactmentState::new(
				best_block_hash,
				finalized_hash,
			))),
			revalidation_queue: Arc::from(revalidation_queue),
			import_notification_sink,
			options,
			metrics,
			is_validator,
		}
	}

	/// Get access to the underlying api
	pub fn api(&self) -> &ChainApi {
		&self.api
	}

	/// Provides a status for all views at the tips of the forks.
	pub fn status_all(&self) -> HashMap<Block::Hash, PoolStatus> {
		self.view_store.status()
	}

	/// Provides a number of views at the tips of the forks.
	pub fn views_count(&self) -> usize {
		self.view_store.views.read().len()
	}

	/// Provides a number of views at the tips of the forks.
	pub fn retracted_views_count(&self) -> usize {
		self.view_store.retracted_views.read().len()
	}

	/// Provides internal views statistics.
	///
	/// Provides block number, count of ready, count of future transactions for every view. It is
	/// suitable for printing log information.
	fn views_stats(&self) -> Vec<(NumberFor<Block>, usize, usize)> {
		self.view_store
			.views
			.read()
			.iter()
			.map(|v| (v.1.at.number, v.1.status().ready, v.1.status().future))
			.collect()
	}

	/// Checks if there is a view at the tip of the fork with given hash.
	pub fn has_view(&self, hash: &Block::Hash) -> bool {
		self.view_store.views.read().contains_key(hash)
	}

	/// Returns numbder of unwatched and watched transactions in internal mempool.
	///
	/// Intended for use in unit tests.
	pub fn mempool_len(&self) -> (usize, usize) {
		self.mempool.unwatched_and_watched_count()
	}

	/// Returns best effort set of ready transactions for given block, without executing full
	/// maintain process.
	///
	/// If maintain was already performed the ready iterator for existing, unmodified view is
	/// returned.
	fn ready_light(&self, at: Block::Hash) -> PolledIterator<ChainApi> {
		let start = Instant::now();
		log::debug!(target: LOG_TARGET, "fatp::ready_light {:?}", at);

		let Ok(block_number) = self.api.resolve_block_number(at) else {
			let empty: ReadyIteratorFor<ChainApi> = Box::new(std::iter::empty());
			return Box::pin(async { empty })
		};

		let (best_view, best_tree_route) = {
			let views = self.view_store.views.read();
			let retracted_views = self.view_store.retracted_views.read();
			let mut best_tree_route = None;
			let mut best_view = None;
			let mut best_enacted_len = usize::MAX;
			for v in views.values().chain(retracted_views.values()) {
				let tree_route = self.api.tree_route(v.at.hash, at);
				if let Ok(tree_route) = tree_route {
					log::debug!(target: LOG_TARGET, "fatp::ready_light {} tree_route from: {} e:{} r:{}", at,v.at.hash,tree_route.enacted().len(), tree_route.retracted().len());
					if tree_route.retracted().is_empty() &&
						tree_route.enacted().len() < best_enacted_len
					{
						best_enacted_len = tree_route.enacted().len();
						best_view = Some(v.clone());
						best_tree_route = Some(tree_route);

						if best_enacted_len == 0 {
							break
						}
					}
				}
			}
			(best_view, best_tree_route)
		};

		let api = self.api.clone();

		Box::pin(async move {
			if let (Some(best_tree_route), Some(best_view)) = (best_tree_route, best_view) {
				let tmp_view = View::new_from_other(
					&best_view,
					&HashAndNumber { hash: at, number: block_number },
				);

				let mut all_extrinsics = vec![];

				for h in best_tree_route.enacted() {
					let extrinsics = api
						.block_body(h.hash)
						.await
						.unwrap_or_else(|e| {
							log::warn!(target: LOG_TARGET, "Compute ready light transactions: error request: {}", e);
							None
						})
						.unwrap_or_default()
						.into_iter()
						.map(|t| api.hash_and_length(&t).0);
					all_extrinsics.extend(extrinsics);
				}

				let before_count = tmp_view.pool.validated_pool().status().ready;
				let in_pool_tags = tmp_view.pool.validated_pool().extrinsics_tags(&all_extrinsics);

				let mut tags = Vec::new();
				for i in in_pool_tags {
					match i {
						// reuse the tags for extrinsics that were found in the pool
						Some(t) => tags.extend(t),
						None => {},
					}
				}
				let _ = tmp_view.pool.validated_pool().prune_tags(tags);

				let after_count = tmp_view.pool.validated_pool().status().ready;
				log::info!(target: LOG_TARGET,
					"fatp::ready_light {} from {} before: {} to be removed: {} after: {} took:{:?}",
					at,
					best_view.at.hash,
					before_count,
					all_extrinsics.len(),
					after_count,
					start.elapsed()
				);
				Box::new(tmp_view.pool.validated_pool().ready())
			} else {
				let empty: ReadyIteratorFor<ChainApi> = Box::new(std::iter::empty());
				log::info!(target: LOG_TARGET, "fatp::ready_light {} -> empty, took:{:?}", at, start.elapsed());
				empty
			}
		})
	}

	fn ready_at_with_timeout_internal(
		&self,
		at: Block::Hash,
		timeout: std::time::Duration,
	) -> PolledIterator<ChainApi> {
		log::info!(target: LOG_TARGET, "fatp::ready_at_with_timeout at {:?} allowed delay: {:?}", at, timeout);

		let timeout = futures_timer::Delay::new(timeout);
		let fall_back_ready = self.ready_light(at).map(|ready| Some(ready));
		let ready_at = self.ready_at(at);

		let maybe_ready = async move {
			select! {
				ready = ready_at => Some(ready),
				_ = timeout => {
					log::warn!(target: LOG_TARGET,
						"Timeout fired waiting for transaction pool at block: ({:?}). \
						Proceeding with production.",
						at,
					);
					None
				}
			}
		};

		Box::pin(async {
			let (maybe_ready, fall_back_ready) =
				futures::future::join(maybe_ready.boxed(), fall_back_ready.boxed()).await;
			maybe_ready
				.unwrap_or_else(|| fall_back_ready.expect("Fallback value is always Some. qed"))
		})
	}
}

/// Converts the input view-to-statuses map into the output vector of statuses.
///
/// The result of importing a bunch of transactions into a single view is the vector of statuses.
/// Every item represents a status for single transaction. The input is the map that associates
/// hash-views with vectors indicating the statuses of transactions imports.
///
/// Import to multiple views result in two-dimensional array of statuses, which is provided as
/// input map.
///
/// This function converts the map into the vec of results, according to the following rules:
/// - for given transaction if at least one status is success, then output vector contains success,
/// - if given transaction status is error for every view, then output vector contains error.
///
/// The results for transactions are in the same order for every view. An output vector preserves
/// this order.
///
/// ```skip
/// in:
/// view  |   xt0 status | xt1 status | xt2 status
/// h1   -> [ Ok(xth0),    Ok(xth1),    Err       ]
/// h2   -> [ Ok(xth0),    Err,         Err       ]
/// h3   -> [ Ok(xth0),    Ok(xth1),    Err       ]
///
/// out:
/// [ Ok(xth0), Ok(xth1), Err ]
/// ```
fn reduce_multiview_result<H, E>(input: &mut HashMap<H, Vec<Result<H, E>>>) -> Vec<Result<H, E>> {
	let mut values = input.values();
	let Some(first) = values.next() else {
		return Default::default();
	};
	let length = first.len();
	assert!(values.all(|x| length == x.len()));

	let mut output = Vec::with_capacity(length);
	for _ in 0..length {
		let ith_results = input
			.values_mut()
			.map(|values_for_view| values_for_view.pop().expect(""))
			.reduce(|mut r, v| {
				if r.is_err() && v.is_ok() {
					r = v;
				}
				r
			});

		output.push(ith_results.expect("views contain at least one entry. qed."));
	}
	output.into_iter().rev().collect()
}

impl<ChainApi, Block> TransactionPool for ForkAwareTxPool<ChainApi, Block>
where
	Block: BlockT,
	ChainApi: 'static + graph::ChainApi<Block = Block>,
	<Block as BlockT>::Hash: Unpin,
{
	type Block = ChainApi::Block;
	type Hash = ExtrinsicHash<ChainApi>;
	type InPoolTransaction = Transaction<ExtrinsicHash<ChainApi>, ExtrinsicFor<ChainApi>>;
	type Error = ChainApi::Error;

	fn submit_at(
		&self,
		_: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xts: Vec<TransactionFor<Self>>,
	) -> PoolFuture<Vec<Result<TxHash<Self>, Self::Error>>, Self::Error> {
		let view_store = self.view_store.clone();
		log::info!(target: LOG_TARGET, "fatp::submit_at count:{} views:{}", xts.len(), self.views_count());
		log_xt_debug!(target: LOG_TARGET, xts.iter().map(|xt| self.tx_hash(xt)), "[{:?}] fatp::submit_at");
		let xts = xts.into_iter().map(Arc::from).collect::<Vec<_>>();
		let mempool_result = self.mempool.extend_unwatched(source, xts.clone());

		if view_store.is_empty() {
			return future::ready(Ok(mempool_result)).boxed()
		}

		let to_be_submitted = mempool_result
			.iter()
			.zip(xts)
			.filter_map(|(result, xt)| result.as_ref().ok().map(|_| xt))
			.collect::<Vec<_>>();

		self.metrics
			.report(|metrics| metrics.submitted_transactions.inc_by(to_be_submitted.len() as _));

		async move {
			let mut results_map = view_store.submit_at(source, to_be_submitted.into_iter()).await;
			let mut submission_result = reduce_multiview_result(&mut results_map).into_iter();

			Ok(mempool_result
				.into_iter()
				.map(|result| {
					result.and_then(|_|
						submission_result
							.next()
							.expect("The number of Ok results in mempool is exactly the same as the size of to-views-submitssion result. qed."))
				})
				.collect::<Vec<_>>())
		}
		.boxed()
	}

	fn submit_one(
		&self,
		_: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> PoolFuture<TxHash<Self>, Self::Error> {
		log::debug!(target: LOG_TARGET, "[{:?}] fatp::submit_one views:{}", self.tx_hash(&xt), self.views_count());
		let xt = Arc::from(xt);
		if let Err(e) = self.mempool.push_unwatched(source, xt.clone()) {
			return future::ready(Err(e)).boxed();
		}
		self.metrics.report(|metrics| metrics.submitted_transactions.inc());

		// assume that transaction may be valid, will be validated later.
		let view_store = self.view_store.clone();
		if view_store.is_empty() {
			return future::ready(Ok(self.hash_of(&xt))).boxed()
		}

		async move {
			let results = view_store.submit_one(source, xt).await;
			let results = results
				.into_values()
				.reduce(|mut r, v| {
					if r.is_err() && v.is_ok() {
						r = v;
					}
					r
				})
				.expect("there is at least one entry in input");
			results
		}
		.boxed()
	}

	fn submit_and_watch(
		&self,
		at: <Self::Block as BlockT>::Hash,
		source: TransactionSource,
		xt: TransactionFor<Self>,
	) -> PoolFuture<Pin<Box<TransactionStatusStreamFor<Self>>>, Self::Error> {
		log::debug!(target: LOG_TARGET, "[{:?}] fatp::submit_and_watch views:{}", self.tx_hash(&xt), self.views_count());
		let xt = Arc::from(xt);
		if let Err(e) = self.mempool.push_watched(source, xt.clone()) {
			return future::ready(Err(e)).boxed();
		}
		self.metrics.report(|metrics| metrics.submitted_transactions.inc());

		let view_store = self.view_store.clone();
		async move { view_store.submit_and_watch(at, source, xt).await }.boxed()
	}

	// todo: api change? we need block hash here (assuming we need it at all - could be useful for
	// verification for debugging purposes).
	fn remove_invalid(&self, hashes: &[TxHash<Self>]) -> Vec<Arc<Self::InPoolTransaction>> {
		if !hashes.is_empty() {
			log::info!(target: LOG_TARGET, "fatp::remove_invalid {}", hashes.len());
			log_xt_debug!(target:LOG_TARGET, hashes, "[{:?}] fatp::remove_invalid");
			let _ = hashes
				.len()
				.try_into()
				.map(|v| self.metrics.report(|metrics| metrics.removed_invalid_txs.inc_by(v)));
		}
		Default::default()
	}

	// todo: api change?
	// status(Hash) -> Option<PoolStatus>
	fn status(&self) -> PoolStatus {
		self.view_store
			.most_recent_view
			.read()
			.map(|hash| self.view_store.status()[&hash].clone())
			.unwrap_or(PoolStatus { ready: 0, ready_bytes: 0, future: 0, future_bytes: 0 })
	}

	/// Return an event stream of notifications for when transactions are imported to the pool.
	///
	/// Consumers of this stream should use the `ready` method to actually get the
	/// pending transactions in the right order.
	fn import_notification_stream(&self) -> ImportNotificationStream<ExtrinsicHash<ChainApi>> {
		self.import_notification_sink.event_stream()
	}

	fn hash_of(&self, xt: &TransactionFor<Self>) -> TxHash<Self> {
		self.api().hash_and_length(xt).0
	}

	fn on_broadcasted(&self, propagations: HashMap<TxHash<Self>, Vec<String>>) {
		self.view_store.listener.transactions_broadcasted(propagations);
	}

	// todo: api change: we probably should have at here?
	fn ready_transaction(&self, tx_hash: &TxHash<Self>) -> Option<Arc<Self::InPoolTransaction>> {
		let most_recent_view = self.view_store.most_recent_view.read();
		let result = most_recent_view
			.map(|block_hash| self.view_store.ready_transaction(block_hash, tx_hash))
			.flatten();
		log::trace!(
			target: LOG_TARGET,
			"[{tx_hash:?}] ready_transaction: {} {:?}",
			result.is_some(),
			most_recent_view
		);
		result
	}

	fn ready_at(&self, at: <Self::Block as BlockT>::Hash) -> PolledIterator<ChainApi> {
		if let Some((view, retracted)) = self.view_store.get_view_at(at, true) {
			log::info!(target: LOG_TARGET, "fatp::ready_at {:?} (retracted:{:?})", at, retracted);
			let iterator: ReadyIteratorFor<ChainApi> = Box::new(view.pool.validated_pool().ready());
			return async move { iterator }.boxed();
		}

		let pending = self
			.ready_poll
			.lock()
			.add(at)
			.map(|received| {
				received.unwrap_or_else(|e| {
					log::warn!(target: LOG_TARGET, "Error receiving ready-set iterator: {:?}", e);
					Box::new(std::iter::empty())
				})
			})
			.boxed();
		log::info!(target: LOG_TARGET,
			"fatp::ready_at {at:?} pending keys: {:?}",
			self.ready_poll.lock().pollers.keys()
		);
		pending
	}

	fn ready(&self) -> ReadyIteratorFor<ChainApi> {
		self.view_store.ready()
	}

	fn futures(&self) -> Vec<Self::InPoolTransaction> {
		self.view_store.futures()
	}

	fn ready_at_with_timeout(
		&self,
		at: <Self::Block as BlockT>::Hash,
		timeout: std::time::Duration,
	) -> PolledIterator<ChainApi> {
		self.ready_at_with_timeout_internal(at, timeout)
	}
}

impl<Block, Client> sc_transaction_pool_api::LocalTransactionPool
	for ForkAwareTxPool<FullChainApi<Client, Block>, Block>
where
	Block: BlockT,
	<Block as BlockT>::Hash: Unpin,
	Client: sp_api::ProvideRuntimeApi<Block>
		+ sc_client_api::BlockBackend<Block>
		+ sc_client_api::blockchain::HeaderBackend<Block>
		+ sp_runtime::traits::BlockIdTo<Block>
		+ sp_blockchain::HeaderMetadata<Block, Error = sp_blockchain::Error>,
	Client: Send + Sync + 'static,
	Client::Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>,
{
	type Block = Block;
	type Hash = ExtrinsicHash<FullChainApi<Client, Block>>;
	type Error = <FullChainApi<Client, Block> as graph::ChainApi>::Error;

	fn submit_local(
		&self,
		_at: Block::Hash,
		_xt: sc_transaction_pool_api::LocalTransactionFor<Self>,
	) -> Result<Self::Hash, Self::Error> {
		//todo
		//looks like view_store / view needs non async submit_local method ?.
		unimplemented!();
	}
}

impl<ChainApi, Block> ForkAwareTxPool<ChainApi, Block>
where
	Block: BlockT,
	ChainApi: graph::ChainApi<Block = Block> + 'static,
	<Block as BlockT>::Hash: Unpin,
{
	async fn handle_new_block(&self, tree_route: &TreeRoute<Block>) {
		let hash_and_number = match tree_route.last() {
			Some(hash_and_number) => hash_and_number,
			None => {
				log::warn!(
					target: LOG_TARGET,
					"Skipping ChainEvent - no last block in tree route {:?}",
					tree_route,
				);
				return
			},
		};

		if self.has_view(&hash_and_number.hash) {
			log::debug!(
				target: LOG_TARGET,
				"view already exists for block: {:?}",
				hash_and_number,
			);
			return
		}

		let best_view = self.view_store.find_best_view(tree_route);
		let new_view = self.build_new_view(best_view, hash_and_number, tree_route).await;

		if let Some(view) = new_view {
			{
				let view = view.clone();
				self.ready_poll.lock().trigger(hash_and_number.hash, move || {
					Box::from(view.pool.validated_pool().ready())
				});
			}

			View::start_background_revalidation(view, self.revalidation_queue.clone()).await;
		}

		// {
		// 	let to_be_removed = {
		// 		let mut to_be_removed = vec![];
		// 		let views = self.view_store.views.read();
		// 		if !views.is_empty() {
		// 			let all = self.mempool.clone_all();
		// 			for (tx_hash, _) in all {
		// 				let mut statuses = views
		// 					.values()
		// 					.map(|v| v.pool.validated_pool().check_is_known(&tx_hash, false));
		//
		// 				if statuses.all(|status| {
		// 					status.is_ok()
		// 					// if let Err(error) = status {
		// 					// 	matches!(error.into_pool_error(), Ok(Error::TemporarilyBanned))
		// 					// } else {
		// 					// 	false
		// 					// }
		// 				}) {
		// 					to_be_removed.push(tx_hash);
		// 				}
		// 			}
		// 		};
		// 		to_be_removed
		// 	};
		// 	self.mempool.remove_transactions(&to_be_removed).await;
		// }
	}

	async fn build_new_view(
		&self,
		origin_view: Option<Arc<View<ChainApi>>>,
		at: &HashAndNumber<Block>,
		tree_route: &TreeRoute<Block>,
	) -> Option<Arc<View<ChainApi>>> {
		log::info!(
			target: LOG_TARGET,
			"build_new_view: for: {:?} from: {:?} tree_route: {:?}",
			at,
			origin_view.as_ref().map(|v| v.at.clone()),
			tree_route
		);
		let mut view = if let Some(origin_view) = origin_view {
			let mut view = View::new_from_other(&origin_view, at);
			if !tree_route.retracted().is_empty() {
				view.pool.clear_recently_pruned();
			}
			view
		} else {
			log::info!(target: LOG_TARGET, "creating non-cloned view: for: {at:?}");
			View::new(
				self.api.clone(),
				at.clone(),
				self.options.clone(),
				self.metrics.clone(),
				self.is_validator.clone(),
			)
		};

		//we need to capture all import notifiication from the very beginning
		self.import_notification_sink.add_view(
			view.at.hash,
			view.pool.validated_pool().import_notification_stream().boxed(),
		);

		let start = Instant::now();
		self.update_view(&mut view).await;
		let duration = start.elapsed();
		log::info!(target: LOG_TARGET, "update_view_pool: at {at:?} took {duration:?}");

		let start = Instant::now();
		self.update_view_with_fork(&view, tree_route, at.clone()).await;
		let duration = start.elapsed();
		log::info!(target: LOG_TARGET, "update_view_fork: at {at:?} took {duration:?}");

		let view = Arc::from(view);
		self.view_store.insert_new_view(view.clone(), tree_route).await;
		Some(view)
	}

	/// Returns the list of xts included in all block ancestors, excluding the block itself.
	///
	/// For the following chain `F<-B1<-B2<-B3` xts from `F,B1,B2` will returned.
	async fn extrinsics_included_since_finalized(&self, at: Block::Hash) -> HashSet<TxHash<Self>> {
		let start = Instant::now();
		let recent_finalized_block = self.enactment_state.lock().recent_finalized_block();

		let Ok(tree_route) = self.api.tree_route(recent_finalized_block, at) else {
			return Default::default()
		};

		let api = self.api.clone();
		let mut all_extrinsics = HashSet::new();

		for h in tree_route.enacted().iter().rev().skip(1) {
			api.block_body(h.hash)
				.await
				.unwrap_or_else(|e| {
					log::warn!(target: LOG_TARGET, "Compute ready light transactions: error request: {}", e);
					None
				})
				.unwrap_or_default()
				.into_iter()
				.map(|t| self.hash_of(&t))
				.for_each(|tx_hash| {
					all_extrinsics.insert(tx_hash);
				});
		}

		log::info!(target: LOG_TARGET,
			"fatp::extrinsics_included_since_finalized {} from {} count: {} took:{:?}",
			at,
			recent_finalized_block,
			all_extrinsics.len(),
			start.elapsed()
		);
		all_extrinsics
	}

	async fn update_view(&self, view: &View<ChainApi>) {
		log::debug!(
			target: LOG_TARGET,
			"update_view: {:?} xts:{:?} v:{}",
			view.at,
			self.mempool.unwatched_and_watched_count(),
			self.views_count()
		);
		//todo: this could be collected/cached in view
		let included_xts = self.extrinsics_included_since_finalized(view.at.hash).await;

		//todo: can we do better - w/o clone?
		let xts = self.mempool.clone_unwatched();

		let mut all_submitted_count = 0;
		if !xts.is_empty() {
			let unwatched_count = xts.len();
			let mut buckets = HashMap::<TransactionSource, Vec<ExtrinsicFor<ChainApi>>>::default();
			xts.into_iter()
				.filter(|(hash, _)| !view.pool.validated_pool().pool.read().is_imported(hash))
				.filter(|(hash, _)| !included_xts.contains(&hash))
				.map(|(_, tx)| (tx.source, tx.tx()))
				.for_each(|(source, tx)| buckets.entry(source).or_default().push(tx));

			for (source, xts) in buckets {
				all_submitted_count += xts.len();
				let _ = view.submit_many(source, xts).await;
			}
			log::info!(target: LOG_TARGET, "update_view_pool: at {:?} unwatched {}/{}", view.at.hash, all_submitted_count, unwatched_count);
		}
		let view = Arc::from(view);

		let included_xts = Arc::from(included_xts);
		//todo: maybe we don't need to register listener in view? We could use
		// multi_view_listner.transcation_in_block
		let submitted_count = Arc::from(AtomicUsize::new(0));

		let results = self
			.mempool
			.clone_watched()
			.into_iter()
			.map(|(tx_hash,tx)| {
				let view = view.clone();
				let included_xts = included_xts.clone();
				let submitted_count = submitted_count.clone();
				async move {
					let result = if view.pool.validated_pool().pool.read().is_imported(&tx_hash) || included_xts.contains(&tx_hash)
					{
						Ok(view.create_watcher(tx_hash))
					} else {
						submitted_count.fetch_add(1, Ordering::Relaxed);
						view.submit_and_watch(tx.source, tx.tx()).await
					};
					let result = result.map_or_else(
						|error| {
							let error = error.into_pool_error();
							log::trace!(
								target: LOG_TARGET,
								"[{:?}] update_view: submit_and_watch result: {:?} {:?}",
								tx_hash,
								view.at.hash,
								error,
							);
							match error {
								// We need to install listener for stale xt: in case of
								// transaction being already included in the block we want to
								// send inblock + finalization event.
								// The same applies for TemporarilyBanned / AlreadyImported. We
								// need to create listener.
								Ok(
									Error::InvalidTransaction(InvalidTransaction::Stale) |
									Error::TemporarilyBanned |
									Error::AlreadyImported(_),
								) => Ok(view.create_watcher(tx_hash)),
								Ok(
									Error::InvalidTransaction(_),
								) => Err((error.expect("already in Ok arm. qed."), tx_hash, tx.tx())),
								_ => {
									log::error!(target: LOG_TARGET, "[{:?}] txpool: update_view: something went wrong: {error:?}", tx_hash);
									Err((
										Error::UnknownTransaction(UnknownTransaction::CannotLookup),
										tx_hash,
										tx.tx(),
									))
								},
							}
						},
						|watcher| Ok(watcher),
					);

					if let Ok(watcher) = result {
						log::trace!(target: LOG_TARGET, "[{:?}] adding watcher {:?}", tx_hash, view.at.hash);
						self.view_store
							.listener
							.add_view_watcher_for_tx(
								tx_hash,
								view.at.hash,
								watcher.into_stream().boxed(),
							);
						Ok(())
					} else {
						result.map(|_| ())
					}
				}
			})
			.collect::<Vec<_>>();

		let results = future::join_all(results).await;
		let submitted_count = submitted_count.load(Ordering::Relaxed);

		log::info!(target: LOG_TARGET, "update_view_pool: at {:?} watched {}/{}", view.at.hash, submitted_count, self.mempool_len().1);

		all_submitted_count += submitted_count;
		let _ = all_submitted_count
			.try_into()
			.map(|v| self.metrics.report(|metrics| metrics.submitted_from_mempool_txs.inc_by(v)));

		// if there are no views yet, and a single newly created view is reporting error, just send
		// out the invalid event, and remove transaction.
		if self.view_store.is_empty() {
			for result in results {
				match result {
					Err((Error::InvalidTransaction(_), tx_hash, tx)) => {
						self.view_store.listener.invalidate_transactions(vec![tx_hash]);
						self.mempool.remove_watched(&tx);
					},
					_ => {},
				}
			}
		}
	}

	async fn update_view_with_fork(
		&self,
		view: &View<ChainApi>,
		tree_route: &TreeRoute<Block>,
		hash_and_number: HashAndNumber<Block>,
	) {
		log::debug!(target: LOG_TARGET, "update_view_with_fork tree_route: {:?} {tree_route:?}", view.at);
		let api = self.api.clone();

		// We keep track of everything we prune so that later we won't add
		// transactions with those hashes from the retracted blocks.
		let mut pruned_log = HashSet::<ExtrinsicHash<ChainApi>>::new();

		future::join_all(
			tree_route
				.enacted()
				.iter()
				// .chain(std::iter::once(&hash_and_number))
				.map(|h| crate::prune_known_txs_for_block(h, &*api, &view.pool)),
		)
		.await
		.into_iter()
		.for_each(|enacted_log| {
			pruned_log.extend(enacted_log);
		});

		//resubmit
		{
			let mut resubmit_transactions = Vec::new();

			for retracted in tree_route.retracted() {
				let hash = retracted.hash;

				let block_transactions = api
					.block_body(hash)
					.await
					.unwrap_or_else(|e| {
						log::warn!(target: LOG_TARGET, "Failed to fetch block body: {}", e);
						None
					})
					.unwrap_or_default()
					.into_iter()
					.filter(|tx| tx.is_signed().unwrap_or(true));

				let mut resubmitted_to_report = 0;

				resubmit_transactions.extend(
					block_transactions
						.into_iter()
						.map(|tx| (self.hash_of(&tx), tx))
						.filter(|(tx_hash, _)| {
							let contains = pruned_log.contains(&tx_hash);

							// need to count all transactions, not just filtered, here
							resubmitted_to_report += 1;

							if !contains {
								log::trace!(
									target: LOG_TARGET,
									"[{:?}]: Resubmitting from retracted block {:?}",
									tx_hash,
									hash,
								);
							}
							!contains
						})
						.map(|(tx_hash, tx)| {
							//find arc if tx is known
							self.mempool.get_by_hash(tx_hash).unwrap_or_else(|| Arc::from(tx))
						}),
				);

				self.metrics.report(|metrics| {
					metrics.resubmitted_retracted_txs.inc_by(resubmitted_to_report)
				});
			}

			let _ = view
				.pool
				.resubmit_at(
					&hash_and_number,
					// These transactions are coming from retracted blocks, we should
					// simply consider them external.
					TransactionSource::External,
					resubmit_transactions,
				)
				.await;
		}
	}

	async fn handle_finalized(&self, finalized_hash: Block::Hash, tree_route: &[Block::Hash]) {
		let finalized_number = self.api.block_id_to_number(&BlockId::Hash(finalized_hash));
		log::info!(target: LOG_TARGET, "handle_finalized {finalized_number:?} tree_route: {tree_route:?} views_count:{}", self.views_count());

		let finalized_xts = self.view_store.handle_finalized(finalized_hash, tree_route).await;

		self.mempool.purge_finalized_transactions(&finalized_xts).await;
		self.import_notification_sink.clean_filter(&finalized_xts);

		self.metrics
			.report(|metrics| metrics.finalized_txs.inc_by(finalized_xts.len() as _));

		if let Ok(Some(finalized_number)) = finalized_number {
			self.revalidation_queue
				.purge_transactions_later(
					self.mempool.clone(),
					HashAndNumber { hash: finalized_hash, number: finalized_number },
				)
				.await;
		} else {
			log::debug!(target: LOG_TARGET, "purge_transactions_later skipped, cannot find block number {finalized_number:?}");
		}

		self.ready_poll.lock().remove_cancelled();
		log::debug!(target: LOG_TARGET, "handle_finalized after views_count:{:?}", self.views_count());
	}

	fn tx_hash(&self, xt: &TransactionFor<Self>) -> TxHash<Self> {
		self.api.hash_and_length(xt).0
	}

	// use for verirfaction - only for debugging purposes
	// todo: to be removed at some point
	#[allow(dead_code)]
	async fn verify(&self) {
		log::info!(target:LOG_TARGET, "fatp::verify++");

		let views_ready_txs = {
			let views = self.view_store.views.read();

			views
				.values()
				.map(|view| {
					let ready = view.pool.validated_pool().ready();
					let future = view.pool.validated_pool().futures();
					(view.at.hash, ready.collect::<Vec<_>>(), future)
				})
				.collect::<Vec<_>>()
		};

		for view in views_ready_txs {
			let block_hash = view.0;
			let ready = view.1;
			for tx in ready {
				let validation_result = self
					.api
					.validate_transaction(block_hash, TransactionSource::External, tx.data.clone())
					.await;
				log::debug!(target:LOG_TARGET, "[{:?}] is ready in view {:?} validation result {:?}", tx.hash, block_hash, validation_result);
			}
			let future = view.2;
			for tx in future {
				let validation_result = self
					.api
					.validate_transaction(block_hash, TransactionSource::External, tx.1.clone())
					.await;
				log::debug!(target:LOG_TARGET, "[{:?}] is future in view {:?} validation result {:?}", tx.0, block_hash, validation_result);
			}
		}
		log::info!(target:LOG_TARGET, "fatp::verify--");
	}
}

#[async_trait]
impl<ChainApi, Block> MaintainedTransactionPool for ForkAwareTxPool<ChainApi, Block>
where
	Block: BlockT,
	ChainApi: 'static + graph::ChainApi<Block = Block>,
	<Block as BlockT>::Hash: Unpin,
{
	async fn maintain(&self, event: ChainEvent<Self::Block>) {
		let start = Instant::now();
		log::info!(target: LOG_TARGET, "processing event: {event:?}");

		self.view_store.finish_background_revalidations().await;

		let prev_finalized_block = self.enactment_state.lock().recent_finalized_block();

		let compute_tree_route = |from, to| -> Result<TreeRoute<Block>, String> {
			match self.api.tree_route(from, to) {
				Ok(tree_route) => Ok(tree_route),
				Err(e) =>
					return Err(format!(
						"Error occurred while computing tree_route from {from:?} to {to:?}: {e}"
					)),
			}
		};
		let block_id_to_number =
			|hash| self.api.block_id_to_number(&BlockId::Hash(hash)).map_err(|e| format!("{}", e));

		let result =
			self.enactment_state
				.lock()
				.update(&event, &compute_tree_route, &block_id_to_number);

		match result {
			Err(msg) => {
				log::debug!(target: LOG_TARGET, "enactment_state::update error: {msg}");
				self.enactment_state.lock().force_update(&event);
			},
			Ok(EnactmentAction::Skip) => return,
			Ok(EnactmentAction::HandleFinalization) => {
				// todo: in some cases handle_new_block is actually needed (new_num > tips_of_forks)
				// let hash = event.hash();
				// if !self.has_view(hash) {
				// 	if let Ok(tree_route) = compute_tree_route(prev_finalized_block, hash) {
				// 		self.handle_new_block(&tree_route).await;
				// 	}
				// }
			},
			Ok(EnactmentAction::HandleEnactment(tree_route)) =>
				self.handle_new_block(&tree_route).await,
		};

		match event {
			ChainEvent::NewBestBlock { .. } => {},
			ChainEvent::Finalized { hash, ref tree_route } => {
				self.handle_finalized(hash, tree_route).await;

				log::trace!(
					target: LOG_TARGET,
					"on-finalized enacted: {tree_route:?}, previously finalized: \
					{prev_finalized_block:?}",
				);
			},
		}

		let maintain_duration = start.elapsed();

		log::info!(
			target: LOG_TARGET,
			"maintain: txs:{:?} views:[{};{:?}] event:{event:?}  took:{:?}",
			self.mempool_len(),
			self.views_count(),
			self.views_stats(),
			maintain_duration
		);

		self.metrics.report(|metrics| {
			let (unwatched, watched) = self.mempool_len();
			let _ = (
				self.views_count().try_into().map(|v| metrics.active_views.set(v)),
				self.retracted_views_count().try_into().map(|v| metrics.inactive_views.set(v)),
				watched.try_into().map(|v| metrics.watched_txs.set(v)),
				unwatched.try_into().map(|v| metrics.unwatched_txs.set(v)),
			);
			metrics.maintain_duration.observe(maintain_duration.as_secs_f64());
		});

		// self.verify().await;

		()
	}
}

impl<Block, Client> FullPool<Block, Client>
where
	Block: BlockT,
	Client: sp_api::ProvideRuntimeApi<Block>
		+ sc_client_api::BlockBackend<Block>
		+ sc_client_api::blockchain::HeaderBackend<Block>
		+ sp_runtime::traits::BlockIdTo<Block>
		+ sc_client_api::ExecutorProvider<Block>
		+ sc_client_api::UsageProvider<Block>
		+ sp_blockchain::HeaderMetadata<Block, Error = sp_blockchain::Error>
		+ Send
		+ Sync
		+ 'static,
	Client::Api: sp_transaction_pool::runtime_api::TaggedTransactionQueue<Block>,
	<Block as BlockT>::Hash: std::marker::Unpin,
{
	/// Create new basic transaction pool for a full node with the provided api.
	pub fn new_full(
		options: Options,
		is_validator: IsValidator,
		prometheus: Option<&PrometheusRegistry>,
		spawner: impl SpawnEssentialNamed,
		client: Arc<Client>,
	) -> Arc<Self> {
		let pool_api = Arc::new(FullChainApi::new(client.clone(), prometheus, &spawner));
		let pool = Arc::new(Self::new_with_background_queue(
			options,
			is_validator,
			pool_api,
			prometheus,
			spawner,
			client.usage_info().chain.best_number,
			client.usage_info().chain.best_hash,
			client.usage_info().chain.finalized_hash,
		));

		pool
	}
}

#[cfg(test)]
mod reduce_multiview_result_tests {
	use super::*;
	use sp_core::H256;
	#[derive(Debug, PartialEq, Clone)]
	enum Error {
		Custom(u8),
	}

	#[test]
	fn empty() {
		sp_tracing::try_init_simple();
		let mut input = HashMap::default();
		let r = reduce_multiview_result::<H256, Error>(&mut input);
		assert!(r.is_empty());
	}

	#[test]
	fn errors_only() {
		sp_tracing::try_init_simple();
		let v: Vec<(H256, Vec<Result<H256, Error>>)> = vec![
			(
				H256::repeat_byte(0x13),
				vec![
					Err(Error::Custom(10)),
					Err(Error::Custom(11)),
					Err(Error::Custom(12)),
					Err(Error::Custom(13)),
				],
			),
			(
				H256::repeat_byte(0x14),
				vec![
					Err(Error::Custom(20)),
					Err(Error::Custom(21)),
					Err(Error::Custom(22)),
					Err(Error::Custom(23)),
				],
			),
			(
				H256::repeat_byte(0x15),
				vec![
					Err(Error::Custom(30)),
					Err(Error::Custom(31)),
					Err(Error::Custom(32)),
					Err(Error::Custom(33)),
				],
			),
		];
		let mut input = HashMap::from_iter(v.clone());
		let r = reduce_multiview_result(&mut input);

		//order in HashMap is random, the result shall be one of:
		assert!(r == v[0].1 || r == v[1].1 || r == v[2].1);
	}

	#[test]
	#[should_panic]
	fn invalid_lengths() {
		sp_tracing::try_init_simple();
		let v: Vec<(H256, Vec<Result<H256, Error>>)> = vec![
			(H256::repeat_byte(0x13), vec![Err(Error::Custom(12)), Err(Error::Custom(13))]),
			(H256::repeat_byte(0x14), vec![Err(Error::Custom(23))]),
		];
		let mut input = HashMap::from_iter(v);
		let _ = reduce_multiview_result(&mut input);
	}

	#[test]
	fn only_hashes() {
		sp_tracing::try_init_simple();

		let v: Vec<(H256, Vec<Result<H256, Error>>)> = vec![
			(
				H256::repeat_byte(0x13),
				vec![Ok(H256::repeat_byte(0x13)), Ok(H256::repeat_byte(0x14))],
			),
			(
				H256::repeat_byte(0x14),
				vec![Ok(H256::repeat_byte(0x13)), Ok(H256::repeat_byte(0x14))],
			),
		];
		let mut input = HashMap::from_iter(v);
		let r = reduce_multiview_result(&mut input);

		assert_eq!(r, vec![Ok(H256::repeat_byte(0x13)), Ok(H256::repeat_byte(0x14))]);
	}

	#[test]
	fn one_view() {
		sp_tracing::try_init_simple();
		let v: Vec<(H256, Vec<Result<H256, Error>>)> = vec![(
			H256::repeat_byte(0x13),
			vec![Ok(H256::repeat_byte(0x10)), Err(Error::Custom(11))],
		)];
		let mut input = HashMap::from_iter(v);
		let r = reduce_multiview_result(&mut input);

		assert_eq!(r, vec![Ok(H256::repeat_byte(0x10)), Err(Error::Custom(11))]);
	}

	#[test]
	fn mix() {
		sp_tracing::try_init_simple();
		let v: Vec<(H256, Vec<Result<H256, Error>>)> = vec![
			(
				H256::repeat_byte(0x13),
				vec![
					Ok(H256::repeat_byte(0x10)),
					Err(Error::Custom(11)),
					Err(Error::Custom(12)),
					Err(Error::Custom(33)),
				],
			),
			(
				H256::repeat_byte(0x14),
				vec![
					Err(Error::Custom(20)),
					Ok(H256::repeat_byte(0x21)),
					Err(Error::Custom(22)),
					Err(Error::Custom(33)),
				],
			),
			(
				H256::repeat_byte(0x15),
				vec![
					Err(Error::Custom(30)),
					Err(Error::Custom(31)),
					Ok(H256::repeat_byte(0x32)),
					Err(Error::Custom(33)),
				],
			),
		];
		let mut input = HashMap::from_iter(v);
		let r = reduce_multiview_result(&mut input);

		assert_eq!(
			r,
			vec![
				Ok(H256::repeat_byte(0x10)),
				Ok(H256::repeat_byte(0x21)),
				Ok(H256::repeat_byte(0x32)),
				Err(Error::Custom(33))
			]
		);
	}
}
