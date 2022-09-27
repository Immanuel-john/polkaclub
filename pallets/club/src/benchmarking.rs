//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as PalletClub;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;
use rand::Rng;

benchmarks! {
	add_club_member {
		let club_name = rand::thread_rng().gen::<[u8; 32]>();
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, club_name.into(), caller.clone())
	verify {
        assert_event::<T>(Event::ClubMemberAdded(club_name.into(), caller).into());  
	}

	// remove_club_member {
	// 	let s = 100u64;
	// 	let caller: T::AccountId = whitelisted_caller();
	// 	<Club<T>>::try_mutate(&s, |account_ids| account_ids
	// 		.try_push(caller.clone()))
	// 		.map_err(|_| <Error<T>>::MaxMembersReached);
		
	
	// }: _(RawOrigin::Root, s.into(), caller.clone())
	// verify {
    //     assert_event::<T>(Event::ClubMemberRemoved(s.into(), caller).into());  
	// }

	impl_benchmark_test_suite!(Club, crate::mock::new_test_ext(), crate::mock::Test);
}

fn assert_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}