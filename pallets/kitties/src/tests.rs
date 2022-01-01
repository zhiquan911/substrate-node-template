#![cfg(test)]

use crate::{mock::*, pallet::Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn should_build_genesis_kitties() {
	new_test_ext().execute_with(|| {
		// Check we have 2 kitties, as specified
		assert_eq!(Kitties::kitty_cnt(), 2);

		// Check owners own the correct amount of kitties
		let kitties_owned_by_1 = Kitties::kitties_owned(0);
		assert_eq!(kitties_owned_by_1, Some(ALICE));

		let kitties_owned_by_2 = Kitties::kitties_owned(1);
		assert_eq!(kitties_owned_by_2, Some(BOB));

		// Check that kitties are owned correctly
		let kitty1 = Kitties::kitties(0).expect("Could have this kitty ID owned by acct 1");
		assert_eq!(kitty1.owner, ALICE);

		let kitty2 = Kitties::kitties(1).expect("Could have this kitty ID owned by acct 2");
		assert_eq!(kitty2.owner, BOB);
	});
}

#[test]
fn create_kitty_unit_test() {
	new_test_ext().execute_with(|| {
		// should failed, InsufficientBalance
		assert_noop!(
			Kitties::create_kitty(Origin::signed(COCO)),
			pallet_balances::Error::<Test>::InsufficientBalance
		);

		assert_ok!(Balances::set_balance(Origin::root(), COCO, 10000, 0));

		// should work
		assert_ok!(Kitties::create_kitty(Origin::signed(COCO)));

		// Event log
		System::assert_has_event(Event::Kitties(crate::Event::Created(COCO, 2)));

		// check that 3 kitties exists (together with the two from genesis)
		assert_eq!(Kitties::kitty_cnt(), 3);

		// check that account ALICE owns 1 kitty
		assert_eq!(Kitties::kitties_owned(2), Some(COCO));

		// check that this kitty is specifically owned by account ALICE
		let kitty = Kitties::kitties(2).expect("should found the kitty");
		assert_eq!(kitty.owner, COCO);
		assert_eq!(kitty.price, None);

		//check COCO reserved balanc after create new kitty
		assert_eq!(KittyDeposit::get(), Balances::reserved_balance(COCO));
	});
}

#[test]
fn transfer_kitty_unit_test() {
	new_test_ext().execute_with(|| {
		assert_ok!(Balances::set_balance(Origin::root(), COCO, 1000, 0));

		// should failed, kitty is not exist
		assert_noop!(
			Kitties::transfer(Origin::signed(ALICE), COCO, 3),
			Error::<Test>::KittyNotExist
		);

		// should failed, wrong owner
		assert_noop!(
			Kitties::transfer(Origin::signed(ALICE), COCO, 1),
			Error::<Test>::NotKittyOwner
		);

		// should failed, can not transfer to self
		assert_noop!(
			Kitties::transfer(Origin::signed(ALICE), ALICE, 0),
			Error::<Test>::TransferToSelf
		);

		// should work
		assert_ok!(Kitties::transfer(Origin::signed(ALICE), COCO, 0));

		// Event log
		System::assert_has_event(Event::Kitties(crate::Event::Transferred(ALICE, COCO, 0)));

		// check new owner of kitty
		assert_eq!(Kitties::kitties_owned(0), Some(COCO));
		let kitty = Kitties::kitties(0).expect("should found the kitty");
		assert_eq!(kitty.owner, COCO);
	});
}

#[test]
fn sell_kitty_unit_test() {
	new_test_ext().execute_with(|| {
		// should failed, kitty is not exist
		assert_noop!(
			Kitties::sell_kitty(Origin::signed(ALICE), 3, Some(100)),
			Error::<Test>::KittyNotExist
		);

		// should failed, wrong owner
		assert_noop!(
			Kitties::sell_kitty(Origin::signed(ALICE), 1, Some(100)),
			Error::<Test>::NotKittyOwner
		);

		// should work
		assert_ok!(Kitties::sell_kitty(Origin::signed(ALICE), 0, Some(100)));

		// Event log
		System::assert_has_event(Event::Kitties(crate::Event::PriceSet(ALICE, 0, Some(100))));

		// check new price of kitty
		let kitty = Kitties::kitties(0).expect("should found the kitty");
		assert_eq!(kitty.price, Some(100));
	});
}

#[test]
fn buy_kitty_unit_test() {
	new_test_ext().execute_with(|| {
		// should failed, Buyer cannot be the owner.
		assert_noop!(
			Kitties::buy_kitty(Origin::signed(ALICE), 0, 50),
			Error::<Test>::BuyerIsKittyOwner
		);

		// should failed, kitty is not exist
		assert_noop!(Kitties::buy_kitty(Origin::signed(BOB), 2, 50), Error::<Test>::KittyNotExist);

		// should failed, kitty is not for sale
		assert_noop!(
			Kitties::buy_kitty(Origin::signed(BOB), 0, 50),
			Error::<Test>::KittyNotForSale
		);

		// set price
		assert_ok!(Kitties::sell_kitty(Origin::signed(ALICE), 0, Some(110000)));

		// should failed, the bid price is too low
		assert_noop!(
			Kitties::buy_kitty(Origin::signed(BOB), 0, 50),
			Error::<Test>::KittyBidPriceTooLow
		);

		// should failed, buyer is not enough balance
		assert_noop!(
			Kitties::buy_kitty(Origin::signed(BOB), 0, 110000),
			Error::<Test>::NotEnoughBalance
		);

		// set price
		assert_ok!(Kitties::sell_kitty(Origin::signed(ALICE), 0, Some(100)));

		// should work
		assert_ok!(Kitties::buy_kitty(Origin::signed(BOB), 0, 150));

		// Event log
		System::assert_has_event(Event::Kitties(crate::Event::Bought(BOB, ALICE, 0, 150)));

		// check kitty information
		let kitty = Kitties::kitties(0).expect("should found the kitty");
		assert_eq!(kitty.owner, BOB);
		assert_eq!(kitty.price, None);
	});
}

#[test]
fn breed_kitty_unit_test() {
	new_test_ext().execute_with(|| {
		// should failed, kitty is not exist
		assert_noop!(
			Kitties::breed_kitty(Origin::signed(ALICE), 0, 2),
			Error::<Test>::KittyNotExist
		);

		// should failed, wrong owner
		assert_noop!(
			Kitties::breed_kitty(Origin::signed(ALICE), 0, 1),
			Error::<Test>::NotKittyOwner
		);

		assert_ok!(Kitties::transfer(Origin::signed(BOB), ALICE, 1));

		// should work
		assert_ok!(Kitties::breed_kitty(Origin::signed(ALICE), 0, 1));

		// Event log
		System::assert_has_event(Event::Kitties(crate::Event::BreedKitty(ALICE, 0, 1, 2)));

		// check new price of kitty
		let kitty = Kitties::kitties(2).expect("should found the kitty");
		assert_eq!(kitty.price, None);
		assert_eq!(kitty.owner, ALICE);
	});
}
