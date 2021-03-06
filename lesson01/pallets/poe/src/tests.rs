use crate::{mock::*, Error, Proofs};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((1, <frame_system::Pallet<Test>>::block_number()))
		)
	});
}

#[test]
#[ignore]
fn create_claim_falied_with_bad_origin() {
	assert_noop!(
		PoeModule::create_claim(Origin::root(), "un".to_string().as_bytes().to_vec()),
		frame_support::error::BadOrigin
	);
}

#[test]
fn create_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let limit_size = MaxClaimLength::get() as usize;

		assert_noop!(
			PoeModule::create_claim(
				Origin::signed(1),
				vec![0u8; limit_size.checked_add(1).unwrap()]
			),
			Error::<Test>::ClaimOverflow
		);
	})
}

#[test]
fn create_claim_failed_when_claim_already_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::create_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ProofAlreadyExist
		);
	})
}

#[test]
fn revoke_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];
		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
		assert_eq!(Proofs::<Test>::get(&claim), None);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let limit_size = MaxClaimLength::get() as usize;

		assert_noop!(
			PoeModule::revoke_claim(
				Origin::signed(1),
				vec![0u8; limit_size.checked_add(1).unwrap()]
			),
			Error::<Test>::ClaimOverflow
		);
	})
}

#[test]
fn revoke_claim_failed_when_claim_is_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(1), claim.clone()),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn revoke_claim_failed_when_origin_is_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::revoke_claim(Origin::signed(2), claim.clone()),
			Error::<Test>::NotClaimOwner
		);
	})
}

#[test]
fn transfer_claim_works() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2));
		assert_eq!(
			Proofs::<Test>::get(&claim),
			Some((2, <frame_system::Pallet<Test>>::block_number()))
		)
	})
}

#[test]
fn transfer_claim_failed_when_claim_is_too_long() {
	new_test_ext().execute_with(|| {
		let limit_size = MaxClaimLength::get() as usize;

		assert_noop!(
			PoeModule::transfer_claim(
				Origin::signed(1),
				vec![0u8; limit_size.checked_add(1).unwrap()],
				2
			),
			Error::<Test>::ClaimOverflow
		);
	})
}

#[test]
fn transfer_claim_failed_when_claim_not_exist() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2),
			Error::<Test>::ClaimNotExist
		);
	})
}

#[test]
fn transfer_claim_failed_when_not_claim_owner() {
	new_test_ext().execute_with(|| {
		let claim = vec![0, 1];

		let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

		assert_noop!(
			PoeModule::transfer_claim(Origin::signed(2), claim.clone(), 3),
			Error::<Test>::NotClaimOwner
		);
	})
}
