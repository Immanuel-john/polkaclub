#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;


#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use scale_info::prelude::vec::Vec;
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		#[pallet::constant]
		type MaxMembers: Get<u32>;

		// type WeightInfo: WeightInfo;
	}

	#[pallet::storage]
	#[pallet::getter(fn club)]
	pub(super) type Club<T: Config> = StorageMap<_, Twox64Concat, Vec<u8>, BoundedVec<T::AccountId, T::MaxMembers>, ValueQuery>;


	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/main-docs/build/events-errors/
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		///[Clubname, Member]
		ClubMemberAdded(Vec<u8>, T::AccountId),
		///[Clubname, Member]
		ClubMemberRemoved(Vec<u8>, T::AccountId),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		///Maximum members reached for the club
		MaxMembersReached,
		///Member not found in a club for removal
		MemberNotFound,
		///Member already exists for adding
		MemberAlreadyExist,
		///Club not found for the operation
		ClubNotFound,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn add_club_member(origin: OriginFor<T>, club_name: Vec<u8>,  club_member: T::AccountId) -> DispatchResult {

			ensure_root(origin)?;
			
			<Club<T>>::try_mutate(&club_name, |account_ids| account_ids.try_push(club_member.clone()))
			.map_err(|_| <Error<T>>::MaxMembersReached)?;

			Self::deposit_event(Event::ClubMemberAdded(club_name, club_member));
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1))]
		pub fn remove_club_member(origin: OriginFor<T>, club_name: Vec<u8>, club_member: T::AccountId) -> DispatchResult {

			ensure_root(origin)?;
			ensure!(<Club<T>>::contains_key(&club_name), Error::<T>::ClubNotFound);

			<Club<T>>::try_mutate(&club_name, |club_members| {
				if let Some(index) = club_members.iter().position(|id| id == &club_member) {
					club_members.remove(index);
					return Ok(())
				} 
				Err(())
			})
			.map_err(|_| <Error<T>>::MemberNotFound)?;

			Self::deposit_event(Event::ClubMemberRemoved(club_name, club_member));

			Ok(())
		}
	}
}
