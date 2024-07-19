// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! The provisioner is responsible for assembling a relay chain block
//! from a set of available parachain candidates of its choice.

#![deny(missing_docs, unused_crate_dependencies)]

use bitvec::vec::BitVec;
use futures::{
	channel::oneshot, future::BoxFuture, prelude::*, stream::FuturesUnordered, FutureExt,
};
use futures_timer::Delay;
use schnellru::{ByLength, LruMap};

use polkadot_node_subsystem::{
	jaeger,
	messages::{
		Ancestors, CandidateBackingMessage, ChainApiMessage, ProspectiveParachainsMessage,
		ProvisionableData, ProvisionerInherentData, ProvisionerMessage, RuntimeApiRequest,
	},
	overseer, ActivatedLeaf, ActiveLeavesUpdate, FromOrchestra, OverseerSignal, PerLeafSpan,
	SpawnedSubsystem, SubsystemError,
};
use polkadot_node_subsystem_util::{
	has_required_runtime, request_availability_cores, request_persisted_validation_data,
	request_session_index_for_child,
	runtime::{prospective_parachains_mode, request_node_features, ProspectiveParachainsMode},
	TimeoutExt,
};
use polkadot_primitives::{
	node_features::FeatureIndex, BackedCandidate, BlockNumber, CandidateHash, CandidateReceipt,
	CoreIndex, CoreState, Hash, Id as ParaId, NodeFeatures, OccupiedCoreAssumption, SessionIndex,
	SignedAvailabilityBitfield, ValidatorIndex,
};
use std::collections::{BTreeMap, HashMap};

mod disputes;
mod error;
mod metrics;

pub use self::metrics::*;
use error::{Error, FatalResult};

#[cfg(test)]
mod tests;

/// How long to wait before proposing.
const PRE_PROPOSE_TIMEOUT: std::time::Duration = core::time::Duration::from_millis(2000);
/// Some timeout to ensure task won't hang around in the background forever on issues.
const SEND_INHERENT_DATA_TIMEOUT: std::time::Duration = core::time::Duration::from_millis(500);

const LOG_TARGET: &str = "parachain::provisioner";

const PRIORITIZED_SELECTION_RUNTIME_VERSION_REQUIREMENT: u32 =
	RuntimeApiRequest::DISPUTES_RUNTIME_REQUIREMENT;

/// The provisioner subsystem.
pub struct ProvisionerSubsystem {
	metrics: Metrics,
}

impl ProvisionerSubsystem {
	/// Create a new instance of the `ProvisionerSubsystem`.
	pub fn new(metrics: Metrics) -> Self {
		Self { metrics }
	}
}

/// Per-session info we need for the provisioner subsystem.
pub struct PerSession {
	prospective_parachains_mode: ProspectiveParachainsMode,
	elastic_scaling_mvp: bool,
}

/// A per-relay-parent state for the provisioning subsystem.
pub struct PerRelayParent {
	leaf: ActivatedLeaf,
	backed_candidates: Vec<CandidateReceipt>,
	prospective_parachains_mode: ProspectiveParachainsMode,
	elastic_scaling_mvp: bool,
	signed_bitfields: Vec<SignedAvailabilityBitfield>,
	is_inherent_ready: bool,
	awaiting_inherent: Vec<oneshot::Sender<ProvisionerInherentData>>,
	span: PerLeafSpan,
}

impl PerRelayParent {
	fn new(leaf: ActivatedLeaf, per_session: &PerSession) -> Self {
		let span = PerLeafSpan::new(leaf.span.clone(), "provisioner");

		Self {
			leaf,
			backed_candidates: Vec::new(),
			prospective_parachains_mode: per_session.prospective_parachains_mode,
			elastic_scaling_mvp: per_session.elastic_scaling_mvp,
			signed_bitfields: Vec::new(),
			is_inherent_ready: false,
			awaiting_inherent: Vec::new(),
			span,
		}
	}
}

type InherentDelays = FuturesUnordered<BoxFuture<'static, Hash>>;

#[overseer::subsystem(Provisioner, error=SubsystemError, prefix=self::overseer)]
impl<Context> ProvisionerSubsystem {
	fn start(self, ctx: Context) -> SpawnedSubsystem {
		let future = async move {
			run(ctx, self.metrics)
				.await
				.map_err(|e| SubsystemError::with_origin("provisioner", e))
		}
		.boxed();

		SpawnedSubsystem { name: "provisioner-subsystem", future }
	}
}

#[overseer::contextbounds(Provisioner, prefix = self::overseer)]
async fn run<Context>(mut ctx: Context, metrics: Metrics) -> FatalResult<()> {
	let mut inherent_delays = InherentDelays::new();
	let mut per_relay_parent = HashMap::new();
	let mut per_session = LruMap::new(ByLength::new(2));

	loop {
		let result = run_iteration(
			&mut ctx,
			&mut per_relay_parent,
			&mut per_session,
			&mut inherent_delays,
			&metrics,
		)
		.await;

		match result {
			Ok(()) => break,
			err => crate::error::log_error(err)?,
		}
	}

	Ok(())
}

#[overseer::contextbounds(Provisioner, prefix = self::overseer)]
async fn run_iteration<Context>(
	ctx: &mut Context,
	per_relay_parent: &mut HashMap<Hash, PerRelayParent>,
	per_session: &mut LruMap<SessionIndex, PerSession>,
	inherent_delays: &mut InherentDelays,
	metrics: &Metrics,
) -> Result<(), Error> {
	loop {
		futures::select! {
			from_overseer = ctx.recv().fuse() => {
				// Map the error to ensure that the subsystem exits when the overseer is gone.
				match from_overseer.map_err(Error::OverseerExited)? {
					FromOrchestra::Signal(OverseerSignal::ActiveLeaves(update)) =>
						handle_active_leaves_update(ctx.sender(), update, per_relay_parent, per_session, inherent_delays).await?,
					FromOrchestra::Signal(OverseerSignal::BlockFinalized(..)) => {},
					FromOrchestra::Signal(OverseerSignal::Conclude) => return Ok(()),
					FromOrchestra::Communication { msg } => {
						handle_communication(ctx, per_relay_parent, msg, metrics).await?;
					},
				}
			},
			hash = inherent_delays.select_next_some() => {
				if let Some(state) = per_relay_parent.get_mut(&hash) {
					state.is_inherent_ready = true;

					gum::trace!(
						target: LOG_TARGET,
						relay_parent = ?hash,
						"Inherent Data became ready"
					);

					let return_senders = std::mem::take(&mut state.awaiting_inherent);
					if !return_senders.is_empty() {
						send_inherent_data_bg(ctx, &state, return_senders, metrics.clone()).await?;
					}
				}
			}
		}
	}
}

async fn handle_active_leaves_update(
	sender: &mut impl overseer::ProvisionerSenderTrait,
	update: ActiveLeavesUpdate,
	per_relay_parent: &mut HashMap<Hash, PerRelayParent>,
	per_session: &mut LruMap<SessionIndex, PerSession>,
	inherent_delays: &mut InherentDelays,
) -> Result<(), Error> {
	gum::trace!(target: LOG_TARGET, "Handle ActiveLeavesUpdate");
	for deactivated in &update.deactivated {
		per_relay_parent.remove(deactivated);
	}

	if let Some(leaf) = update.activated {
		let session_index = request_session_index_for_child(leaf.hash, sender)
			.await
			.await
			.map_err(Error::CanceledSessionIndex)??;
		if per_session.get(&session_index).is_none() {
			let prospective_parachains_mode =
				prospective_parachains_mode(sender, leaf.hash).await?;
			let elastic_scaling_mvp = request_node_features(leaf.hash, session_index, sender)
				.await?
				.unwrap_or(NodeFeatures::EMPTY)
				.get(FeatureIndex::ElasticScalingMVP as usize)
				.map(|b| *b)
				.unwrap_or(false);

			per_session.insert(
				session_index,
				PerSession { prospective_parachains_mode, elastic_scaling_mvp },
			);
		}

		let session_info = per_session.get(&session_index).expect("Just inserted");

		gum::trace!(target: LOG_TARGET, leaf_hash=?leaf.hash, "Adding delay");
		let delay_fut = Delay::new(PRE_PROPOSE_TIMEOUT).map(move |_| leaf.hash).boxed();
		per_relay_parent.insert(leaf.hash, PerRelayParent::new(leaf, session_info));
		inherent_delays.push(delay_fut);
	}

	Ok(())
}

#[overseer::contextbounds(Provisioner, prefix = self::overseer)]
async fn handle_communication<Context>(
	ctx: &mut Context,
	per_relay_parent: &mut HashMap<Hash, PerRelayParent>,
	message: ProvisionerMessage,
	metrics: &Metrics,
) -> Result<(), Error> {
	match message {
		ProvisionerMessage::RequestInherentData(relay_parent, return_sender) => {
			gum::trace!(target: LOG_TARGET, ?relay_parent, "Inherent data got requested.");

			if let Some(state) = per_relay_parent.get_mut(&relay_parent) {
				if state.is_inherent_ready {
					gum::trace!(target: LOG_TARGET, ?relay_parent, "Calling send_inherent_data.");
					send_inherent_data_bg(ctx, &state, vec![return_sender], metrics.clone())
						.await?;
				} else {
					gum::trace!(
						target: LOG_TARGET,
						?relay_parent,
						"Queuing inherent data request (inherent data not yet ready)."
					);
					state.awaiting_inherent.push(return_sender);
				}
			}
		},
		ProvisionerMessage::ProvisionableData(relay_parent, data) => {
			if let Some(state) = per_relay_parent.get_mut(&relay_parent) {
				let span = state.span.child("provisionable-data");
				let _timer = metrics.time_provisionable_data();

				gum::trace!(target: LOG_TARGET, ?relay_parent, "Received provisionable data.");

				note_provisionable_data(state, &span, data);
			}
		},
	}

	Ok(())
}

#[overseer::contextbounds(Provisioner, prefix = self::overseer)]
async fn send_inherent_data_bg<Context>(
	ctx: &mut Context,
	per_relay_parent: &PerRelayParent,
	return_senders: Vec<oneshot::Sender<ProvisionerInherentData>>,
	metrics: Metrics,
) -> Result<(), Error> {
	let leaf = per_relay_parent.leaf.clone();
	let signed_bitfields = per_relay_parent.signed_bitfields.clone();
	let backed_candidates = per_relay_parent.backed_candidates.clone();
	let mode = per_relay_parent.prospective_parachains_mode;
	let elastic_scaling_mvp = per_relay_parent.elastic_scaling_mvp;
	let span = per_relay_parent.span.child("req-inherent-data");

	let mut sender = ctx.sender().clone();

	let bg = async move {
		let _span = span;
		let _timer = metrics.time_request_inherent_data();

		gum::trace!(
			target: LOG_TARGET,
			relay_parent = ?leaf.hash,
			"Sending inherent data in background."
		);

		let send_result = send_inherent_data(
			&leaf,
			&signed_bitfields,
			&backed_candidates,
			mode,
			elastic_scaling_mvp,
			return_senders,
			&mut sender,
			&metrics,
		) // Make sure call is not taking forever:
		.timeout(SEND_INHERENT_DATA_TIMEOUT)
		.map(|v| match v {
			Some(r) => r,
			None => Err(Error::SendInherentDataTimeout),
		});

		match send_result.await {
			Err(err) => {
				if let Error::CanceledBackedCandidates(_) = err {
					gum::debug!(
						target: LOG_TARGET,
						err = ?err,
						"Failed to assemble or send inherent data - block got likely obsoleted already."
					);
				} else {
					gum::warn!(target: LOG_TARGET, err = ?err, "failed to assemble or send inherent data");
				}
				metrics.on_inherent_data_request(Err(()));
			},
			Ok(()) => {
				metrics.on_inherent_data_request(Ok(()));
				gum::debug!(
					target: LOG_TARGET,
					signed_bitfield_count = signed_bitfields.len(),
					leaf_hash = ?leaf.hash,
					"inherent data sent successfully"
				);
				metrics.observe_inherent_data_bitfields_count(signed_bitfields.len());
			},
		}
	};

	ctx.spawn("send-inherent-data", bg.boxed())
		.map_err(|_| Error::FailedToSpawnBackgroundTask)?;

	Ok(())
}

fn note_provisionable_data(
	per_relay_parent: &mut PerRelayParent,
	span: &jaeger::Span,
	provisionable_data: ProvisionableData,
) {
	match provisionable_data {
		ProvisionableData::Bitfield(_, signed_bitfield) =>
			per_relay_parent.signed_bitfields.push(signed_bitfield),
		ProvisionableData::BackedCandidate(backed_candidate) => {
			let candidate_hash = backed_candidate.hash();
			gum::trace!(
				target: LOG_TARGET,
				?candidate_hash,
				para = ?backed_candidate.descriptor().para_id,
				"noted backed candidate",
			);
			let _span = span
				.child("provisionable-backed")
				.with_candidate(candidate_hash)
				.with_para_id(backed_candidate.descriptor().para_id);
			per_relay_parent.backed_candidates.push(backed_candidate);
		},
		// We choose not to punish these forms of misbehavior for the time being.
		// Risks from misbehavior are sufficiently mitigated at the protocol level
		// via reputation changes. Punitive actions here may become desirable
		// enough to dedicate time to in the future.
		ProvisionableData::MisbehaviorReport(_, _, _) => {},
		// We wait and do nothing here, preferring to initiate a dispute after the
		// parablock candidate is included for the following reasons:
		//
		// 1. A dispute for a candidate triggered at any point before the candidate
		// has been made available, including the backing stage, can't be
		// guaranteed to conclude. Non-concluding disputes are unacceptable.
		// 2. Candidates which haven't been made available don't pose a security
		// risk as they can not be included, approved, or finalized.
		//
		// Currently we rely on approval checkers to trigger disputes for bad
		// parablocks once they are included. But we can do slightly better by
		// allowing disagreeing backers to record their disagreement and initiate a
		// dispute once the parablock in question has been included. This potential
		// change is tracked by: https://github.com/paritytech/polkadot/issues/3232
		ProvisionableData::Dispute(_, _) => {},
	}
}

type CoreAvailability = BitVec<u8, bitvec::order::Lsb0>;

/// The provisioner is the subsystem best suited to choosing which specific
/// backed candidates and availability bitfields should be assembled into the
/// block. To engage this functionality, a
/// `ProvisionerMessage::RequestInherentData` is sent; the response is a set of
/// non-conflicting candidates and the appropriate bitfields. Non-conflicting
/// means that there are never two distinct parachain candidates included for
/// the same parachain and that new parachain candidates cannot be included
/// until the previous one either gets declared available or expired.
///
/// The main complication here is going to be around handling
/// occupied-core-assumptions. We might have candidates that are only
/// includable when some bitfields are included. And we might have candidates
/// that are not includable when certain bitfields are included.
///
/// When we're choosing bitfields to include, the rule should be simple:
/// maximize availability. So basically, include all bitfields. And then
/// choose a coherent set of candidates along with that.
async fn send_inherent_data(
	leaf: &ActivatedLeaf,
	bitfields: &[SignedAvailabilityBitfield],
	candidates: &[CandidateReceipt],
	prospective_parachains_mode: ProspectiveParachainsMode,
	elastic_scaling_mvp: bool,
	return_senders: Vec<oneshot::Sender<ProvisionerInherentData>>,
	from_job: &mut impl overseer::ProvisionerSenderTrait,
	metrics: &Metrics,
) -> Result<(), Error> {
	gum::trace!(
		target: LOG_TARGET,
		relay_parent = ?leaf.hash,
		"Requesting availability cores"
	);
	let availability_cores = request_availability_cores(leaf.hash, from_job)
		.await
		.await
		.map_err(|err| Error::CanceledAvailabilityCores(err))??;

	gum::trace!(
		target: LOG_TARGET,
		relay_parent = ?leaf.hash,
		"Selecting disputes"
	);

	debug_assert!(
		has_required_runtime(
			from_job,
			leaf.hash,
			PRIORITIZED_SELECTION_RUNTIME_VERSION_REQUIREMENT,
		)
		.await,
		"randomized selection no longer supported, please upgrade your runtime!"
	);

	let disputes = disputes::prioritized_selection::select_disputes(from_job, metrics, leaf).await;

	gum::trace!(
		target: LOG_TARGET,
		relay_parent = ?leaf.hash,
		"Selected disputes"
	);

	let bitfields = select_availability_bitfields(&availability_cores, bitfields, &leaf.hash);

	gum::trace!(
		target: LOG_TARGET,
		relay_parent = ?leaf.hash,
		"Selected bitfields"
	);

	let candidates = select_candidates(
		&availability_cores,
		&bitfields,
		candidates,
		prospective_parachains_mode,
		elastic_scaling_mvp,
		leaf.hash,
		from_job,
	)
	.await?;

	gum::trace!(
		target: LOG_TARGET,
		relay_parent = ?leaf.hash,
		"Selected candidates"
	);

	gum::debug!(
		target: LOG_TARGET,
		availability_cores_len = availability_cores.len(),
		disputes_count = disputes.len(),
		bitfields_count = bitfields.len(),
		candidates_count = candidates.len(),
		leaf_hash = ?leaf.hash,
		"inherent data prepared",
	);

	let inherent_data =
		ProvisionerInherentData { bitfields, backed_candidates: candidates, disputes };

	gum::trace!(
		target: LOG_TARGET,
		relay_parent = ?leaf.hash,
		"Sending back inherent data to requesters."
	);

	for return_sender in return_senders {
		return_sender
			.send(inherent_data.clone())
			.map_err(|_data| Error::InherentDataReturnChannel)?;
	}

	Ok(())
}

/// In general, we want to pick all the bitfields. However, we have the following constraints:
///
/// - not more than one per validator
/// - each 1 bit must correspond to an occupied core
///
/// If we have too many, an arbitrary selection policy is fine. For purposes of maximizing
/// availability, we pick the one with the greatest number of 1 bits.
///
/// Note: This does not enforce any sorting precondition on the output; the ordering there will be
/// unrelated to the sorting of the input.
fn select_availability_bitfields(
	cores: &[CoreState],
	bitfields: &[SignedAvailabilityBitfield],
	leaf_hash: &Hash,
) -> Vec<SignedAvailabilityBitfield> {
	let mut selected: BTreeMap<ValidatorIndex, SignedAvailabilityBitfield> = BTreeMap::new();

	gum::debug!(
		target: LOG_TARGET,
		bitfields_count = bitfields.len(),
		?leaf_hash,
		"bitfields count before selection"
	);

	'a: for bitfield in bitfields.iter().cloned() {
		if bitfield.payload().0.len() != cores.len() {
			gum::debug!(target: LOG_TARGET, ?leaf_hash, "dropping bitfield due to length mismatch");
			continue
		}

		let is_better = selected
			.get(&bitfield.validator_index())
			.map_or(true, |b| b.payload().0.count_ones() < bitfield.payload().0.count_ones());

		if !is_better {
			gum::trace!(
				target: LOG_TARGET,
				val_idx = bitfield.validator_index().0,
				?leaf_hash,
				"dropping bitfield due to duplication - the better one is kept"
			);
			continue
		}

		for (idx, _) in cores.iter().enumerate().filter(|v| !v.1.is_occupied()) {
			// Bit is set for an unoccupied core - invalid
			if *bitfield.payload().0.get(idx).as_deref().unwrap_or(&false) {
				gum::debug!(
					target: LOG_TARGET,
					val_idx = bitfield.validator_index().0,
					?leaf_hash,
					"dropping invalid bitfield - bit is set for an unoccupied core"
				);
				continue 'a
			}
		}

		let _ = selected.insert(bitfield.validator_index(), bitfield);
	}

	gum::debug!(
		target: LOG_TARGET,
		?leaf_hash,
		"selected {} of all {} bitfields (each bitfield is from a unique validator)",
		selected.len(),
		bitfields.len()
	);

	selected.into_values().collect()
}

/// Selects candidates from tracked ones to note in a relay chain block.
///
/// Should be called when prospective parachains are disabled.
async fn select_candidate_hashes_from_tracked(
	availability_cores: &[CoreState],
	bitfields: &[SignedAvailabilityBitfield],
	candidates: &[CandidateReceipt],
	relay_parent: Hash,
	sender: &mut impl overseer::ProvisionerSenderTrait,
) -> Result<HashMap<ParaId, Vec<(CandidateHash, Hash)>>, Error> {
	let block_number = get_block_number_under_construction(relay_parent, sender).await?;

	let mut selected_candidates =
		HashMap::with_capacity(candidates.len().min(availability_cores.len()));

	gum::debug!(
		target: LOG_TARGET,
		leaf_hash=?relay_parent,
		n_candidates = candidates.len(),
		"Candidate receipts (before selection)",
	);

	for (core_idx, core) in availability_cores.iter().enumerate() {
		let (scheduled_core, assumption) = match core {
			CoreState::Scheduled(scheduled_core) => (scheduled_core, OccupiedCoreAssumption::Free),
			CoreState::Occupied(occupied_core) => {
				if bitfields_indicate_availability(core_idx, bitfields, &occupied_core.availability)
				{
					if let Some(ref scheduled_core) = occupied_core.next_up_on_available {
						(scheduled_core, OccupiedCoreAssumption::Included)
					} else {
						continue
					}
				} else {
					if occupied_core.time_out_at != block_number {
						continue
					}
					if let Some(ref scheduled_core) = occupied_core.next_up_on_time_out {
						(scheduled_core, OccupiedCoreAssumption::TimedOut)
					} else {
						continue
					}
				}
			},
			CoreState::Free => continue,
		};

		if selected_candidates.contains_key(&scheduled_core.para_id) {
			// We already picked a candidate for this parachain. Elastic scaling only works with
			// prospective parachains mode.
			continue
		}

		let validation_data = match request_persisted_validation_data(
			relay_parent,
			scheduled_core.para_id,
			assumption,
			sender,
		)
		.await
		.await
		.map_err(|err| Error::CanceledPersistedValidationData(err))??
		{
			Some(v) => v,
			None => continue,
		};

		let computed_validation_data_hash = validation_data.hash();

		// we arbitrarily pick the first of the backed candidates which match the appropriate
		// selection criteria
		if let Some(candidate) = candidates.iter().find(|backed_candidate| {
			let descriptor = &backed_candidate.descriptor;
			descriptor.para_id == scheduled_core.para_id &&
				descriptor.persisted_validation_data_hash == computed_validation_data_hash
		}) {
			let candidate_hash = candidate.hash();
			gum::trace!(
				target: LOG_TARGET,
				leaf_hash=?relay_parent,
				?candidate_hash,
				para = ?candidate.descriptor.para_id,
				core = core_idx,
				"Selected candidate receipt",
			);

			selected_candidates.insert(
				candidate.descriptor.para_id,
				vec![(candidate_hash, candidate.descriptor.relay_parent)],
			);
		}
	}

	Ok(selected_candidates)
}

/// Requests backable candidates from Prospective Parachains subsystem
/// based on core states.
///
/// Should be called when prospective parachains are enabled.
async fn request_backable_candidates(
	availability_cores: &[CoreState],
	elastic_scaling_mvp: bool,
	bitfields: &[SignedAvailabilityBitfield],
	relay_parent: Hash,
	sender: &mut impl overseer::ProvisionerSenderTrait,
) -> Result<HashMap<ParaId, Vec<(CandidateHash, Hash)>>, Error> {
	let block_number = get_block_number_under_construction(relay_parent, sender).await?;

	// Record how many cores are scheduled for each paraid. Use a BTreeMap because
	// we'll need to iterate through them.
	let mut scheduled_cores_per_para: BTreeMap<ParaId, usize> = BTreeMap::new();
	// The on-chain ancestors of a para present in availability-cores.
	let mut ancestors: HashMap<ParaId, Ancestors> =
		HashMap::with_capacity(availability_cores.len());

	for (core_idx, core) in availability_cores.iter().enumerate() {
		let core_idx = CoreIndex(core_idx as u32);
		match core {
			CoreState::Scheduled(scheduled_core) => {
				*scheduled_cores_per_para.entry(scheduled_core.para_id).or_insert(0) += 1;
			},
			CoreState::Occupied(occupied_core) => {
				let is_available = bitfields_indicate_availability(
					core_idx.0 as usize,
					bitfields,
					&occupied_core.availability,
				);

				if is_available {
					ancestors
						.entry(occupied_core.para_id())
						.or_default()
						.insert(occupied_core.candidate_hash);

					if let Some(ref scheduled_core) = occupied_core.next_up_on_available {
						// Request a new backable candidate for the newly scheduled para id.
						*scheduled_cores_per_para.entry(scheduled_core.para_id).or_insert(0) += 1;
					}
				} else if occupied_core.time_out_at <= block_number {
					// Timed out before being available.

					if let Some(ref scheduled_core) = occupied_core.next_up_on_time_out {
						// Candidate's availability timed out, practically same as scheduled.
						*scheduled_cores_per_para.entry(scheduled_core.para_id).or_insert(0) += 1;
					}
				} else {
					// Not timed out and not available.
					ancestors
						.entry(occupied_core.para_id())
						.or_default()
						.insert(occupied_core.candidate_hash);
				}
			},
			CoreState::Free => continue,
		};
	}

	let mut selected_candidates: HashMap<ParaId, Vec<(CandidateHash, Hash)>> =
		HashMap::with_capacity(scheduled_cores_per_para.len());

	for (para_id, core_count) in scheduled_cores_per_para {
		let para_ancestors = ancestors.remove(&para_id).unwrap_or_default();

		// If elastic scaling MVP is disabled, only allow one candidate per parachain.
		if !elastic_scaling_mvp && core_count > 1 {
			continue
		}

		let response = get_backable_candidates(
			relay_parent,
			para_id,
			para_ancestors,
			core_count as u32,
			sender,
		)
		.await?;

		if response.is_empty() {
			gum::debug!(
				target: LOG_TARGET,
				leaf_hash = ?relay_parent,
				?para_id,
				"No backable candidate returned by prospective parachains",
			);
			continue
		}

		selected_candidates.insert(para_id, response);
	}

	Ok(selected_candidates)
}

/// Determine which cores are free, and then to the degree possible, pick a candidate appropriate to
/// each free core.
async fn select_candidates(
	availability_cores: &[CoreState],
	bitfields: &[SignedAvailabilityBitfield],
	candidates: &[CandidateReceipt],
	prospective_parachains_mode: ProspectiveParachainsMode,
	elastic_scaling_mvp: bool,
	relay_parent: Hash,
	sender: &mut impl overseer::ProvisionerSenderTrait,
) -> Result<Vec<BackedCandidate>, Error> {
	gum::trace!(target: LOG_TARGET,
		leaf_hash=?relay_parent,
		"before GetBackedCandidates");

	let selected_candidates = match prospective_parachains_mode {
		ProspectiveParachainsMode::Enabled { .. } =>
			request_backable_candidates(
				availability_cores,
				elastic_scaling_mvp,
				bitfields,
				relay_parent,
				sender,
			)
			.await?,
		ProspectiveParachainsMode::Disabled =>
			select_candidate_hashes_from_tracked(
				availability_cores,
				bitfields,
				&candidates,
				relay_parent,
				sender,
			)
			.await?,
	};
	gum::debug!(target: LOG_TARGET, ?selected_candidates, "Got backable candidates");

	// now get the backed candidates corresponding to these candidate receipts
	let (tx, rx) = oneshot::channel();
	sender.send_unbounded_message(CandidateBackingMessage::GetBackableCandidates(
		selected_candidates.clone(),
		tx,
	));
	let candidates = rx.await.map_err(|err| Error::CanceledBackedCandidates(err))?;
	gum::trace!(target: LOG_TARGET, leaf_hash=?relay_parent,
				"Got {} backed candidates", candidates.len());

	// keep only one candidate with validation code.
	let mut with_validation_code = false;
	// merge the candidates into a common collection, preserving the order
	let mut merged_candidates = Vec::with_capacity(availability_cores.len());

	for para_candidates in candidates.into_values() {
		for candidate in para_candidates {
			if candidate.candidate().commitments.new_validation_code.is_some() {
				if with_validation_code {
					break
				} else {
					with_validation_code = true;
				}
			}

			merged_candidates.push(candidate);
		}
	}

	gum::debug!(
		target: LOG_TARGET,
		n_candidates = merged_candidates.len(),
		n_cores = availability_cores.len(),
		?relay_parent,
		"Selected backed candidates",
	);

	Ok(merged_candidates)
}

/// Produces a block number 1 higher than that of the relay parent
/// in the event of an invalid `relay_parent`, returns `Ok(0)`
async fn get_block_number_under_construction(
	relay_parent: Hash,
	sender: &mut impl overseer::ProvisionerSenderTrait,
) -> Result<BlockNumber, Error> {
	let (tx, rx) = oneshot::channel();
	sender.send_message(ChainApiMessage::BlockNumber(relay_parent, tx)).await;

	match rx.await.map_err(|err| Error::CanceledBlockNumber(err))? {
		Ok(Some(n)) => Ok(n + 1),
		Ok(None) => Ok(0),
		Err(err) => Err(err.into()),
	}
}

/// Requests backable candidates from Prospective Parachains based on
/// the given ancestors in the fragment chain. The ancestors may not be ordered.
async fn get_backable_candidates(
	relay_parent: Hash,
	para_id: ParaId,
	ancestors: Ancestors,
	count: u32,
	sender: &mut impl overseer::ProvisionerSenderTrait,
) -> Result<Vec<(CandidateHash, Hash)>, Error> {
	let (tx, rx) = oneshot::channel();
	sender
		.send_message(ProspectiveParachainsMessage::GetBackableCandidates(
			relay_parent,
			para_id,
			count,
			ancestors,
			tx,
		))
		.await;

	rx.await.map_err(Error::CanceledBackableCandidates)
}

/// The availability bitfield for a given core is the transpose
/// of a set of signed availability bitfields. It goes like this:
///
/// - construct a transverse slice along `core_idx`
/// - bitwise-or it with the availability slice
/// - count the 1 bits, compare to the total length; true on 2/3+
fn bitfields_indicate_availability(
	core_idx: usize,
	bitfields: &[SignedAvailabilityBitfield],
	availability: &CoreAvailability,
) -> bool {
	let mut availability = availability.clone();
	let availability_len = availability.len();

	for bitfield in bitfields {
		let validator_idx = bitfield.validator_index().0 as usize;
		match availability.get_mut(validator_idx) {
			None => {
				// in principle, this function might return a `Result<bool, Error>` so that we can
				// more clearly express this error condition however, in practice, that would just
				// push off an error-handling routine which would look a whole lot like this one.
				// simpler to just handle the error internally here.
				gum::warn!(
					target: LOG_TARGET,
					validator_idx = %validator_idx,
					availability_len = %availability_len,
					"attempted to set a transverse bit at idx {} which is greater than bitfield size {}",
					validator_idx,
					availability_len,
				);

				return false
			},
			Some(mut bit_mut) => *bit_mut |= bitfield.payload().0[core_idx],
		}
	}

	3 * availability.count_ones() >= 2 * availability.len()
}
