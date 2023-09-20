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

//! Autogenerated weights for `pallet_sassafras`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-20, STEPS: `10`, REPEAT: `3`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `behemoth`, CPU: `AMD Ryzen Threadripper 3970X 32-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/node-sassafras
// benchmark
// pallet
// --chain
// dev
// --pallet
// pallet_sassafras
// --extrinsic
// *
// --steps
// 10
// --repeat
// 3
// --output
// weights.rs
// --template
// substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_sassafras`.
pub trait WeightInfo {
	fn submit_tickets(x: u32, ) -> Weight;
	fn plan_config_change() -> Weight;
	fn recompute_ring_verifier(x: u32, ) -> Weight;
	fn sort_segments(x: u32, y: u32, ) -> Weight;
}

/// Weights for `pallet_sassafras` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Sassafras::RingVerifierData` (r:1 w:0)
	/// Proof: `Sassafras::RingVerifierData` (`max_values`: Some(1), `max_size`: Some(422), added: 917, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextAuthorities` (r:1 w:0)
	/// Proof: `Sassafras::NextAuthorities` (`max_values`: Some(1), `max_size`: Some(1057), added: 1552, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextEpochConfig` (r:1 w:0)
	/// Proof: `Sassafras::NextEpochConfig` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextRandomness` (r:1 w:0)
	/// Proof: `Sassafras::NextRandomness` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::EpochIndex` (r:1 w:0)
	/// Proof: `Sassafras::EpochIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsMeta` (r:1 w:1)
	/// Proof: `Sassafras::TicketsMeta` (`max_values`: Some(1), `max_size`: Some(12), added: 507, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextTicketsSegments` (r:0 w:1)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(165), added: 2640, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsData` (r:0 w:3)
	/// Proof: `Sassafras::TicketsData` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 3]`.
	fn submit_tickets(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1262`
		//  Estimated: `2542`
		// Minimum execution time: 19_145_978_000 picoseconds.
		Weight::from_parts(18_557_150_122, 2542)
			// Standard Error: 663_552_591
			.saturating_add(Weight::from_parts(35_168_774_866, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((2_u64).saturating_mul(x.into())))
	}
	/// Storage: `Sassafras::PendingEpochConfigChange` (r:0 w:1)
	/// Proof: `Sassafras::PendingEpochConfigChange` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn plan_config_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_150_000 picoseconds.
		Weight::from_parts(5_791_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Sassafras::RingContext` (r:1 w:0)
	/// Proof: `Sassafras::RingContext` (`max_values`: Some(1), `max_size`: Some(147748), added: 148243, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn recompute_ring_verifier(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147876`
		//  Estimated: `149233`
		// Minimum execution time: 2_381_346_273_000 picoseconds.
		Weight::from_parts(2_431_419_770_352, 149233)
			// Standard Error: 1_109_324_219
			.saturating_add(Weight::from_parts(153_175_998, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: `Sassafras::NextTicketsSegments` (r:3 w:1)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(165), added: 2640, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsIds` (r:0 w:10)
	/// Proof: `Sassafras::TicketsIds` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 1800]`.
	/// The range of component `y` is `[1, 2]`.
	fn sort_segments(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `2047 + x * (1 ±0) + y * (2984 ±233)`
		// Minimum execution time: 11_642_000 picoseconds.
		Weight::from_parts(16_037_829, 2047)
			// Standard Error: 800_270
			.saturating_add(Weight::from_parts(470_377, 0).saturating_mul(y.into()))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(T::DbWeight::get().writes(7_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(0, 2984).saturating_mul(y.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Sassafras::RingVerifierData` (r:1 w:0)
	/// Proof: `Sassafras::RingVerifierData` (`max_values`: Some(1), `max_size`: Some(422), added: 917, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextAuthorities` (r:1 w:0)
	/// Proof: `Sassafras::NextAuthorities` (`max_values`: Some(1), `max_size`: Some(1057), added: 1552, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextEpochConfig` (r:1 w:0)
	/// Proof: `Sassafras::NextEpochConfig` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextRandomness` (r:1 w:0)
	/// Proof: `Sassafras::NextRandomness` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::EpochIndex` (r:1 w:0)
	/// Proof: `Sassafras::EpochIndex` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsMeta` (r:1 w:1)
	/// Proof: `Sassafras::TicketsMeta` (`max_values`: Some(1), `max_size`: Some(12), added: 507, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::NextTicketsSegments` (r:0 w:1)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(165), added: 2640, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsData` (r:0 w:3)
	/// Proof: `Sassafras::TicketsData` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[0, 3]`.
	fn submit_tickets(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1262`
		//  Estimated: `2542`
		// Minimum execution time: 19_145_978_000 picoseconds.
		Weight::from_parts(18_557_150_122, 2542)
			// Standard Error: 663_552_591
			.saturating_add(Weight::from_parts(35_168_774_866, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((2_u64).saturating_mul(x.into())))
	}
	/// Storage: `Sassafras::PendingEpochConfigChange` (r:0 w:1)
	/// Proof: `Sassafras::PendingEpochConfigChange` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	fn plan_config_change() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_150_000 picoseconds.
		Weight::from_parts(5_791_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Sassafras::RingContext` (r:1 w:0)
	/// Proof: `Sassafras::RingContext` (`max_values`: Some(1), `max_size`: Some(147748), added: 148243, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 20]`.
	fn recompute_ring_verifier(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `147876`
		//  Estimated: `149233`
		// Minimum execution time: 2_381_346_273_000 picoseconds.
		Weight::from_parts(2_431_419_770_352, 149233)
			// Standard Error: 1_109_324_219
			.saturating_add(Weight::from_parts(153_175_998, 0).saturating_mul(x.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: `Sassafras::NextTicketsSegments` (r:3 w:1)
	/// Proof: `Sassafras::NextTicketsSegments` (`max_values`: None, `max_size`: Some(165), added: 2640, mode: `MaxEncodedLen`)
	/// Storage: `Sassafras::TicketsIds` (r:0 w:10)
	/// Proof: `Sassafras::TicketsIds` (`max_values`: None, `max_size`: Some(21), added: 2496, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 1800]`.
	/// The range of component `y` is `[1, 2]`.
	fn sort_segments(x: u32, y: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `452`
		//  Estimated: `2047 + x * (1 ±0) + y * (2984 ±233)`
		// Minimum execution time: 11_642_000 picoseconds.
		Weight::from_parts(16_037_829, 2047)
			// Standard Error: 800_270
			.saturating_add(Weight::from_parts(470_377, 0).saturating_mul(y.into()))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(y.into())))
			.saturating_add(RocksDbWeight::get().writes(7_u64))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(x.into()))
			.saturating_add(Weight::from_parts(0, 2984).saturating_mul(y.into()))
	}
}
