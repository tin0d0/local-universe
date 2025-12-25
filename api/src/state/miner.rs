use serde::{Deserialize, Serialize};
use steel::*;
use crate::state::{miner_pda, Drill, Treasury};
use super::LocalUniverseAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Miner {
    /// The authority of this miner account.
    pub authority: Pubkey,

    /// The dimension this miner is on.
    pub dimension_id: u64,

    /// SOL this miner has deployed this tick.
    pub deployed: u64,

    /// Pending LUXITE rewards to claim.
    pub rewards_luxite: u64,

    /// The rewards factor last time rewards were updated.
    pub rewards_factor: Numeric,

    /// Refined LUXITE from claim fees redistribution.
    pub refined_luxite: u64,

    /// The ID of the tick this miner last played in.
    pub tick_id: u64,

    /// The total amount of SOL this miner has deployed across all ticks.
    pub lifetime_deployed: u64,

    /// The last time this miner claimed LUXITE rewards.
    pub last_claim_luxite_at: i64,

    /// The total amount of LUXITE this miner has earned.
    pub lifetime_rewards_luxite: u64,

    /// Buffer a (placeholder).
    pub buffer_a: u64,

    /// Buffer b (placeholder).
    pub buffer_b: u64,
}

impl Miner {
    pub fn pda(&self) -> (Pubkey, u8) {
        miner_pda(self.dimension_id, self.authority)
    }

    /// Claims pending LUXITE rewards. Charges 10% fee redistributed to other miners.
    pub fn claim_luxite(&mut self, clock: &Clock, drill: &mut Drill, treasury: &mut Treasury) -> u64 {
        self.update_rewards(treasury);

        let refined_luxite = self.refined_luxite;
        let rewards_luxite = self.rewards_luxite;
        let mut amount = refined_luxite + rewards_luxite;

        self.refined_luxite = 0;
        self.rewards_luxite = 0;

        treasury.total_unclaimed -= rewards_luxite;
        treasury.total_refined -= refined_luxite;

        // Update drill tracking
        drill.total_unclaimed = drill.total_unclaimed.saturating_sub(rewards_luxite);

        self.last_claim_luxite_at = clock.unix_timestamp;

        // Charge 10% fee, redistribute to miners who haven't claimed yet
        if treasury.total_unclaimed > 0 {
            let fee = rewards_luxite / 10;
            amount -= fee;
            treasury.miner_rewards_factor += Numeric::from_fraction(fee, treasury.total_unclaimed);
            treasury.total_refined += fee;
        }

        self.lifetime_rewards_luxite += amount;
        amount
    }

    /// Updates rewards based on treasury's miner_rewards_factor.
    pub fn update_rewards(&mut self, treasury: &Treasury) {
        if treasury.miner_rewards_factor > self.rewards_factor {
            let accumulated_rewards = treasury.miner_rewards_factor - self.rewards_factor;
            let personal_rewards = accumulated_rewards * Numeric::from_u64(self.rewards_luxite);
            self.refined_luxite += personal_rewards.to_u64();
        }
        self.rewards_factor = treasury.miner_rewards_factor;
    }
}

account!(LocalUniverseAccount, Miner);
