use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok};

#[test]
fn it_works_for_add_club_member() {
	new_test_ext().execute_with(|| {
		// Dispatch a signed extrinsic.
		assert_ok!(PalletClub::add_club_member(Origin::root(), 42_i32.to_be_bytes().to_vec(), 23));
		let club = PalletClub::club(42_i32.to_be_bytes().to_vec());
		// Read pallet storage and assert an expected result.
		assert_eq!(PalletClub::club(42_i32.to_be_bytes().to_vec()), club);
	})
}

#[test]
fn correct_error_for_none_value() {
	new_test_ext().execute_with(|| {
		assert_ok!(PalletClub::add_club_member(Origin::root(), 42_i32.to_be_bytes().to_vec(), 23));
		assert_ok!(PalletClub::remove_club_member(Origin::root(), 42_i32.to_be_bytes().to_vec(), 23));
		let club = PalletClub::club(0_i32.to_be_bytes().to_vec());
		assert_eq!(PalletClub::club(42_i32.to_be_bytes().to_vec()), club);

	});
}
