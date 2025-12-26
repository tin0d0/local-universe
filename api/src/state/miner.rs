use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::{miner_pda, Treasury};
use super::LocalUniverseAccount;

/// Tracks a miner's deployed SOL and reward balances for a dimension.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Miner {
    /// The authority of this miner account.
    pub authority: Pubkey,

    /// The dimension this miner is on.
    pub dimension_id: u64,

    /// SOL this miner has deployed this excavation (after fee).
    pub deployed: u64,

    /// SOL withheld in reserve to pay bots for checkpointing.
    pub checkpoint_fee: u64,

    /// The last excavation that this miner checkpointed.
    pub checkpoint_id: u64,

    /// The ID of the excavation this miner last played in.
    pub excavation_id: u64,

    /// The amount of SOL this miner can claim.
    pub rewards_sol: u64,

    /// The amount of LUXITE this miner can claim.
    pub rewards_luxite: u64,

    /// The amount of LUXITE this miner has earned from claim fees.
    pub refined_luxite: u64,

    /// The rewards factor last time rewards were updated (for 10% claim fee redistribution).
    pub rewards_factor: Numeric,

    /// The last time this miner claimed LUXITE rewards.
    pub last_claim_luxite_at: i64,

    /// The last time this miner claimed SOL rewards.
    pub last_claim_sol_at: i64,

    /// The total amount of SOL this miner has received back from hits.
    pub lifetime_rewards_sol: u64,

    /// The total amount of LUXITE this miner has earned.
    pub lifetime_rewards_luxite: u64,

    /// The total amount of SOL this miner has deployed across all excavations.
    pub lifetime_deployed: u64,

    /// Reserved for future use.
    pub buffer_a: u64,

    /// Reserved for future use.
    pub buffer_b: u64,

    /// Reserved for future use.
    pub buffer_c: u64,

    /// Reserved for future use.
    pub buffer_d: u64,
}

impl Miner {
    pub fn pda(&self) -> (Pubkey, u8) {
        miner_pda(self.dimension_id, self.authority)
    }

    /// Claims pending LUXITE rewards. Charges 10% fee redistributed to other miners.
    pub fn claim_luxite(&mut self, clock: &Clock, treasury: &mut Treasury) -> u64 {
        self.update_rewards(treasury);

        let refined = self.refined_luxite;
        let rewards = self.rewards_luxite;
        let total = refined + rewards;

        if total == 0 {
            return 0;
        }

        // Clear balances
        self.refined_luxite = 0;
        self.rewards_luxite = 0;
        self.last_claim_luxite_at = clock.unix_timestamp;

        // Update treasury tracking
        treasury.total_unclaimed = treasury.total_unclaimed.saturating_sub(rewards);
        treasury.total_refined = treasury.total_refined.saturating_sub(refined);

        // Charge 10% fee on mining rewards, redistribute to unclaimed miners
        let fee = if treasury.total_unclaimed > 0 && rewards > 0 {
            let f = rewards / 10;
            treasury.miner_rewards_factor += Numeric::from_fraction(f, treasury.total_unclaimed);
            treasury.total_refined += f;
            f
        } else {
            0
        };

        let amount = total - fee;
        self.lifetime_rewards_luxite += amount;
        amount
    }

    /// Claims pending SOL rewards (already on miner account from checkpoint).
    pub fn claim_sol(&mut self, clock: &Clock) -> u64 {
        let amount = self.rewards_sol;
        self.rewards_sol = 0;
        self.last_claim_sol_at = clock.unix_timestamp;
        self.lifetime_rewards_sol += amount;
        amount
    }

    /// Updates LUXITE rewards from 10% claim fee redistribution.
    pub fn update_rewards(&mut self, treasury: &Treasury) {
        if treasury.miner_rewards_factor <= self.rewards_factor {
            self.rewards_factor = treasury.miner_rewards_factor;
            return;
        }

        let accumulated = treasury.miner_rewards_factor - self.rewards_factor;
        let personal = accumulated * Numeric::from_u64(self.rewards_luxite);
        self.refined_luxite += personal.to_u64();
        self.rewards_factor = treasury.miner_rewards_factor;
    }
}

account!(LocalUniverseAccount, Miner);
