#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;
use scale_info::{prelude::vec, TypeInfo};

// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

use frame_support::{
	codec::{Decode, Encode},
	inherent::Vec,
	sp_runtime::RuntimeDebug,
};

#[derive(PartialEq, Eq, Encode, Decode, RuntimeDebug, TypeInfo)]
pub struct Votes<AccountId> {
	ayes: Vec<AccountId>,
	nays: Vec<AccountId>,
}

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug, TypeInfo)]
pub enum Vote {
	Aye,
	Nay,
}

#[frame_support::pallet]
pub mod pallet {
	use crate::Votes;
	use frame_support::{inherent::Vec, pallet_prelude::*};
	use frame_system::pallet_prelude::*;
	use scale_info::{prelude::vec, TypeInfo};

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	#[pallet::without_storage_info]
	pub struct Pallet<T>(_);

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
	}

	#[pallet::storage]
	#[pallet::getter(fn community_members)]
	pub type CommunityMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn committee_members)]
	pub type CommitteeMembers<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

	#[pallet::storage]
	#[pallet::getter(fn voting)]
	pub type Voting<T: Config> = StorageMap<_, Identity, T::Hash, Votes<T::AccountId>, OptionQuery>;

	// Pallets use events to inform users when important changes are made.
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		MemberAdded,
		MemberAddedToCommittee,
		ProposalReject,
		ProposalApproved,
		ProposalAdded,
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// If a member try to add in a community for multiple times.
		AlreadyMemberOfCommunity,
		/// If sudo try to add a community member in a committee multiple times.
		AlreadyMemberOfCommittee,
		/// If sudo try to add a member which is not a part of community.
		MemberIsNotPresentInCommunity,
		ProposalAlreadyExist,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		#[pallet::weight(10_000)]
		pub fn add_community_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_signed(origin.clone())?;

			let mut members = CommunityMembers::<T>::get();
			let location =
				members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMemberOfCommunity)?;

			members.insert(location, who.clone());

			CommunityMembers::<T>::put(&members);

			Self::deposit_event(Event::MemberAdded);
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_committee_member(origin: OriginFor<T>, who: T::AccountId) -> DispatchResult {
			ensure_root(origin.clone())?;

			// member should be present in community members list
			let community_member = CommunityMembers::<T>::get();
			let _is_present = community_member
				.binary_search(&who)
				.map_err(|_| Error::<T>::MemberIsNotPresentInCommunity)?;

			let mut members = CommitteeMembers::<T>::get();
			let location =
				members.binary_search(&who).err().ok_or(Error::<T>::AlreadyMemberOfCommittee)?;

			members.insert(location, who.clone());

			CommitteeMembers::<T>::put(&members);

			Self::deposit_event(Event::MemberAddedToCommittee);
			Ok(())
		}

		#[pallet::weight(10_000)]
		pub fn add_proposal(
			origin: OriginFor<T>,
			who: T::AccountId,
			proposal: T::Hash,
		) -> DispatchResult {
			ensure_signed(origin.clone())?;

			// member should be present in community members list
			let community_member = CommunityMembers::<T>::get();
			let _is_present = community_member
				.binary_search(&who)
				.map_err(|_| Error::<T>::MemberIsNotPresentInCommunity)?;

			ensure!(!Voting::<T>::contains_key(&proposal), Error::<T>::ProposalAlreadyExist);

			// Add Proposal
			let votes = { Votes { ayes: vec![], nays: vec![] } };
			<Voting<T>>::insert(proposal, votes);
			Self::deposit_event(Event::ProposalAdded);
			Ok(())
		}
	}
}
