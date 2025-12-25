use serde::{Deserialize, Serialize};
use steel::*;
use crate::state::config_pda;
use super::LocalUniverseAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Config {
    /// The address that can update the config.
    pub admin: Pubkey,

    /// The address that receives scan fees.
    pub fee_collector: Pubkey,

    /// The fee for scanning a new dimension (in lamports).
    pub scan_fee: u64,

    /// Buffer a (placeholder)
    pub buffer_a: u64,

    /// Buffer b (placeholder)
    pub buffer_b: u64,

    /// Buffer c (placeholder)
    pub buffer_c: u64,

    /// Buffer d (placeholder)
    pub buffer_d: u64,
}

impl Config {
    pub fn pda() -> (Pubkey, u8) {
        config_pda()
    }
}

account!(LocalUniverseAccount, Config);
