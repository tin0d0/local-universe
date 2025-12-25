use serde::{Deserialize, Serialize};
use steel::*;
use crate::state::drill_pda;
use super::LocalUniverseAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Drill {
    /// The dimension this drill is on.
    pub dimension_id: u64,

    /// Total SOL deployed this tick.
    pub total_deployed: u64,

    /// Lifetime SOL deployed across all ticks.
    pub lifetime_deployed: u64,

    /// Lifetime LUXITE earned across all ticks.
    pub lifetime_rewards_luxite: u64,

    /// Number of active miners this tick.
    pub miner_count: u64,

    /// Current mining depth.
    pub depth: u64,

    /// The ID of the tick this drill last played in.
    pub tick_id: u64,

    /// The hash of the end slot, used for RNG.
    pub slot_hash: [u8; 32],

    /// The cumulative LUXITE distributed, divided by total deployed at time of distribution.
    pub rewards_factor: Numeric,

    /// Total unclaimed LUXITE rewards on this drill.
    pub total_unclaimed: u64,

    /// Total refined LUXITE from claim fees.
    pub total_refined: u64,

    /// Buffer a (placeholder).
    pub buffer_a: u64,
}

impl Drill {
    pub fn pda(&self) -> (Pubkey, u8) {
        drill_pda(self.dimension_id)
    }

    pub fn rng(&self) -> Option<u64> {
        if self.slot_hash == [0; 32] || self.slot_hash == [u8::MAX; 32] {
            return None;
        }
        let r1 = u64::from_le_bytes(self.slot_hash[0..8].try_into().unwrap());
        let r2 = u64::from_le_bytes(self.slot_hash[8..16].try_into().unwrap());
        let r3 = u64::from_le_bytes(self.slot_hash[16..24].try_into().unwrap());
        let r4 = u64::from_le_bytes(self.slot_hash[24..32].try_into().unwrap());
        Some(r1 ^ r2 ^ r3 ^ r4)
    }
}

account!(LocalUniverseAccount, Drill);
