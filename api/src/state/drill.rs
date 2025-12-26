use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::drill_pda;
use super::LocalUniverseAccount;

/// Global drill state for a dimension. Tracks lifetime mining stats only.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Drill {
    /// The dimension this drill is on.
    pub dimension_id: u64,

    /// Current mining depth (number of excavations processed).
    pub depth: u64,

    /// Lifetime SOL deployed across all excavations.
    pub lifetime_deployed: u64,

    /// Lifetime LUXITE earned across all excavations.
    pub lifetime_rewards_luxite: u64,

    /// Reserved for future use.
    pub buffer_a: u64,

    /// Reserved for future use.
    pub buffer_b: u64,

    /// Reserved for future use.
    pub buffer_c: u64,

    /// Reserved for future use.
    pub buffer_d: u64,
}

impl Drill {
    pub fn pda(&self) -> (Pubkey, u8) {
        drill_pda(self.dimension_id)
    }
}

account!(LocalUniverseAccount, Drill);
