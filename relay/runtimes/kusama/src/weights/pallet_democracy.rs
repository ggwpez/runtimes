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
//! Autogenerated weights for `pallet_democracy`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-02-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
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
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --header=./file_header.txt
// --output=./runtime/kusama/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_democracy`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_democracy::WeightInfo for WeightInfo<T> {
	/// Storage: Democracy PublicPropCount (r:1 w:1)
	/// Proof: Democracy PublicPropCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:0 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4831`
		//  Estimated: `23409`
		// Minimum execution time: 34_876 nanoseconds.
		Weight::from_ref_time(35_375_000)
			.saturating_add(Weight::from_proof_size(23409))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	fn second() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3587`
		//  Estimated: `5705`
		// Minimum execution time: 31_982 nanoseconds.
		Weight::from_ref_time(32_412_000)
			.saturating_add(Weight::from_proof_size(5705))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn vote_new() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3532`
		//  Estimated: `12720`
		// Minimum execution time: 46_525 nanoseconds.
		Weight::from_ref_time(46_812_000)
			.saturating_add(Weight::from_proof_size(12720))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	fn vote_existing() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3554`
		//  Estimated: `12720`
		// Minimum execution time: 46_269 nanoseconds.
		Weight::from_ref_time(46_629_000)
			.saturating_add(Weight::from_proof_size(12720))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Cancellations (r:1 w:1)
	/// Proof: Democracy Cancellations (max_values: None, max_size: Some(33), added: 2508, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn emergency_cancel() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `365`
		//  Estimated: `7712`
		// Minimum execution time: 25_015 nanoseconds.
		Weight::from_ref_time(25_351_000)
			.saturating_add(Weight::from_proof_size(7712))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:3 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:0 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	fn blacklist() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6003`
		//  Estimated: `36392`
		// Minimum execution time: 90_150 nanoseconds.
		Weight::from_ref_time(91_058_000)
			.saturating_add(Weight::from_proof_size(36392))
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:0)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	fn external_propose() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3415`
		//  Estimated: `6340`
		// Minimum execution time: 12_616 nanoseconds.
		Weight::from_ref_time(12_812_000)
			.saturating_add(Weight::from_proof_size(6340))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_majority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_172 nanoseconds.
		Weight::from_ref_time(3_295_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:0 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	fn external_propose_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_117 nanoseconds.
		Weight::from_ref_time(3_288_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:1)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:2)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn fast_track() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `3654`
		// Minimum execution time: 26_675 nanoseconds.
		Weight::from_ref_time(27_369_000)
			.saturating_add(Weight::from_proof_size(3654))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	/// Storage: Democracy NextExternal (r:1 w:1)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy Blacklist (r:1 w:1)
	/// Proof: Democracy Blacklist (max_values: None, max_size: Some(3238), added: 5713, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn veto_external() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `3518`
		//  Estimated: `8868`
		// Minimum execution time: 31_174 nanoseconds.
		Weight::from_ref_time(31_517_000)
			.saturating_add(Weight::from_proof_size(8868))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy PublicProps (r:1 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy DepositOf (r:1 w:1)
	/// Proof: Democracy DepositOf (max_values: None, max_size: Some(3230), added: 5705, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn cancel_proposal() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `5882`
		//  Estimated: `28033`
		// Minimum execution time: 73_218 nanoseconds.
		Weight::from_ref_time(74_394_000)
			.saturating_add(Weight::from_proof_size(28033))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:0 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	fn cancel_referendum() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `238`
		//  Estimated: `2528`
		// Minimum execution time: 18_468 nanoseconds.
		Weight::from_ref_time(18_870_000)
			.saturating_add(Weight::from_proof_size(2528))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + r * (117 ±0)`
		//  Estimated: `998 + r * (2676 ±0)`
		// Minimum execution time: 6_237 nanoseconds.
		Weight::from_ref_time(9_788_030)
			.saturating_add(Weight::from_proof_size(998))
			// Standard Error: 5_493
			.saturating_add(Weight::from_ref_time(2_774_064).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy LowestUnbaked (r:1 w:1)
	/// Proof: Democracy LowestUnbaked (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumCount (r:1 w:0)
	/// Proof: Democracy ReferendumCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Democracy LastTabledWasExternal (r:1 w:0)
	/// Proof: Democracy LastTabledWasExternal (max_values: Some(1), max_size: Some(1), added: 496, mode: MaxEncodedLen)
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn on_initialize_base_with_launch_period(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `211 + r * (117 ±0)`
		//  Estimated: `19318 + r * (2676 ±0)`
		// Minimum execution time: 9_370 nanoseconds.
		Weight::from_ref_time(12_551_319)
			.saturating_add(Weight::from_proof_size(19318))
			// Standard Error: 5_735
			.saturating_add(Weight::from_ref_time(2_771_452).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:3 w:3)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn delegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `925 + r * (139 ±0)`
		//  Estimated: `22584 + r * (2676 ±0)`
		// Minimum execution time: 39_375 nanoseconds.
		Weight::from_ref_time(42_351_696)
			.saturating_add(Weight::from_proof_size(22584))
			// Standard Error: 7_527
			.saturating_add(Weight::from_ref_time(3_880_547).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy VotingOf (r:2 w:2)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Democracy ReferendumInfoOf (r:99 w:99)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn undelegate(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `524 + r * (139 ±0)`
		//  Estimated: `12540 + r * (2676 ±0)`
		// Minimum execution time: 20_672 nanoseconds.
		Weight::from_ref_time(22_405_117)
			.saturating_add(Weight::from_proof_size(12540))
			// Standard Error: 7_623
			.saturating_add(Weight::from_ref_time(3_830_380).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_proof_size(2676).saturating_mul(r.into()))
	}
	/// Storage: Democracy PublicProps (r:0 w:1)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	fn clear_public_proposals() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_083 nanoseconds.
		Weight::from_ref_time(3_242_000)
			.saturating_add(Weight::from_proof_size(0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_remove(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `594`
		//  Estimated: `12647`
		// Minimum execution time: 20_374 nanoseconds.
		Weight::from_ref_time(25_944_613)
			.saturating_add(Weight::from_proof_size(12647))
			// Standard Error: 1_258
			.saturating_add(Weight::from_ref_time(19_696).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[0, 99]`.
	fn unlock_set(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `595 + r * (22 ±0)`
		//  Estimated: `12647`
		// Minimum execution time: 24_535 nanoseconds.
		Weight::from_ref_time(25_495_740)
			.saturating_add(Weight::from_proof_size(12647))
			// Standard Error: 551
			.saturating_add(Weight::from_ref_time(60_441).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `758 + r * (26 ±0)`
		//  Estimated: `8946`
		// Minimum execution time: 14_979 nanoseconds.
		Weight::from_ref_time(17_266_413)
			.saturating_add(Weight::from_proof_size(8946))
			// Standard Error: 1_973
			.saturating_add(Weight::from_ref_time(87_619).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:1)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy VotingOf (r:1 w:1)
	/// Proof: Democracy VotingOf (max_values: None, max_size: Some(3795), added: 6270, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 100]`.
	fn remove_other_vote(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `758 + r * (26 ±0)`
		//  Estimated: `8946`
		// Minimum execution time: 15_169 nanoseconds.
		Weight::from_ref_time(17_559_094)
			.saturating_add(Weight::from_proof_size(8946))
			// Standard Error: 2_076
			.saturating_add(Weight::from_ref_time(88_471).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `323`
		//  Estimated: `3193`
		// Minimum execution time: 17_984 nanoseconds.
		Weight::from_ref_time(18_599_000)
			.saturating_add(Weight::from_proof_size(3193))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy NextExternal (r:1 w:0)
	/// Proof: Democracy NextExternal (max_values: Some(1), max_size: Some(132), added: 627, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_external_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `253`
		//  Estimated: `3155`
		// Minimum execution time: 16_277 nanoseconds.
		Weight::from_ref_time(16_581_000)
			.saturating_add(Weight::from_proof_size(3155))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4886`
		//  Estimated: `19763`
		// Minimum execution time: 33_325 nanoseconds.
		Weight::from_ref_time(33_908_000)
			.saturating_add(Weight::from_proof_size(19763))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy PublicProps (r:1 w:0)
	/// Proof: Democracy PublicProps (max_values: Some(1), max_size: Some(16702), added: 17197, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_proposal_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `4820`
		//  Estimated: `19725`
		// Minimum execution time: 30_899 nanoseconds.
		Weight::from_ref_time(31_309_000)
			.saturating_add(Weight::from_proof_size(19725))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Preimage StatusFor (r:1 w:0)
	/// Proof: Preimage StatusFor (max_values: None, max_size: Some(91), added: 2566, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:0 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn set_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `144`
		//  Estimated: `2566`
		// Minimum execution time: 14_007 nanoseconds.
		Weight::from_ref_time(14_342_000)
			.saturating_add(Weight::from_proof_size(2566))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Democracy ReferendumInfoOf (r:1 w:0)
	/// Proof: Democracy ReferendumInfoOf (max_values: None, max_size: Some(201), added: 2676, mode: MaxEncodedLen)
	/// Storage: Democracy MetadataOf (r:1 w:1)
	/// Proof: Democracy MetadataOf (max_values: None, max_size: Some(53), added: 2528, mode: MaxEncodedLen)
	fn clear_referendum_metadata() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `269`
		//  Estimated: `5204`
		// Minimum execution time: 17_978 nanoseconds.
		Weight::from_ref_time(18_262_000)
			.saturating_add(Weight::from_proof_size(5204))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
