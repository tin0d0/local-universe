use serde::{Deserialize, Serialize};
use steel::*;
use crate::state::treasury_pda;
use super::LocalUniverseAccount;

/// Treasury is a singleton account which is the mint authority for the LUXITE token
/// and manages protocol revenue, buybacks, burns, and emissions.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Treasury {
    /// The amount of SOL collected for buyback-burn operations.
    pub sol_balance: u64,

    /// The amount of LUXITE remaining for mining emissions.
    pub luxite_balance: u64,

    /// The cumulative LUXITE distributed to miners, divided by total unclaimed at time of distribution.
    pub miner_rewards_factor: Numeric,

    /// The cumulative LUXITE distributed to stakers, divided by total stake at time of distribution.
    pub stake_rewards_factor: Numeric,

    /// The current total amount of refined LUXITE mining rewards.
    pub total_refined: u64,

    /// The current total amount of LUXITE staking deposits.
    pub total_staked: u64,

    /// The current total amount of unclaimed LUXITE mining rewards.
    pub total_unclaimed: u64,

    /// The total amount of LUXITE emitted from mining.
    pub total_emitted: u64,

    /// The total amount of LUXITE burned from buybacks.
    pub total_burned: u64,

    /// Buffer a (placeholder).
    pub buffer_a: u64,

    /// Buffer b (placeholder).
    pub buffer_b: u64,

    /// Buffer c (placeholder).
    pub buffer_c: u64,
}

impl Treasury {
    pub fn pda() -> (Pubkey, u8) {
        treasury_pda()
    }
}

account!(LocalUniverseAccount, Treasury);
