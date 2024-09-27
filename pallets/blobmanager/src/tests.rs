#![cfg(test)]
use super::*;
use crate::{mock::*, Blobs, Error, Event, Uploader};
use frame_support::{assert_noop, assert_ok, pallet_prelude::DispatchError, traits::Get};

#[test]
fn genesis_config_uploader() {
	new_test_ext().execute_with(|| {
		assert_eq!(Uploader::<Test>::get(), Some(2));
	})
}

#[test]
fn set_uploader_works() {
	new_test_ext().execute_with(|| {
		assert_ok!(BlobManager::set_uploader(RuntimeOrigin::root(), 3));
		assert_eq!(Uploader::<Test>::get(), Some(3));
	})
}

#[test]
fn set_uploader_wrong_origin() {
	new_test_ext().execute_with(|| {
		assert_noop!(
			BlobManager::set_uploader(RuntimeOrigin::signed(1), 2),
			DispatchError::BadOrigin
		);
	})
}

#[test]
fn upload_blob_works() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Prepare blob
		let max_blob_size: u32 = <Test as Config>::MaxBlobSize::get();
		let blob = vec![42u8; max_blob_size as usize];
		// Store blob
		assert_ok!(BlobManager::upload_blob(RuntimeOrigin::signed(2), blob.clone()));
		// Make sure event was deposited
		System::assert_last_event(Event::BlobStored.into());
		// Verify storage
		assert_eq!(
			Blobs::<Test>::iter_values().next().expect("No blobs stored").first(),
			Some(&blob.try_into().unwrap())
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
fn upload_blob_exceeds_blobs_per_block() {
	new_test_ext().execute_with(|| {
		// Go past genesis block so events get deposited
		System::set_block_number(1);
		// Prepare 1k blob
		let max_blob_size: u32 = <Test as Config>::MaxBlobSize::get();
		let blob = vec![0u8; max_blob_size as usize];
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

#[test]
fn upload_blob_exceeds_max_blob_size() {
	new_test_ext().execute_with(|| {
		let max_blob_size: u32 = <Test as Config>::MaxBlobSize::get();
		let blob = vec![0u8; (max_blob_size + 1) as usize];
		assert_noop!(
			BlobManager::upload_blob(RuntimeOrigin::signed(2), blob),
			Error::<Test>::ExceededMaxBlobSize
		);
	})
}
