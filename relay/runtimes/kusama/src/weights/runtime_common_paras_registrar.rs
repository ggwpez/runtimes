// Copyright 2017-2022 Parity Technologies (UK) Ltd.
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
//! Autogenerated weights for `runtime_common::paras_registrar`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-28, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `bm5`, CPU: `Intel(R) Core(TM) i7-7700K CPU @ 4.20GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("kusama-dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot
// benchmark
// pallet
// --chain=kusama-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::paras_registrar
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/runtime_common_paras_registrar.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::paras_registrar`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::paras_registrar::WeightInfo for WeightInfo<T> {
	/// Storage: Registrar NextFreeParaId (r:1 w:1)
	/// Proof Skipped: Registrar NextFreeParaId (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:0)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	fn reserve() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `70`
		//  Estimated: `5655`
		// Minimum execution time: 24_974 nanoseconds.
		Weight::from_ref_time(25_915_000)
			.saturating_add(Weight::from_proof_size(5655))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:0 w:1)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	/// Proof Skipped: Paras UpcomingParasGenesis (max_values: None, max_size: None, mode: Measured)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `302`
		//  Estimated: `18063`
		// Minimum execution time: 7_300_136 nanoseconds.
		Weight::from_ref_time(7_429_498_000)
			.saturating_add(Weight::from_proof_size(18063))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:0 w:1)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingParasGenesis (r:0 w:1)
	/// Proof Skipped: Paras UpcomingParasGenesis (max_values: None, max_size: None, mode: Measured)
	fn force_register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `160`
		//  Estimated: `16785`
		// Minimum execution time: 7_300_715 nanoseconds.
		Weight::from_ref_time(7_412_926_000)
			.saturating_add(Weight::from_proof_size(16785))
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Registrar Paras (r:1 w:1)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:1 w:1)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeHash (r:1 w:0)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar PendingSwap (r:0 w:1)
	/// Proof Skipped: Registrar PendingSwap (max_values: None, max_size: None, mode: Measured)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467`
		//  Estimated: `13197`
		// Minimum execution time: 39_974 nanoseconds.
		Weight::from_ref_time(40_522_000)
			.saturating_add(Weight::from_proof_size(13197))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Registrar Paras (r:1 w:0)
	/// Proof Skipped: Registrar Paras (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras ParaLifecycles (r:2 w:2)
	/// Proof Skipped: Paras ParaLifecycles (max_values: None, max_size: None, mode: Measured)
	/// Storage: Registrar PendingSwap (r:1 w:1)
	/// Proof Skipped: Registrar PendingSwap (max_values: None, max_size: None, mode: Measured)
	/// Storage: ParasShared CurrentSessionIndex (r:1 w:0)
	/// Proof Skipped: ParasShared CurrentSessionIndex (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras ActionsQueue (r:1 w:1)
	/// Proof Skipped: Paras ActionsQueue (max_values: None, max_size: None, mode: Measured)
	/// Storage: Crowdloan Funds (r:2 w:2)
	/// Proof Skipped: Crowdloan Funds (max_values: None, max_size: None, mode: Measured)
	/// Storage: Slots Leases (r:2 w:2)
	/// Proof Skipped: Slots Leases (max_values: None, max_size: None, mode: Measured)
	fn swap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `707`
		//  Estimated: `27719`
		// Minimum execution time: 47_771 nanoseconds.
		Weight::from_ref_time(48_623_000)
			.saturating_add(Weight::from_proof_size(27719))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Paras FutureCodeHash (r:1 w:1)
	/// Proof Skipped: Paras FutureCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeRestrictionSignal (r:1 w:1)
	/// Proof Skipped: Paras UpgradeRestrictionSignal (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CurrentCodeHash (r:1 w:0)
	/// Proof Skipped: Paras CurrentCodeHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpgradeCooldowns (r:1 w:1)
	/// Proof Skipped: Paras UpgradeCooldowns (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras PvfActiveVoteMap (r:1 w:0)
	/// Proof Skipped: Paras PvfActiveVoteMap (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras CodeByHash (r:1 w:1)
	/// Proof Skipped: Paras CodeByHash (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras UpcomingUpgrades (r:1 w:1)
	/// Proof Skipped: Paras UpcomingUpgrades (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Digest (r:1 w:1)
	/// Proof Skipped: System Digest (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Paras CodeByHashRefs (r:1 w:1)
	/// Proof Skipped: Paras CodeByHashRefs (max_values: None, max_size: None, mode: Measured)
	/// Storage: Paras FutureCodeUpgrades (r:0 w:1)
	/// Proof Skipped: Paras FutureCodeUpgrades (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 3145728]`.
	fn schedule_code_upgrade(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `28`
		//  Estimated: `16615`
		// Minimum execution time: 38_081 nanoseconds.
		Weight::from_ref_time(38_483_000)
			.saturating_add(Weight::from_proof_size(16615))
			// Standard Error: 1
			.saturating_add(Weight::from_ref_time(2_291).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(8))
	}
	/// Storage: Paras Heads (r:0 w:1)
	/// Proof Skipped: Paras Heads (max_values: None, max_size: None, mode: Measured)
	/// The range of component `b` is `[1, 1048576]`.
	fn set_current_head(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 8_520 nanoseconds.
		Weight::from_ref_time(8_722_000)
			.saturating_add(Weight::from_proof_size(0))
			// Standard Error: 2
			.saturating_add(Weight::from_ref_time(872).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
