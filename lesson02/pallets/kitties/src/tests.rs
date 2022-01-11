use super::*;
use crate::mock::{new_test_ext, Event as TestEvent, KittiesModule, Origin, System, Test};
use frame_support::{assert_noop, assert_ok};

#[test]
fn create_works() {
	new_test_ext().execute_with(|| {
		let account_id: u64 = 1;
		let kitty_id: u32 = 0;
		let kitty_price = 10;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_eq!(Owner::<Test>::get(kitty_id), Some(account_id));
		assert_has_event!(Event::<Test>::KittyCreate(account_id, kitty_id));
	});
}

#[test]
fn create_failied_with_kitty_index_overflow() {
	new_test_ext().execute_with(|| {
		KittiesCount::<Test>::put(u32::MAX);

		let account_id = 1;
		let kitty_price = 1;
		assert_noop!(
			KittiesModule::create(Origin::signed(account_id), kitty_price),
			Error::<Test>::KittiesCountOverflow
		);
	})
}

#[test]
fn create_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_price = 20;
		assert_noop!(
			KittiesModule::create(Origin::signed(account_id), kitty_price),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn transfer_works() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let transfer_account_id = 2;
		let kitty_price = 10;
		let kitty_id = 0;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_ok!(KittiesModule::transfer(
			Origin::signed(account_id),
			transfer_account_id,
			kitty_id
		));
		assert_eq!(Owner::<Test>::get(kitty_id), Some(transfer_account_id));
		assert_has_event!(Event::<Test>::KittyTransfer(account_id, transfer_account_id, kitty_id));
	});
}

#[test]
fn transfer_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let transfer_account_id = 2;
		let kitty_price = 10;
		let invalid_kitty_id = 2;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_noop!(
			KittiesModule::transfer(
				Origin::signed(account_id),
				transfer_account_id,
				invalid_kitty_id
			),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn transfer_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let invalid_owner_account_id = 2;
		let kitty_price = 10;
		let kitty_id = 0;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_noop!(
			KittiesModule::transfer(Origin::signed(invalid_owner_account_id), account_id, kitty_id),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn transfer_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let not_enough_balance_account_id = 100;
		let kitty_price = 10;
		let kitty_id = 0;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_noop!(
			KittiesModule::transfer(
				Origin::signed(account_id),
				not_enough_balance_account_id,
				kitty_id
			),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn breed_works() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_price = 2;
		let parent_kitty_id_1 = 0;
		let parent_kitty_id_2 = 1;
		let breed_kitty_id = 2;

		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_ok!(KittiesModule::breed(
			Origin::signed(account_id),
			parent_kitty_id_1,
			parent_kitty_id_2,
			kitty_price
		));
		assert_eq!(Owner::<Test>::get(breed_kitty_id), Some(account_id));
		assert_has_event!(Event::<Test>::KittyCreate(account_id, breed_kitty_id));
	});
}

#[test]
fn breed_failed_with_same_parent_index() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_id = 1;
		let same_kitty_id = 1;
		let kitty_price = 2;
		assert_noop!(
			KittiesModule::breed(Origin::signed(account_id), kitty_id, same_kitty_id, kitty_price),
			Error::<Test>::SameParentIndex
		);
	});
}

#[test]
fn breed_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_price = 2;
		let valid_parent_kitty_id = 0;
		let invalid_parent_kitty_id = 3;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		// 1st parent index invalid
		assert_noop!(
			KittiesModule::breed(
				Origin::signed(account_id),
				invalid_parent_kitty_id,
				valid_parent_kitty_id,
				kitty_price
			),
			Error::<Test>::InvalidKittyIndex
		);
		// 2nd parent index invalid
		assert_noop!(
			KittiesModule::breed(
				Origin::signed(account_id),
				valid_parent_kitty_id,
				invalid_parent_kitty_id,
				kitty_price
			),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn breed_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let yet_another_account_id = 2;
		let kitty_price = 2;
		let parent_kitty_id_1 = 0;
		let parent_kitty_id_2 = 1;
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_ok!(KittiesModule::create(Origin::signed(yet_another_account_id), kitty_price));
		assert_noop!(
			KittiesModule::breed(
				Origin::signed(account_id),
				parent_kitty_id_1,
				parent_kitty_id_2,
				kitty_price
			),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn breed_failed_with_kitty_count_overflow() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_price = 2;
		let parent_kitty_id_1 = 0;
		let parent_kitty_id_2 = 1;

		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));

		KittiesCount::<Test>::put(u32::MAX);

		assert_noop!(
			KittiesModule::breed(
				Origin::signed(account_id),
				parent_kitty_id_1,
				parent_kitty_id_2,
				kitty_price
			),
			Error::<Test>::KittiesCountOverflow
		);
	});
}

#[test]
fn breed_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let account_id = 1;
		let kitty_price = 2;
		let max_kitty_price = 10;
		let parent_kitty_id_1 = 0;
		let parent_kitty_id_2 = 1;

		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));
		assert_ok!(KittiesModule::create(Origin::signed(account_id), kitty_price));

		assert_noop!(
			KittiesModule::breed(
				Origin::signed(account_id),
				parent_kitty_id_1,
				parent_kitty_id_2,
				max_kitty_price
			),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn buy_works() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let kitty_id = 0;
		let kitty_price = 2;
		let seller_account_id = 2;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id), kitty_price));
		assert_ok!(KittiesModule::buy(
			Origin::signed(buyer_account_id),
			kitty_id,
			seller_account_id
		));
		assert_has_event!(Event::<Test>::KittyBuy(seller_account_id, buyer_account_id, kitty_id));
	});
}

#[test]
fn buy_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let invalid_kitty_id = 10;
		let kitty_price = 2;
		let seller_account_id = 2;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id), kitty_price));
		assert_noop!(
			KittiesModule::buy(
				Origin::signed(buyer_account_id),
				invalid_kitty_id,
				seller_account_id
			),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn buy_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let kitty_id = 0;
		let kitty_price = 2;
		let seller_account_id = 2;
		let invalid_owner_account_id = 100;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id), kitty_price));
		assert_noop!(
			KittiesModule::buy(
				Origin::signed(buyer_account_id),
				kitty_id,
				invalid_owner_account_id
			),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn buy_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let kitty_id = 1;
		let kitty_price = 2;
		let max_balance = 10;
		let seller_account_id_1 = 1;
		let seller_account_id_2 = 2;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id_1), max_balance));
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id_2), kitty_price));
		assert_noop!(
			KittiesModule::buy(Origin::signed(buyer_account_id), kitty_id, seller_account_id_2),
			Error::<Test>::NotEnoughBalance
		);
	});
}

#[test]
fn sell_works() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let kitty_id = 0;
		let kitty_price = 2;
		let seller_account_id = 2;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id), kitty_price));
		assert_ok!(KittiesModule::sell(
			Origin::signed(seller_account_id),
			kitty_id,
			buyer_account_id
		));
		assert_has_event!(Event::<Test>::KittySell(seller_account_id, buyer_account_id, kitty_id));
	});
}

#[test]
fn sell_failed_with_invalid_kitty_index() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let invalid_kitty_id = 10;
		let kitty_price = 2;
		let seller_account_id = 2;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id), kitty_price));
		assert_noop!(
			KittiesModule::sell(
				Origin::signed(seller_account_id),
				invalid_kitty_id,
				buyer_account_id
			),
			Error::<Test>::InvalidKittyIndex
		);
	});
}

#[test]
fn sell_failed_with_not_owner() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let kitty_id = 0;
		let kitty_price = 2;
		let seller_account_id = 2;
		let invalid_owner_account_id = 100;
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id), kitty_price));
		assert_noop!(
			KittiesModule::sell(
				Origin::signed(invalid_owner_account_id),
				kitty_id,
				buyer_account_id
			),
			Error::<Test>::NotOwner
		);
	});
}

#[test]
fn sell_failed_with_not_enough_balance() {
	new_test_ext().execute_with(|| {
		let buyer_account_id = 1;
		let kitty_id = 1;
		let kitty_price = 2;
		let max_balance = 10;
		let seller_account_id_1 = 1;
		let seller_account_id_2 = 2;

		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id_1), max_balance));
		assert_ok!(KittiesModule::create(Origin::signed(seller_account_id_2), kitty_price));
		assert_noop!(
			KittiesModule::sell(Origin::signed(seller_account_id_2), kitty_id, buyer_account_id),
			Error::<Test>::NotEnoughBalance
		);
	});
}
