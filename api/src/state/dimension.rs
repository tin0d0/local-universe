use serde::{Deserialize, Serialize};
use steel::*;
use crate::state::dimension_pda;
use super::LocalUniverseAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Dimension {
    /// The authority of this dimension account.
    pub authority: Pubkey,

    /// Wallet that originally discovered this dimension.
    pub discoverer: Pubkey,

    /// The dimension ID.
    pub id: u64,

    /// Unix timestamp when scanned.
    pub scanned_at: i64,

    /// Richness score (9 decimals, lower = better hit rate).
    pub richness: u32,

    /// Explicit padding for alignment.
    pub _padding: u32,

    /// Buffer a (placeholder).
    pub buffer_a: u64,

    /// Buffer b (placeholder).
    pub buffer_b: u64,

    /// Buffer c (placeholder).
    pub buffer_c: u64,

    /// Buffer d (placeholder).
    pub buffer_d: u64,
}

impl Dimension {
    pub fn pda(&self) -> (Pubkey, u8) {
        dimension_pda(self.id)
    }
}

account!(LocalUniverseAccount, Dimension);
