use crate::{mock::*, Error, Proposal, Votes};
use frame_support::{assert_noop, assert_ok};
use frame_system::Call;
use sp_runtime::traits::{BlakeTwo256, Hash};

pub type HashType = <Test as frame_system::Config>::Hash;
pub type Hashing = <Test as frame_system::Config>::Hashing;

#[test]
fn add_community_member_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));

		assert_eq!(ProposalPallet::community_members(), vec![TEST_ACCOUNT]);
	});
}

#[test]
fn add_committee_member_passed() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));

		assert_ok!(ProposalPallet::add_committee_member(RuntimeOrigin::root(), TEST_ACCOUNT));

		assert_eq!(ProposalPallet::committee_members(), vec![TEST_ACCOUNT]);
	});
}

#[test]
fn add_committee_member_from_extrinsic_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));

		assert_noop!(
			ProposalPallet::add_committee_member(RuntimeOrigin::signed(TEST_ACCOUNT), 1),
			sp_runtime::DispatchError::BadOrigin
		);
	});
}

#[test]
fn add_proposal_successfully() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Create a dispute
		// let proposal = make_proposal(42);
		let hash = HashType::from(Hashing::hash_of(&42));
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));
		let title = Vec::new();
		assert_ok!(ProposalPallet::add_proposal(RuntimeOrigin::signed(1), title, hash, 1000));

		assert_eq!(ProposalPallet::voting(hash), Some(Votes { ayes: vec![], nays: vec![] }));
	});
}
