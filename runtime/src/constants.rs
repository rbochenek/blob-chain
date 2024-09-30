use crate::{Balance, BlockNumber};

pub mod blobmanager {
	pub const MAX_BLOBS_PER_BLOCK: u32 = 5;
	pub const MAX_BLOB_SIZE: u32 = 1024 * 1024; // 1 MB
}

pub mod scheduler {
	pub const MAX_SCHEDULED_PER_BLOCK: u32 = 20;
}

pub mod referenda {
	use super::*;
	// The minimum amount to be used as a deposit for a public referendum proposal.
	pub const SUBMISSION_DEPOSIT: Balance = 100_000_000_000;
	// Maximum size of the referendum queue for a single track.
	pub const MAX_QUEUED: u32 = 5;
	// The number of blocks after submission that a referendum must begin being decided by. Once
	// this passes, then anyone may cancel the referendum.
	pub const UNDECIDING_TIMEOUT: BlockNumber = 20;
	// Quantization level for the referendum wakeup scheduler. A higher number will result in fewer
	// storage reads/writes needed for smaller voters, but also result in delays to the automatic
	// referendum status changes. Explicit servicing instructions are unaffected.
	pub const ALARM_INTERVAL: BlockNumber = 1;
}

pub mod conviction_voting {
	use super::*;
	// The maximum number of concurrent votes an account may have.
	//
	// Also used to compute weight, an overly large value can lead to extrinsics with large weight
	// estimation: see delegate for instance.
	pub const MAX_VOTES: u32 = 100;
	// The minimum period of vote locking.
	//
	// It should be no shorter than enactment period to ensure that in the case of an approval,
	// those successful voters are locked into the consequences that their votes entail.
	pub const VOTE_LOCKING_PERIOD: BlockNumber = 20;
}
