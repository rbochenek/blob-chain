use crate::{Balance, BlockNumber};
use sp_staking::{EraIndex, SessionIndex};

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

pub mod staking {
	use super::*;
	// Number of eras to keep in history.
	//
	// Following information is kept for eras in [current_era - HistoryDepth, current_era]:
	// ErasStakers, ErasStakersClipped, ErasValidatorPrefs, ErasValidatorReward, ErasRewardPoints,
	// ErasTotalStake, ErasStartSessionIndex, ClaimedRewards, ErasStakersPaged, ErasStakersOverview.
	//
	// Must be more than the number of eras delayed by session. I.e. active era must always be in
	// history. I.e. active_era > current_era - history_depth must be guaranteed.
	//
	// If migrating an existing pallet from storage value to config value, this should be set to
	// same value or greater as in storage.
	//
	// Note: HistoryDepth is used as the upper bound for the BoundedVec item
	// StakingLedger.legacy_claimed_rewards. Setting this value lower than the existing value can
	// lead to inconsistencies in the StakingLedger and will need to be handled properly in a
	// migration. The test reducing_history_depth_abrupt shows this effect.
	pub const HISTORY_DEPTH: u32 = 100;
	// Number of sessions per era.
	pub const SESSIONS_PER_ERA: SessionIndex = 6;
	// Number of eras that staked funds must remain bonded for.
	pub const BONDING_DURATION: EraIndex = 28;
	// Number of eras that slashes are deferred by, after computation.
	//
	// This should be less than the bonding duration. Set to 0 if slashes should be applied
	// immediately, without opportunity for intervention.
	pub const SLASH_DEFER_DURATION: EraIndex = 7;
	// The maximum size of each T::ExposurePage.
	//
	// An ExposurePage is weakly bounded to a maximum of MaxExposurePageSize nominators.
	//
	// For older non-paged exposure, a reward payout was restricted to the top MaxExposurePageSize
	// nominators. This is to limit the i/o cost for the nominator payout.
	//
	// Note: MaxExposurePageSize is used to bound ClaimedRewards and is unsafe to reduce without
	// handling it in a migration.
	pub const MAX_EXPOSURE_PAGE_SIZE: u32 = 20;
	// The maximum number of unlocking chunks a StakingLedger can have. Effectively determines how
	// many unique eras a staker may be unbonding in.
	//
	// Note: MaxUnlockingChunks is used as the upper bound for the BoundedVec item
	// StakingLedger.unlocking. Setting this value lower than the existing value can lead to
	// inconsistencies in the StakingLedger and will need to be handled properly in a runtime
	// migration. The test reducing_max_unlocking_chunks_abrupt shows this effect.
	pub const MAX_UNLOCKING_CHUNKS: u32 = 10;
	// The maximum amount of controller accounts that can be deprecated in one call.
	pub const MAX_CONTROLLERS_IN_DEPRECATION_BATCH: u32 = 5;
}
