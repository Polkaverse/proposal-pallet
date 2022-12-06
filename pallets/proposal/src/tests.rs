use crate::{mock::*, Error, Vote, Votes};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::Hash;

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
		// Create a proposal
		let hash = HashType::from(Hashing::hash_of(&42));
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));
		let title = Vec::new();
		assert_ok!(ProposalPallet::add_proposal(RuntimeOrigin::signed(1), title, hash, 1000));

		assert_eq!(ProposalPallet::voting(hash), Some(Votes { ayes: vec![], nays: vec![] }));
	});
}

#[test]
fn add_proposal_when_not_present_in_community_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// Create a proposal
		let hash = HashType::from(Hashing::hash_of(&42));

		let title = Vec::new();

		assert_noop!(
			ProposalPallet::add_proposal(RuntimeOrigin::signed(1), title, hash, 1000),
			Error::<Test>::MemberIsNotPresentInCommunity
		);
	});
}

#[test]
fn add_same_proposal_multiple_times_fail() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// let proposal = make_proposal(42);
		let hash = HashType::from(Hashing::hash_of(&42));
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));
		let title = Vec::new();

		assert_ok!(ProposalPallet::add_proposal(
			RuntimeOrigin::signed(1),
			title.clone(),
			hash,
			1000
		));

		assert_noop!(
			ProposalPallet::add_proposal(RuntimeOrigin::signed(1), title, hash, 1000),
			Error::<Test>::ProposalAlreadyExist
		);
	});
}

#[test]
fn approve_proposal_correctly() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// let proposal = make_proposal(42);
		let hash = HashType::from(Hashing::hash_of(&42));
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));

		assert_ok!(ProposalPallet::add_committee_member(RuntimeOrigin::root(), TEST_ACCOUNT));

		let title = Vec::new();

		assert_ok!(ProposalPallet::add_proposal(
			RuntimeOrigin::signed(1),
			title.clone(),
			hash,
			1000
		));

		assert_ok!(ProposalPallet::approve_proposal(RuntimeOrigin::signed(1), hash, Vote::Aye));

		assert_eq!(ProposalPallet::approvers(hash), vec![1]);
	});
}

#[test]
fn approve_same_proposal_multiple_times_fails() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// let proposal = make_proposal(42);
		let hash = HashType::from(Hashing::hash_of(&42));
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));

		assert_ok!(ProposalPallet::add_committee_member(RuntimeOrigin::root(), TEST_ACCOUNT));

		let title = Vec::new();

		assert_ok!(ProposalPallet::add_proposal(
			RuntimeOrigin::signed(1),
			title.clone(),
			hash,
			1000
		));

		assert_ok!(ProposalPallet::approve_proposal(RuntimeOrigin::signed(1), hash, Vote::Aye));

		assert_noop!(
			ProposalPallet::approve_proposal(RuntimeOrigin::signed(1), hash, Vote::Aye),
			Error::<Test>::AlreadyApproved
		);
	});
}

#[test]
fn approve_wrong_proposal_fails() {
	new_test_ext().execute_with(|| {
		const TEST_ACCOUNT: <Test as frame_system::Config>::AccountId = 1;
		// let proposal = make_proposal(42);
		let hash = HashType::from(Hashing::hash_of(&42));
		// Dispatch a signed extrinsic.
		assert_ok!(ProposalPallet::add_community_member(RuntimeOrigin::signed(1), TEST_ACCOUNT));

		assert_ok!(ProposalPallet::add_committee_member(RuntimeOrigin::root(), TEST_ACCOUNT));

		assert_noop!(
			ProposalPallet::approve_proposal(RuntimeOrigin::signed(1), hash, Vote::Aye),
			Error::<Test>::ProposalMissing
		);
	});
}
