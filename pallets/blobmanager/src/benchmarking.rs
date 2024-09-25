//! Benchmarking setup for pallet-blobmanager
#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as Template;
use alloc::vec;
use frame_benchmarking::v2::*;
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
		// TODO: This benchmark is flawed since the pallet storage is unbounded at this point
		let blob = vec![9; 1024 * 1024]; // 1 MB

		#[extrinsic_call]
		upload_blob(RawOrigin::Signed(uploader), blob.clone());

		// Verification code
		assert_eq!(Blobs::<T>::iter_values().next().expect("No blobs stored").first(), Some(&blob));
	}

	impl_benchmark_test_suite!(Template, crate::mock::new_test_ext(), crate::mock::Test);
}
