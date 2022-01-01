use crate::{mock::*, Error, KittiesCount, Owner};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		assert_eq!(KittiesCount::<Test>::get(), 1);
		assert_eq!(Owner::<Test>::get(1), Some(1));
	});
}

#[test]
fn create_failied_with_kitty_index_overflow() {
	new_test_ext().execute_with(|| {
		// Max kitty index has been mocked to 4, manually creates 3 first.
		assert_ok!(KittiesModule::create(Origin::signed(1), 1));
		assert_ok!(KittiesModule::create(Origin::signed(1), 1));
		assert_ok!(KittiesModule::create(Origin::signed(1), 1));

		// Should fails when trying to create the 3rd kitty.
		assert_noop!(
			KittiesModule::create(Origin::signed(1), 1),
			Error::<Test>::KittiesCountOverflow
		);
	})
}

#[test]
fn create_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_noop!(KittiesModule::create(Origin::signed(1), 20), Error::<Test>::NotEnoughBalance);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		assert_ok!(KittiesModule::transfer(Origin::signed(1), 2, 1));
		assert_eq!(Owner::<Test>::get(1), Some(2));
	});
}

#[test]
fn transfer_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 2, 2),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn transfer_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		assert_noop!(KittiesModule::transfer(Origin::signed(2), 1, 1), Error::<Test>::NotOwner);
	});
}

#[test]
fn transfer_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		// user 2 created a kitty before hand, will endup with 0 reservable balance when transfer.
		assert_ok!(KittiesModule::create(Origin::signed(2), 10));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(1), 2, 1),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn breed_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_ok!(KittiesModule::breed(Origin::signed(1), 1, 2, 2));
		assert_eq!(Owner::<Test>::get(3), Some(1));
	});
}

#[test]
fn breed_failed_with_same_parent_index() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 1, 1, 2),
			Error::<Test>::SameParentIndex
		);
	});
}

#[test]
fn breed_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		// 1st parent index invalid
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 3, 1, 2),
			Error::<Test>::InvalidKittyIndex
		);
		// 2nd parent index invalid
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 1, 3, 2),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn breed_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(KittiesModule::breed(Origin::signed(1), 1, 2, 2), Error::<Test>::NotOwner);
	});
}

#[test]
fn breed_failed_with_kitty_count_overflow() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 1, 2, 2),
			Error::<Test>::KittiesCountOverflow
		);
	});
}

#[test]
fn breed_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_ok!(KittiesModule::create(Origin::signed(1), 2));
		assert_noop!(
			KittiesModule::breed(Origin::signed(1), 1, 2, 10),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_ok!(KittiesModule::buy(Origin::signed(1), 1, 2));
	});
}

#[test]
fn buy_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(KittiesModule::buy(Origin::signed(1), 2, 2), Error::<Test>::InvalidKittyIndex);
	});
}

#[test]
fn buy_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(KittiesModule::buy(Origin::signed(1), 1, 3), Error::<Test>::NotOwner);
	});
}

#[test]
fn buy_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(KittiesModule::buy(Origin::signed(1), 2, 2), Error::<Test>::NotEnoughBalance);
	});
}

#[test]
fn sell_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_ok!(KittiesModule::sell(Origin::signed(2), 1, 1));
	});
}

#[test]
fn sell_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(
			KittiesModule::sell(Origin::signed(2), 2, 1),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn sell_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(KittiesModule::sell(Origin::signed(3), 1, 1), Error::<Test>::NotOwner);
	});
}

#[test]
fn sell_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		assert_ok!(KittiesModule::create(Origin::signed(1), 10));
		assert_ok!(KittiesModule::create(Origin::signed(2), 2));
		assert_noop!(KittiesModule::sell(Origin::signed(2), 2, 1), Error::<Test>::NotEnoughBalance);
	});
}
