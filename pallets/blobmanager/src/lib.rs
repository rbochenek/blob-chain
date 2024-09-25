// SPDX-License-Identifier: Unlicense

//! # BlobManager Pallet
//! A pallet for storing blobs on-chain
//!
//! - [`Config`]
//! - [`Call`]
//!
//! # Overview
//!
//! This pallet contains functionality for storing blobs (Binary Large Objects) using on-chain
//! storage

// NOTE: PoV limits size of the blob (5 MB): https://github.com/paritytech/polkadot-sdk/blob/c987da33935898cd5b2f8605d548bc48727c1815/polkadot/primitives/src/v8/mod.rs#L429

// Ensure we're 'no_std' when compiling for WebAssembly.
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;
use alloc::vec::Vec;

pub use pallet::*;

mod benchmarking;
pub mod weights;
pub use weights::*;

// WARNING: Uses 'Dev Mode' to simplify things for now. Do NOT use in production.
// See: https://paritytech.github.io/polkadot-sdk/master/frame_support/attr.pallet.html#dev-mode-palletdev_mode
// TODO: Tests
#[frame_support::pallet(dev_mode)]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
		type WeightInfo: WeightInfo;
		/// The maximum number of blobs stored per block
		type MaxBlobsPerBlock: Get<u32>;
	}

	#[pallet::storage]
	pub type Admin<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	pub type Uploader<T: Config> = StorageValue<_, T::AccountId, OptionQuery>;

	#[pallet::storage]
	pub type Blobs<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		BlockNumberFor<T>,
		BoundedVec<Vec<u8>, T::MaxBlobsPerBlock>,
		ValueQuery,
	>;

	// Errors that can be returned by this pallet
	#[pallet::error]
	pub enum Error<T> {
		// Admin is not set
		AdminNotSet,
		// Only callable by Admin
		CallableByAdminOnly,
		// Uploader not set
		UploaderNotSet,
		// Only callable by Uploader
		CallableByUploaderOnly,
		// Trying to append too many blobs in the current block
		ExceededMaxBlobsPerBlock,
	}

	// Events that can be emitted
	#[pallet::event]
	#[pallet::generate_deposit(fn deposit_event)]
	pub enum Event<T: Config> {
		/// Blob stored
		BlobStored,
	}

	// Genesis config
	#[pallet::genesis_config]
	pub struct GenesisConfig<T: Config> {
		pub admin: Option<T::AccountId>,
		pub uploader: Option<T::AccountId>,
	}

	// Genesis config (default)
	impl<T: Config> Default for GenesisConfig<T> {
		fn default() -> Self {
			Self { admin: None, uploader: None }
		}
	}

	// Genesis config build
	#[pallet::genesis_build]
	impl<T: Config> BuildGenesisConfig for GenesisConfig<T> {
		fn build(&self) {
			if let Some(admin) = &self.admin {
				Admin::<T>::put(admin);
			}

			if let Some(uploader) = &self.uploader {
				Uploader::<T>::put(uploader);
			}
		}
	}

	// Dispatchable functions
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Set new Uploader
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::set_uploader())]
		pub fn set_uploader(origin: OriginFor<T>, uploader: T::AccountId) -> DispatchResult {
			let sender = ensure_signed_or_root(origin)?;

			// Validate Admin
			if let Some(sender_other_than_root) = sender {
				if let Some(admin) = Admin::<T>::get() {
					ensure!(sender_other_than_root == admin, Error::<T>::CallableByAdminOnly);
				} else {
					return Err(Error::<T>::AdminNotSet.into());
				}
			}

			// Update Uploader
			Uploader::<T>::put(uploader);

			Ok(())
		}

		/// Upload new Blob
		/// Only callable by Uploader
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::upload_blob())]
		pub fn upload_blob(origin: OriginFor<T>, blob: Vec<u8>) -> DispatchResult {
			let sender = ensure_signed(origin)?;

			// Only callable by Uploader
			if let Some(uploader) = Uploader::<T>::get() {
				ensure!(sender == uploader, Error::<T>::CallableByUploaderOnly);
			} else {
				return Err(Error::<T>::UploaderNotSet.into());
			}

			// Get current block number
			let block_number = <frame_system::Pallet<T>>::block_number();

			// Get vector for current block from storage
			let mut blob_vec = Blobs::<T>::get(block_number);

			// Append blob
			blob_vec.try_push(blob).map_err(|_| Error::<T>::ExceededMaxBlobsPerBlock)?;

			// Store Blobs
			Blobs::<T>::insert(block_number, blob_vec);

			// Emit BlobStored event
			Self::deposit_event(Event::BlobStored);

			Ok(())
		}
	}
}
