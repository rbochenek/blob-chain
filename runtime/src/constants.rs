use crate::{Balance, BlockNumber};

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

pub mod democracy {
	use super::*;
	// The period between a proposal being approved and enacted.
	//
	// It should generally be a little more than the unstake period to ensure that voting stakers
	// have an opportunity to remove themselves from the system in the case where they are on the
	// losing side of a vote.
	pub const ENACTMENT_PERIOD: BlockNumber = 12;
	// How often (in blocks) new public referenda are launched.
	pub const LAUNCH_PERIOD: BlockNumber = 8;
	// How often (in blocks) to check for new votes.
	pub const VOTING_PERIOD: BlockNumber = 4;
	// The minimum period of vote locking.
	//
	// It should be no shorter than enactment period to ensure that in the case of an approval,
	// those successful voters are locked into the consequences that their votes entail.
	pub const VOTE_LOCKING_PERIOD: BlockNumber = 12;
	// The minimum amount to be used as a deposit for a public referendum proposal.
	pub const MINIMUM_DEPOSIT: Balance = 100_000_000;
	// Indicator for whether an emergency origin is even allowed to happen. Some chains may want to
	// set this permanently to false, others may want to condition it on things such as an upgrade
	// having happened recently.
	pub const INSTANT_ALLOWED: bool = true;
	// Minimum voting period allowed for a fast-track referendum.
	pub const FAST_TRACK_VOTING_PERIOD: BlockNumber = 2;
	// Period in blocks where an external proposal may not be re-submitted after being vetoed.
	pub const COOLOFF_PERIOD: BlockNumber = 30;
	// The maximum number of votes for an account.
	//
	// Also used to compute weight, an overly big value can lead to extrinsic with very big weight:
	// see delegate for instance.
	pub const MAX_VOTES: u32 = 100;
	// The maximum number of public proposals that can exist at any time.
	pub const MAX_PROPOSALS: u32 = 20;
	// The maximum number of deposits a public proposal may have at any time.
	pub const MAX_DEPOSITS: u32 = 60;
	// The maximum number of items which can be blacklisted.
	pub const MAX_BLACKLISTED: u32 = 100;
}
