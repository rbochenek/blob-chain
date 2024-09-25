#![cfg(test)]
use super::*;
use crate::{mock::*, Admin, Blobs, Error, Event, Uploader};
use frame_support::{assert_noop, assert_ok, traits::Get};

#[test]
fn genesis_config_admin() {
	new_test_ext().execute_with(|| {
		assert_eq!(Admin::<Test>::get(), Some(1));
	})
}

#[test]
fn genesis_config_uploader() {
	new_test_ext().execute_with(|| {
		assert_eq!(Uploader::<Test>::get(), Some(2));
	})
}

#[test]
fn set_uploader_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(BlobManager::set_uploader(RuntimeOrigin::signed(1), 2));
		assert_eq!(Uploader::<Test>::get(), Some(2));
	})
}

#[test]
fn set_uploader_admin_not_set() {
	new_test_ext().execute_with(|| {
		Admin::<Test>::set(None);
		assert_noop!(
			BlobManager::set_uploader(RuntimeOrigin::signed(1), 2),
			Error::<Test>::AdminNotSet
		);
	});
}

#[test]
fn set_uploader_wrong_origin() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			BlobManager::set_uploader(RuntimeOrigin::signed(2), 2),
			Error::<Test>::CallableByAdminOnly
		);
	})
}

#[test]
fn set_uploader_root_origin() {
	new_test_ext().execute_with(|| {
		assert_ok!(BlobManager::set_uploader(RuntimeOrigin::root(), 2));
		assert_eq!(Uploader::<Test>::get(), Some(2));
	})
}

#[test]
fn upload_blob_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Store blob
		let blob = vec![1, 2, 3, 4, 5];
		assert_ok!(BlobManager::upload_blob(RuntimeOrigin::signed(2), blob.clone()));
		// Make sure event was deposited
		System::assert_last_event(Event::BlobStored.into());
		// Verify storage
		assert_eq!(
			Blobs::<Test>::iter_values().next().expect("No blobs stored").first(),
			Some(&blob)
		);
	})
}

#[test]
fn upload_blob_wrong_origin() {
	new_test_ext().execute_with(|| {
		let blob = vec![0u8, 32];
		assert_noop!(
			BlobManager::upload_blob(RuntimeOrigin::signed(1), blob),
			Error::<Test>::CallableByUploaderOnly
		);
	})
}

#[test]
fn upload_blob_uploader_not_set() {
	new_test_ext().execute_with(|| {
		// Set Uploader to None
		Uploader::<Test>::set(None);
		// Try storing a blob
		let blob = vec![0u8; 32];
		assert_noop!(
			BlobManager::upload_blob(RuntimeOrigin::signed(1), blob),
			Error::<Test>::UploaderNotSet
		);
	})
}

#[test]
fn upload_blob_too_much_blobs_per_block() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Prepare 1k blob
		let blob = vec![0u8; 1024 * 1024];
		// Store maximum number of blobs per block
		let maxblobs: u32 = <Test as Config>::MaxBlobsPerBlock::get();
		for _ in 0..maxblobs {
			assert_ok!(BlobManager::upload_blob(RuntimeOrigin::signed(2), blob.clone()));
			System::assert_last_event(Event::BlobStored.into());
		}
		// Storing one more blob should fail
		assert_noop!(
			BlobManager::upload_blob(RuntimeOrigin::signed(2), blob.clone()),
			Error::<Test>::ExceededMaxBlobsPerBlock
		);
	})
}
