// This file is part of Substrate.

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

//! # Stake Tracker Pallet
//!
//! The stake-tracker pallet is responsible to keep track of the voter's stake and target's approval
//! voting in the staking system. It provides an easy way to other pallets to query the list of
//! strictly sorted targets based on their approvals.
//!
//! ## Overview
//!
//! The stake-tracker pallet listens to staking events through implementing the [`OnStakingUpdate`]
//! trait. Based on the emitted events, the goal of this pallet is to maintain a **strictly**
//! sorted list of targets by approval voting. This pallet may also update a voter list and their
//! scores, based on the [`crate::VoterUpdateMode`] configuration.
//!
//! For the voter list, the [`crate::VoterUpdateMode`] defines the type of sortition of the list,
//! namely:
//!
//! - [`crate::VoterUpdateMode::Lazy`]: will skip the score update in the voter list.
//! - [`crate::VoterUpdateMode::Strict`]: will ensure that the score updates are kept sorted
//! for the corresponding list. In this case, the [`Config::VoterList`] is *strictly*
//! sorted by [`SortedListProvider::Score`] (note: from the time the sorting mode is strict).
//!
//! Note that insertions and removals of voter nodes will be executed regardless of the sorting
//! mode.
//!
//! ## Goals
//!
//! The [`OnStakingUpdate`] implementation (in strict mode) aims to achieve the following goals:
//!
//! * The [`Config::TargetList`] keeps a sorted list of validators, *strictly* sorted by approvals
//! (which include self-vote and nominations' stake).
//! * The [`Config::VoterList`] keeps a list of voters, *stricly* sorted by bonded stake if it has
//! [`crate::VoterUpdateMode::Strict`] mode enabled, otherwise the list is kept lazily sorted.
//! * The [`Config::TargetList`] sorting must be *always* kept up to date, even in the event of new
//! nomination updates, nominator/validator slashes and rewards. This pallet *must* ensure that the
//! scores of the targets and voters are always up to date and thus, that the targets and voters in
//! the lists are sorted by score at all time.
//!
//! Note that from the POV of this pallet, staking actions may result in one or multiple updates to
//! [`Config::VoterList`] and/or [`Config::TargetList`] state. If a set of staking updates require
//! too much weight to execute (e.g. at nominator's rewards payout or at slashes), the event emitter
//! should handle that in some way (e.g. buffering events and implementing a multi-block event
//! emitter).
//!
//! ## Lazy target approvals' updates
//!
//! This pallet exposes a configuration that defines a threshold below which the target score
//! *should not* be updated automatically. The `Config::ScoreStrictUpdateThreshold` defines said
//! threshold. If a target stake update is below the threshold, the stake update is buffered in
//! the `UnsettledTargetScore` storage map while the target list is not affected. Multiple
//! approvals updates can be buffered for the same target. Calling `Call::settle` will setttle the
//! buffered approvals tally for a given target.
//!
//! Setting `Config::ScoreStrictUpdateThreshold` to `None` disables the stake approvals buffering.
//!
//! ## Staker status and list invariants
//!
//! * A [`sp_staking::StakerStatus::Nominator`] is part of the voter list and its self-stake is the
//! voter list's score. In addition, if the `VoterList` is in strict mode, the voters' scores are up
//! to date with the current stake returned by [`sp_staking::StakingInterface`].
//! * A [`sp_staking::StakerStatus::Validator`] is part of both voter and target list. In addition,
//! its approvals score (nominations + self-stake) is kept up to date as the target list's score.
//! * A [`sp_staking::StakerStatus::Idle`] may have a target list's score while other stakers
//!   nominate the idle validator.
//! * A "dangling" target, which is not an active staker anymore (i.e. not bonded), may still have
//!   an associated target list score. This may happen when active nominators are still nominating
//!   the target after the validator unbonded. The target list's node and score will automatically
//!   be removed onced all the voters stop nominating the unbonded account (i.e. the target's score
//!   drops to 0).
//!
//! ## Expectations
//!
//! For the goals and invariants to be respected, this pallet expects the following:
//!
//! - **Event emitting order**: It is important to ensure that the events are emitted from staking
//!   (i.e. the calls into
//! [`OnStakingUpdate`]) *after* the staking ledger has been updated by the caller, since the new
//! state will be fetched and used to update the sorted lists accordingly.
//! - **Deduplicate nominations**: The nominations should be deduplicate. This pallet handles
//! nominations of voters from the underlying staking system. The nominations may be retrieved
//! through the [`sp_staking::StakingInterface`] and/or through the [`sp_staking::OnStakingUpdate`]
//! methods. This pallet expects that there are no duplicate nominations for each voter.

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmarking;

#[cfg(test)]
pub mod mock;
#[cfg(test)]
mod tests;

pub mod weights;

extern crate alloc;

use alloc::{collections::btree_map::BTreeMap, vec, vec::Vec};
use frame_election_provider_support::{ExtendedBalance, SortedListProvider, VoteWeight};
use frame_support::{
	defensive,
	pallet_prelude::*,
	traits::{
		fungible::{Inspect as FnInspect, Mutate as FnMutate},
		Defensive, DefensiveSaturating,
	},
};
use frame_system::pallet_prelude::*;
use sp_runtime::traits::Zero;
use sp_staking::{
	currency_to_vote::CurrencyToVote, OnStakingUpdate, Stake, StakerStatus, StakingInterface,
};
use sp_std::ops::Add;
pub use weights::WeightInfo;

/// The balance type of this pallet.
pub type BalanceOf<T> = <<T as Config>::Staking as StakingInterface>::Balance;
/// The account ID of this pallet.
pub type AccountIdOf<T> = <T as frame_system::Config>::AccountId;
/// Score of a sorted list provider.
pub type ScoreOf<T> = <<T as Config>::TargetList as SortedListProvider<AccountIdOf<T>>>::Score;
/// List of current unsettled target scores.
pub type UnsettledList<T> = Vec<(AccountIdOf<T>, StakeImbalance<ExtendedBalance>)>;

/// Represents a stake imbalance to be applied to a staker's score.
#[derive(TypeInfo, Copy, Debug, Clone, Encode, Decode, PartialEq, MaxEncodedLen)]
pub enum StakeImbalance<Score> {
	/// Represents the reduction of stake by `Score`.
	Negative(Score),
	/// Represents the increase of stake by `Score`.
	Positive(Score),
	/// No stake imbalance.
	Zero,
}

impl<S: Zero> Default for StakeImbalance<S> {
	fn default() -> Self {
		Self::Positive(Zero::zero())
	}
}

impl<Score: PartialOrd + Zero + Copy + DefensiveSaturating> Add for StakeImbalance<Score> {
	type Output = Self;

	fn add(self, other: Self) -> Self {
		match (self, other) {
			(Self::Positive(x), Self::Positive(y)) => Self::Positive(x.defensive_saturating_add(y)),
			(Self::Negative(x), Self::Negative(y)) => Self::Negative(x.defensive_saturating_add(y)),
			(Self::Positive(x), Self::Negative(y)) =>
				if x > y {
					Self::Positive(x.defensive_saturating_sub(y))
				} else if x < y {
					Self::Negative(y.defensive_saturating_sub(x))
				} else {
					Self::Zero
				},
			(Self::Negative(x), Self::Positive(y)) =>
				if x > y {
					Self::Negative(x.defensive_saturating_sub(y))
				} else if x < y {
					Self::Positive(y.defensive_saturating_sub(x))
				} else {
					Self::Zero
				},
			(Self::Zero, _) => other,
			(_, Self::Zero) => self,
		}
	}
}

impl<V: PartialOrd + DefensiveSaturating + Zero + Copy> StakeImbalance<V> {
	/// Constructor for a stake imbalance instance based on the previous and next unsigned value.
	fn from(prev: V, new: V) -> Self {
		if prev > new {
			StakeImbalance::Negative(prev.defensive_saturating_sub(new))
		} else {
			StakeImbalance::Positive(new.defensive_saturating_sub(prev))
		}
	}

	/// Returns the unsigned value of a staking balance instance.
	fn value(&self) -> V {
		match self {
			Self::Positive(score) | Self::Negative(score) => *score,
			Self::Zero => Zero::zero(),
		}
	}
}

/// Defines the sorting mode of sorted list providers.
#[derive(Copy, Clone, Debug)]
pub enum VoterUpdateMode {
	/// All score update events will be automatically reflected in the sorted list.
	Strict,
	/// Score update events are *not* be automatically reflected in the sorted list. However, node
	/// insertion and removals are reflected in the list.
	Lazy,
}

impl VoterUpdateMode {
	fn is_strict_mode(&self) -> bool {
		matches!(self, Self::Strict)
	}
}

#[frame_support::pallet]
pub mod pallet {
	use crate::*;

	/// The current storage version.
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The stake balance.
		type Currency: FnInspect<Self::AccountId, Balance = BalanceOf<Self>>
			+ FnMutate<Self::AccountId>;

		/// The staking interface.
		type Staking: StakingInterface<AccountId = Self::AccountId>;

		/// A sorted list provider for staking voters that is kept up to date by this pallet.
		// [`Self::VoterUpdateMode`] defines whether this pallet will keep the voter list
		// *strictly ordered* for every nominator stake updateor lazily ordered.
		type VoterList: SortedListProvider<Self::AccountId, Score = VoteWeight>;

		/// A sorted list provider for staking targets that is ketp *always* sorted by the target's
		/// stake approvals.
		type TargetList: SortedListProvider<Self::AccountId, Score = ExtendedBalance>;

		/// The voter list update mode.
		type VoterUpdateMode: Get<VoterUpdateMode>;

		/// Score threshold which defines whether the approvals should be updated on buffered.
		///
		/// If the approvals score to be updated is higher than `ScoreStrictUpdateThreshold`,
		/// update the target list. Otherwise, buffer the update.
		type ScoreStrictUpdateThreshold: Get<Option<ExtendedBalance>>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;
	}

	/// Map with unsettled score for targets.
	///
	/// This map keeps track of unsettled score for targets.
	#[pallet::storage]
	pub type UnsettledTargetScore<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, StakeImbalance<ScoreOf<T>>>;

	#[pallet::error]
	#[derive(PartialEq)]
	pub enum Error<T> {
		/// Account is not a target.
		NotTarget,
		/// No unsettled score for a given target.
		NoScoreToSettle,
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Settles a buffered target approvals imbalance.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::settle())]
		pub fn settle(origin: OriginFor<T>, who: AccountIdOf<T>) -> DispatchResult {
			let _ = ensure_signed(origin)?;

			ensure!(T::TargetList::contains(&who), Error::<T>::NotTarget);
			ensure!(UnsettledTargetScore::<T>::contains_key(&who), Error::<T>::NoScoreToSettle);

			Self::do_settle(&who)?;
			Ok(())
		}
	}

	impl<T: Config> Pallet<T> {
		/// Settles buffered score for a target, if it exists.
		pub(crate) fn do_settle(who: &T::AccountId) -> Result<(), Error<T>> {
			UnsettledTargetScore::<T>::try_mutate_exists(who, |maybe_imbalance| {
				let imbalance = maybe_imbalance.ok_or(Error::<T>::NoScoreToSettle)?;

				Self::update_target_score_unchecked(who, imbalance);
				*maybe_imbalance = None;
				Ok(())
			})?;

			Ok(())
		}

		/// Updates the stake of a voter.
		///
		/// NOTE: This method expects `nominations` to be deduplicated, otherwise the approvals
		/// stakes of the duplicated target may become higher than expected silently.
		pub(crate) fn do_stake_update_voter(
			who: &T::AccountId,
			prev_stake: Option<Stake<BalanceOf<T>>>,
			stake: Stake<BalanceOf<T>>,
			nominations: Vec<T::AccountId>,
		) {
			defensive_assert!(!Self::has_duplicate_nominations(nominations.clone()));

			let voter_weight = Self::to_vote(stake.active);

			// if voter list is in strict sorting mode, update the voter score too.
			if T::VoterUpdateMode::get().is_strict_mode() {
				let _ = T::VoterList::on_update(who, voter_weight).defensive_proof(
					"staker should exist in VoterList, as per the contract \
                            with staking.",
				);
			}

			let stake_imbalance = StakeImbalance::from(
				prev_stake.map_or(Default::default(), |s| Self::to_vote(s.active).into()),
				voter_weight.into(),
			);

			// updates vote weight of nominated targets accordingly. Note: this will
			// update the score of up to `T::MaxNominations` validators.
			for target in nominations.into_iter() {
				Self::update_target_score(&target, stake_imbalance);
			}
		}

		/// Updates the stake of a target.
		pub(crate) fn do_stake_update_target(
			who: &T::AccountId,
			prev_stake: Option<Stake<BalanceOf<T>>>,
			stake: Stake<BalanceOf<T>>,
		) {
			let voter_weight = Self::to_vote(stake.active).into();
			let stake_imbalance = StakeImbalance::from(
				prev_stake.map_or(Default::default(), |s| Self::to_vote(s.active).into()),
				voter_weight,
			);

			Self::update_target_score(who, stake_imbalance);

			// validator is both a target and a voter. update the voter score if the voter list
			// is in strict mode.
			if T::VoterUpdateMode::get().is_strict_mode() {
				let _ = T::VoterList::on_update(who, Self::to_vote(stake.active)).defensive_proof(
					"the staker should exist in VoterList, as per the \
                            contract with staking.",
				);
			}
		}

		/// Updates a target's score by increasing/decreasing an imbalance of the current score in
		/// the target list.
		pub(crate) fn update_target_score(
			who: &T::AccountId,
			imbalance: StakeImbalance<ExtendedBalance>,
		) {
			// if target list does not contain target, add it and proceed.
			if !T::TargetList::contains(who) {
				let _ = T::TargetList::on_insert(who.clone(), Zero::zero()).defensive_proof(
					"staker does not yet exist in the list as per check above; qed.",
				);
			}

			// updates the target list OR buffers the score depending on whether the score
			// imbalance is higher than the update threshold (if set).
			if let Some(update_threshold) = T::ScoreStrictUpdateThreshold::get() {
				if imbalance.value() > update_threshold {
					Self::update_target_score_unchecked(who, imbalance);
				} else {
					// buffer the approvals update in `UnsettledTargetScore` map.
					UnsettledTargetScore::<T>::insert(
						who,
						UnsettledTargetScore::<T>::get(who).unwrap_or_default().add(imbalance),
					);
				}
			} else {
				Self::update_target_score_unchecked(who, imbalance);
			}
		}

		pub(crate) fn update_target_score_unchecked(
			who: &T::AccountId,
			imbalance: StakeImbalance<ExtendedBalance>,
		) {
			match imbalance {
				StakeImbalance::Positive(imbalance) => {
					let _ = T::TargetList::on_increase(who, imbalance).defensive_proof(
						"staker should exist in the list, otherwise returned earlier.",
					);
				},
				StakeImbalance::Negative(imbalance) => {
					if let Ok(current_score) = T::TargetList::get_score(who) {
						let balance = current_score.saturating_sub(imbalance);

						// the target is removed from the list IFF score is 0.
						if balance.is_zero() {
							let _ = T::TargetList::on_remove(who).defensive_proof(
								"staker exists in the list as per the check above; qed.",
							);
						} else {
							// update the target score without removing it.
							let _ = T::TargetList::on_update(who, balance).defensive_proof(
								"staker exists in the list as per the check above; qed.",
							);
						}
					} else {
						defensive!("unexpected: unable to fetch score from staking interface of an existent staker");
					}
				},
				StakeImbalance::Zero => (), // update not needed.
			};
		}

		// ------ Helpers

		/// Helper to convert the balance of a staker into its vote weight.
		pub(crate) fn to_vote(balance: BalanceOf<T>) -> VoteWeight {
			<T::Staking as StakingInterface>::CurrencyToVote::to_vote(
				balance,
				T::Currency::total_issuance(),
			)
		}

		/// Helper to fetch te active stake of a staker and convert it to vote weight.
		pub fn vote_of(who: &T::AccountId) -> VoteWeight {
			let active = T::Staking::stake(who).map(|s| s.active).defensive_unwrap_or_default();
			Self::to_vote(active)
		}

		/// Returns whether a nomination vec has duplicate targets.
		///
		/// Used for debug assertions only, since this pallet expects the nominations to be
		/// deduplicated at all places.
		pub fn has_duplicate_nominations(mut v: Vec<T::AccountId>) -> bool {
			use alloc::collections::btree_set::BTreeSet;
			let size_before = v.len();
			let dedup = v.drain(..).collect::<BTreeSet<_>>().into_iter().collect::<Vec<_>>();

			size_before != dedup.len()
		}

		#[cfg(feature = "runtime-benchmarks")]
		pub(crate) fn setup_target_with_unsettled_score(target: &T::AccountId) -> DispatchResult {
			// fund target account.
			let mut balance = T::Currency::minimum_balance();
			for _ in 0..100 {
				balance = balance + T::Currency::minimum_balance();
			}
			let _ = T::Currency::set_balance(target, balance + T::Currency::minimum_balance());

			<T::Staking as StakingInterface>::bond(target, balance, target)?;
			<T::Staking as StakingInterface>::validate(target)?;

			UnsettledTargetScore::<T>::insert(target, StakeImbalance::Positive(10));

			Ok(())
		}
	}
}

/// Returns a vec with all the unsettled scores.
pub struct UnsettledTargetScores<T: Config>(PhantomData<T>);
impl<T: Config> sp_runtime::traits::TypedGet for UnsettledTargetScores<T> {
	type Type = UnsettledList<T>;

	fn get() -> Self::Type {
		UnsettledTargetScore::<T>::iter().collect::<Vec<_>>()
	}
}

impl<T: Config> OnStakingUpdate<T::AccountId, BalanceOf<T>> for Pallet<T> {
	/// When a nominator's stake is updated, all the nominated targets must be updated
	/// accordingly.
	///
	/// The score of the node associated with `who` in the *VoterList* will be updated if the
	/// the mode is [`VoterUpdateMode::Strict`]. The approvals of the nominated targets (by `who`)
	/// are always updated.
	fn on_stake_update(
		who: &T::AccountId,
		prev_stake: Option<Stake<BalanceOf<T>>>,
		stake: Stake<BalanceOf<T>>,
	) {
		match T::Staking::status(who) {
			Ok(StakerStatus::Nominator(nominations)) =>
				Self::do_stake_update_voter(who, prev_stake, stake, nominations),
			Ok(StakerStatus::Validator) => Self::do_stake_update_target(who, prev_stake, stake),
			Ok(StakerStatus::Idle) => (), // nothing to see here.
			Err(_) => {
				defensive!(
					"staker should exist when calling `on_stake_update` and have a valid status"
				);
			},
		}
	}

	/// Triggered when a new validator is added to the system.
	///
	/// Overview: If `who` is part of the target list, update its score. Otherwise, insert a new
	/// node to the target list with self-stake as initial node score.
	///
	/// A validator is also considered a voter with self-vote and should be added to
	/// [`Config::VoterList`].
	fn on_validator_add(who: &T::AccountId, self_stake: Stake<BalanceOf<T>>) {
		let self_stake = Self::to_vote(self_stake.active).into();

		match T::TargetList::on_insert(who.clone(), self_stake) {
			Ok(_) => (),
			Err(_) => {
				// if the target already exists in the list, it means that the target is idle
				// and/or is dangling and now it's becoming active again.
				defensive_assert!(
					T::Staking::status(who) == Ok(StakerStatus::Idle) ||
						T::Staking::status(who).is_err()
				);

				Self::update_target_score(who, StakeImbalance::Positive(self_stake));
			},
		}

		// a validator is also a nominator.
		Self::on_nominator_add(who, vec![who.clone()])
	}

	/// Triggered when a validator is chilled.
	///
	/// Overview: When chilled, the target node may not be removed from the target list. The
	/// associated target list score is updated so that the self-stake is decreased from itself.
	///
	/// This method will not *try* to remove the target from the target list. That is the
	/// responsability of [`OnStakingUpdate::on_validator_remove`].
	fn on_validator_idle(who: &T::AccountId) {
		// validator is a validator with itself as a nomination.
		Self::on_nominator_idle(who, vec![who.clone()]);
	}

	/// Triggered when a validator is set as inactive/removed by the staking system.
	///
	/// Overview: Updates the target list score so that `who`'s self-vote is decreased from itself.
	/// The target node is removed from the target list IFF its score is 0 after update. Otherwise,
	/// it means that there are "dangling" nominations to `who`, ie. there are nominators who are
	/// nominating `who`. even though it is chilled/removed.
	///
	/// Note: `who` *MUST* be either an active validator or chilled staker.
	fn on_validator_remove(who: &T::AccountId) {
		debug_assert!(
			T::Staking::status(who) == Ok(StakerStatus::Validator) ||
				T::Staking::status(who) == Ok(StakerStatus::Idle)
		);

		if let Ok(score) = T::TargetList::get_score(who) {
			// remove from target list IIF score is zero. If `score != 0`, the target still has
			// active nominations, thus we keep it in the target list with corresponding approval
			// stake.
			if score.is_zero() {
				let _ = T::TargetList::on_remove(who)
					.defensive_proof("target exists as per above; qed");

				// ensure that the unsettled score list is cleaned.
				let _ = UnsettledTargetScore::<T>::take(who);
			}
		} else {
			// target is not part of the list. Given the contract with staking and the checks above,
			// this may actually be called. Do nothing and skip defensive warns.
		};
	}

	/// Triggered when a new nominator is added to the system.
	///
	/// Overview: Inserts a new node in the voter list with the score being `who`'s bonded stake.
	/// The new node is inserted regardless of the [`crate::VoterUpdateMode`] set.
	///
	/// Note: this method is also used locally when adding a new target (target is also a voter).
	fn on_nominator_add(who: &T::AccountId, nominations: Vec<AccountIdOf<T>>) {
		defensive_assert!(!Self::has_duplicate_nominations(nominations.clone()));

		let nominator_vote = Self::vote_of(who);

		// the new voter node will be added even if the voter is in lazy mode. In lazy mode, we
		// ensure that the nodes exist in the voter list, even though they may not have the updated
		// score at all times.
		let _ = T::VoterList::on_insert(who.clone(), nominator_vote).defensive_proof(
			"the nominator must not exist in the list as per the contract with staking.",
		);

		// if `who` is a nominator, update the vote weight of the nominations if they exist. Note:
		// this will update the score of up to `T::MaxNominations` validators. Note that `who` may
		// be a validator.
		match T::Staking::status(who).defensive() {
			Ok(StakerStatus::Nominator(_)) =>
				for t in nominations {
					Self::update_target_score(&t, StakeImbalance::Positive(nominator_vote.into()))
				},
			Ok(StakerStatus::Idle) | Ok(StakerStatus::Validator) | Err(_) => (), // nada.
		};
	}

	/// Triggered when a nominator is chilled.
	///
	/// Overview: From this pallet POV, chilling a nominator is the same as removing it, since each
	/// nominator only has self-stake as the voter list's node score.
	fn on_nominator_idle(who: &T::AccountId, nominations: Vec<T::AccountId>) {
		Self::on_nominator_remove(who, nominations);
	}

	/// Triggered when a nominator is removed (or chilled).
	///
	/// Overview: for each of `who`'s nomination targets, decrease `who`'s self-stake from their
	/// score. In addition, remove `who`'s node from the voter list.
	///
	/// Note: the number of nodes that are updated is bounded by the maximum number of
	/// nominators, which is defined in the staking pallet.
	fn on_nominator_remove(who: &T::AccountId, nominations: Vec<T::AccountId>) {
		defensive_assert!(!Self::has_duplicate_nominations(nominations.clone()));

		let nominator_vote = Self::vote_of(who);

		// updates the nominated target's score.
		for t in nominations.iter() {
			Self::update_target_score(t, StakeImbalance::Negative(nominator_vote.into()))
		}

		let _ = T::VoterList::on_remove(who).defensive_proof(
			"the nominator must exist in the list as per the contract with staking.",
		);
	}

	/// Triggered when a nominator updates their nominations.
	///
	/// Overview: The approval scores of the the targets affected by the nomination updates must be
	/// updated accordingly.
	///
	/// Note that the nominator's stake remains the same (updates to the nominator's stake should
	/// emit [`Self::on_stake_update`] instead).
	fn on_nominator_update(
		who: &T::AccountId,
		prev_nominations: Vec<T::AccountId>,
		nominations: Vec<T::AccountId>,
	) {
		defensive_assert!(!Self::has_duplicate_nominations(nominations.clone()));

		let nominator_vote = Self::vote_of(who);

		// new nominations.
		for target in nominations.iter() {
			if !prev_nominations.contains(target) {
				Self::update_target_score(target, StakeImbalance::Positive(nominator_vote.into()));
			}
		}
		// removed nominations.
		for target in prev_nominations.iter() {
			if !nominations.contains(target) {
				Self::update_target_score(target, StakeImbalance::Negative(nominator_vote.into()));
			}
		}
	}

	// no-op events.

	/// Triggered when a staker (nominator/validator) is slashed.
	///
	/// From the stake-tracker POV, no direct updates should be made to the target or voter list in
	/// this event handler, since the stake updates from a slash will be indirectly performed
	/// through the call to `on_stake_update` resulting from the slash performed at a higher level
	/// (i.e. by staking).
	fn on_slash(
		_stash: &T::AccountId,
		_slashed_active: BalanceOf<T>,
		_slashed_unlocking: &BTreeMap<sp_staking::EraIndex, BalanceOf<T>>,
		_slashed_total: BalanceOf<T>,
	) {
	}

	/// The score of the staker `who` is updated through the `on_stake_update` calls following the
	/// full unstake (ledger kill).
	fn on_unstake(_who: &T::AccountId) {}

	/// The score of the staker `who` is updated through the `on_stake_update` calls following the
	/// withdraw.
	fn on_withdraw(_who: &T::AccountId, _amount: BalanceOf<T>) {}
}
