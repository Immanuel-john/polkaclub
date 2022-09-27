//! Benchmarking setup for pallet-template

use super::*;

#[allow(unused)]
use crate::Pallet as PalletClub;
use frame_benchmarking::{benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

benchmarks! {
	add_club_member {
		let s in 0 .. ((100 as u64)).try_into().unwrap();
		let club_name = s.to_be_bytes().to_vec();
		let caller: T::AccountId = whitelisted_caller();
	}: _(RawOrigin::Root, club_name.clone(), caller.clone())
	verify {
        assert_event::<T>(Event::ClubMemberAdded(club_name, caller).into());  
	}

	remove_club_member {
		let s = 100u64;
		let club_name = s.to_be_bytes().to_vec();
		let caller: T::AccountId = whitelisted_caller();
		<Club<T>>::try_mutate(&club_name, |account_ids| account_ids
			.try_push(caller.clone()))
			.map_err(|_| <Error<T>>::MaxMembersReached);
		
	
	}: _(RawOrigin::Root, club_name.clone(), caller.clone())
	verify {
        assert_event::<T>(Event::ClubMemberRemoved(club_name, caller).into());  
	}

	impl_benchmark_test_suite!(Club, crate::mock::new_test_ext(), crate::mock::Test);
}

fn assert_event<T: Config>(generic_event: <T as Config>::Event) {
	frame_system::Pallet::<T>::assert_has_event(generic_event.into());
}