#![cfg(test)]
use super::*;
use crate as pallet_blobmanager;
use frame_support::{derive_impl, sp_runtime::BuildStorage, traits::ConstU32};

type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
pub enum Test {
	System: frame_system,
	BlobManager: pallet_blobmanager,
}
);

#[derive_impl(frame_system::config_preludes::TestDefaultConfig)]
impl frame_system::Config for Test {
	type Block = Block;
}

impl Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type WeightInfo = ();
	type MaxBlobsPerBlock = ConstU32<4>;
}

// Build genesis storage according to the mock runtime
pub fn new_test_ext() -> sp_io::TestExternalities {
	let mut t = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	// Set Admin to Account 1, Uploader Account 2
	let genesis = pallet_blobmanager::GenesisConfig::<Test> { admin: Some(1), uploader: Some(2) };
	genesis.assimilate_storage(&mut t).unwrap();
	t.into()
}
