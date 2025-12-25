use serde::{Deserialize, Serialize};
use steel::*;
use crate::state::navigator_pda;
use super::LocalUniverseAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Navigator {
    /// The authority of this navigator account.
    pub authority: Pubkey,

    /// Total dimensions this navigator has discovered.
    pub lifetime_dimensions_discovered: u64,

    /// Lifetime LUXITE earned across all dimensions.
    pub lifetime_rewards_luxite: u64,

    /// Lifetime SOL deployed across all dimensions.
    pub lifetime_deployed: u64,

    /// Unix timestamp when this navigator was created.
    pub created_at: i64,

    /// Buffer a (placeholder).
    pub buffer_a: u64,

    /// Buffer b (placeholder).
    pub buffer_b: u64,

    /// Buffer c (placeholder).
    pub buffer_c: u64,

    /// Buffer d (placeholder).
    pub buffer_d: u64,

    /// Buffer e (placeholder).
    pub buffer_e: u64,
}

impl Navigator {
    pub fn pda(&self) -> (Pubkey, u8) {
        navigator_pda(self.authority)
    }
}

account!(LocalUniverseAccount, Navigator);
