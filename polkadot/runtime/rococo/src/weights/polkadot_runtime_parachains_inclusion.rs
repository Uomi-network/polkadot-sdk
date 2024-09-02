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

//! Autogenerated weights for `polkadot_runtime_parachains::inclusion`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-08-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-696hpswk-project-674-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("rococo-dev")`, DB CACHE: 1024

// Executed Command:
// target/production/polkadot
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=polkadot_runtime_parachains::inclusion
// --chain=rococo-dev
// --header=./polkadot/file_header.txt
// --output=./polkadot/runtime/rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `polkadot_runtime_parachains::inclusion`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> polkadot_runtime_parachains::inclusion::WeightInfo for WeightInfo<T> {
	/// Storage: `Paras::FutureCodeHash` (r:1 w:1)
	/// Proof: `Paras::FutureCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CurrentCodeHash` (r:1 w:0)
	/// Proof: `Paras::CurrentCodeHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeCooldowns` (r:1 w:1)
	/// Proof: `Paras::UpgradeCooldowns` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteMap` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteMap` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHash` (r:1 w:1)
	/// Proof: `Paras::CodeByHash` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParasShared::ActiveValidatorKeys` (r:1 w:0)
	/// Proof: `ParasShared::ActiveValidatorKeys` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::PvfActiveVoteList` (r:1 w:1)
	/// Proof: `Paras::PvfActiveVoteList` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::CodeByHashRefs` (r:1 w:1)
	/// Proof: `Paras::CodeByHashRefs` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DownwardMessageQueues` (r:1 w:1)
	/// Proof: `Dmp::DownwardMessageQueues` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Dmp::DeliveryFeeFactor` (r:1 w:1)
	/// Proof: `Dmp::DeliveryFeeFactor` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `MessageQueue::BookStateFor` (r:1 w:1)
	/// Proof: `MessageQueue::BookStateFor` (`max_values`: None, `max_size`: Some(55), added: 2530, mode: `MaxEncodedLen`)
	/// Storage: `MessageQueue::Pages` (r:1 w:32)
	/// Proof: `MessageQueue::Pages` (`max_values`: None, `max_size`: Some(32818), added: 35293, mode: `MaxEncodedLen`)
	/// Storage: `Hrmp::HrmpChannelDigests` (r:33 w:33)
	/// Proof: `Hrmp::HrmpChannelDigests` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannels` (r:32 w:32)
	/// Proof: `Hrmp::HrmpChannels` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Hrmp::HrmpChannelContents` (r:32 w:32)
	/// Proof: `Hrmp::HrmpChannelContents` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::FutureCodeUpgrades` (r:1 w:0)
	/// Proof: `Paras::FutureCodeUpgrades` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Registrar::Paras` (r:1 w:0)
	/// Proof: `Registrar::Paras` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: UNKNOWN KEY `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0x3a72656c61795f64697370617463685f71756575655f72656d61696e696e675f` (r:0 w:1)
	/// Storage: `Hrmp::HrmpWatermarks` (r:0 w:1)
	/// Proof: `Hrmp::HrmpWatermarks` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::Heads` (r:0 w:1)
	/// Proof: `Paras::Heads` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeGoAheadSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeGoAheadSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::MostRecentContext` (r:0 w:1)
	/// Proof: `Paras::MostRecentContext` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Paras::UpgradeRestrictionSignal` (r:0 w:1)
	/// Proof: `Paras::UpgradeRestrictionSignal` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: UNKNOWN KEY `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	/// Proof: UNKNOWN KEY `0xf5207f03cfdce586301014700e2c2593fad157e461d71fd4c1f936839a5f1f3e` (r:0 w:1)
	/// The range of component `u` is `[1, 32]`.
	/// The range of component `h` is `[1, 32]`.
	/// The range of component `c` is `[0, 1]`.
	fn enact_candidate(u: u32, h: u32, c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33353 + c * (16114 ±0) + h * (75 ±0)`
		//  Estimated: `36818 + c * (26467 ±0) + h * (2551 ±0)`
		// Minimum execution time: 4_829_551_000 picoseconds.
		Weight::from_parts(1_892_697_027, 0)
			.saturating_add(Weight::from_parts(0, 36818))
			// Standard Error: 793_993
			.saturating_add(Weight::from_parts(126_698_671, 0).saturating_mul(u.into()))
			// Standard Error: 793_993
			.saturating_add(Weight::from_parts(144_116_038, 0).saturating_mul(h.into()))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().reads((3_u64).saturating_mul(h.into())))
			.saturating_add(T::DbWeight::get().reads((8_u64).saturating_mul(c.into())))
			.saturating_add(T::DbWeight::get().writes(10))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(u.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(h.into())))
			.saturating_add(T::DbWeight::get().writes((7_u64).saturating_mul(c.into())))
			.saturating_add(Weight::from_parts(0, 26467).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 2551).saturating_mul(h.into()))
	}
}
