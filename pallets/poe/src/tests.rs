use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_unit_test() {
	new_test_ext().execute_with(|| {
		let proof = vec![1u8; 10];
		// should work
		assert_ok!(POE::create_claim(Origin::signed(ALICE), proof.clone()));

		// Event log
		System::assert_has_event(Event::POE(crate::Event::ClaimCreated(ALICE, proof.clone())));

		// check proof value
		assert_eq!(POE::get_proofs(&proof).0, ALICE);
		assert_eq!(POE::get_proofs(&proof).1, 1);

		// failed, ProofAlreadyClaimed
		assert_noop!(
			POE::create_claim(Origin::signed(ALICE), proof),
			Error::<Test>::ProofAlreadyClaimed
		);

		let proof = vec![1u8; 21];

		// failed, ProofLengthOverflow
		assert_noop!(
			POE::create_claim(Origin::signed(ALICE), proof),
			Error::<Test>::ProofLengthOverflow
		);
	});
}

#[test]
fn revoke_claim_unit_test() {
	new_test_ext().execute_with(|| {
		let proof = vec![1u8; 10];
		// failed, proof is not existed
		assert_noop!(
			POE::revoke_claim(Origin::signed(ALICE), proof.clone()),
			Error::<Test>::NoSuchProof
		);

		// create_claim
		assert_ok!(POE::create_claim(Origin::signed(ALICE), proof.clone()));

		// failed, wrong origin
		assert_noop!(
			POE::revoke_claim(Origin::signed(BOB), proof.clone()),
			Error::<Test>::NotProofOwner
		);

		// should work
		assert_ok!(POE::revoke_claim(Origin::signed(ALICE), proof.clone()));

		// Event log
		System::assert_has_event(Event::POE(crate::Event::ClaimRevoked(ALICE, proof.clone())));
	});
}

#[test]
fn transfer_claim_unit_test() {
	new_test_ext().execute_with(|| {
		let proof = vec![1u8; 10];
		// failed, proof is not existed
		assert_noop!(
			POE::transfer_claim(Origin::signed(ALICE), BOB, proof.clone()),
			Error::<Test>::NoSuchProof
		);

		// create_claim
		assert_ok!(POE::create_claim(Origin::signed(ALICE), proof.clone()));

		// failed, wrong origin
		assert_noop!(
			POE::transfer_claim(Origin::signed(BOB), ALICE, proof.clone()),
			Error::<Test>::NotProofOwner
		);

		// should work
		assert_ok!(POE::transfer_claim(Origin::signed(ALICE), BOB, proof.clone()));

		// Event log
		System::assert_has_event(Event::POE(crate::Event::ClaimTransfered(
			ALICE,
			BOB,
			proof.clone(),
		)));
	});
}

#[test]
fn create_claim_length_overflow() {
	new_test_ext().execute_with(|| {
		let proof = vec![1u8; 21];

		// failed, ProofLengthOverflow
		assert_noop!(
			POE::create_claim(Origin::signed(ALICE), proof),
			Error::<Test>::ProofLengthOverflow
		);
	});
}
