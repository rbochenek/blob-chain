//! Benchmarking setup for pallet-blobmanager
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Template;
use alloc::vec;
use frame_benchmarking::v2::*;
use frame_support::traits::Get;
use frame_system::RawOrigin;

#[benchmarks]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_uploader() {
		// Setup code
		let uploader: T::AccountId = whitelisted_caller();

		#[extrinsic_call]
		set_uploader(RawOrigin::Root, uploader.clone());

		// Verification code
		assert_eq!(Uploader::<T>::get(), Some(uploader));
	}

	#[benchmark]
	fn upload_blob() {
		// Setup code
		// Set Uploader
		let uploader: T::AccountId = whitelisted_caller();
		Template::<T>::set_uploader(RawOrigin::Root.into(), uploader.clone())
			.expect("set_uploader() fail");

		// Prepare blob to upload
		let blob = vec![0u8; <T as Config>::MaxBlobSize::get() as usize];

		#[extrinsic_call]
		upload_blob(RawOrigin::Signed(uploader), blob.clone());

		// Verification code
		assert_eq!(
			Blobs::<T>::iter_values().next().expect("No blobs stored").first(),
			Some(&blob.try_into().unwrap())
		);
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
