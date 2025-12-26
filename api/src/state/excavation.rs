use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::excavation_pda;
use super::LocalUniverseAccount;

/// Per-tick mining state for a dimension. Closeable after expiry to reclaim rent.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Excavation {
    /// The tick number this excavation is for.
    pub id: u64,

    /// The dimension this excavation is on.
    pub dimension_id: u64,

    /// The hash of the end slot, used for RNG.
    pub slot_hash: [u8; 32],

    /// The slot at which claims for this excavation expire.
    pub expires_at: u64,

    /// The account to receive rent when this account is closed.
    pub rent_payer: Pubkey,

    /// The total amount of SOL deployed in this excavation (after fees).
    pub total_deployed: u64,

    /// The total number of unique miners in this excavation.
    pub total_miners: u64,

    /// Whether this excavation hit (1) or missed (0).
    pub did_hit: u64,

    /// The amount of LUXITE distributed this excavation.
    pub luxite_distributed: u64,

    /// Reserved for future use.
    pub buffer_a: u64,

    /// Reserved for future use.
    pub buffer_b: u64,

    /// Reserved for future use.
    pub buffer_c: u64,

    /// Reserved for future use.
    pub buffer_d: u64,
}

impl Excavation {
    pub fn pda(&self) -> (Pubkey, u8) {
        excavation_pda(self.dimension_id, self.id)
    }

    /// Generates RNG from slot hash. Returns None if hash is invalid.
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

    /// Returns true if this excavation was a hit.
    pub fn hit(&self) -> bool {
        self.did_hit == 1
    }

    /// Returns true if this excavation has been processed.
    pub fn is_processed(&self) -> bool {
        self.slot_hash != [0; 32]
    }
}

account!(LocalUniverseAccount, Excavation);
