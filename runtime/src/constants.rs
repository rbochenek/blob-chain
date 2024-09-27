use crate::BlockNumber;

pub mod blobmanager {
	pub const MAX_BLOBS_PER_BLOCK: u32 = 5;
	pub const MAX_BLOB_SIZE: u32 = 1024 * 1024; // 1 MB
}

pub mod scheduler {
	pub const MAX_SCHEDULED_PER_BLOCK: u32 = 20;
}

pub mod council_collective {
	use super::*;
	// The time-out for council motions.
	pub const MOTION_DURATION: BlockNumber = 20;
	// Maximum number of proposals allowed to be active in parallel.
	pub const MAX_PROPOSALS: u32 = 10;
	// The maximum number of members supported by the pallet. Used for weight estimation.
	//
	// Benchmarks will need to be re-run and weights adjusted if this changes.
	// This pallet assumes that dependents keep to the limit without enforcing it.
	pub const MAX_MEMBERS: u32 = 15;
}

pub mod technicalcommittee_collective {
	use super::*;
	// The time-out for council motions.
	pub const MOTION_DURATION: BlockNumber = 16;
	// Maximum number of proposals allowed to be active in parallel.
	pub const MAX_PROPOSALS: u32 = 4;
	// The maximum number of members supported by the pallet. Used for weight estimation.
	//
	// Benchmarks will need to be re-run and weights adjusted if this changes.
	// This pallet assumes that dependents keep to the limit without enforcing it.
	pub const MAX_MEMBERS: u32 = 10;
}
