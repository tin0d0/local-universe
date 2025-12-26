use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::automation_pda;
use super::LocalUniverseAccount;

/// Automation settings for hands-free mining on a specific dimension.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Automation {
    /// The amount of SOL to deploy per tick.
    pub amount: u64,

    /// The authority who owns this automation.
    pub authority: Pubkey,

    /// The SOL balance available for automation.
    pub balance: u64,

    /// The dimension this automation is for.
    pub dimension_id: u64,

    /// The executor bot allowed to run this automation.
    pub executor: Pubkey,

    /// The fee paid to executor per operation (in lamports).
    pub fee: u64,

    /// Reserved for future strategy use.
    pub strategy: u64,

    /// Whether to auto-reload SOL winnings into balance (1 = yes, 0 = no).
    pub reload: u64,
}

impl Automation {
    pub fn pda(&self) -> (Pubkey, u8) {
        automation_pda(self.authority, self.dimension_id)
    }
}

account!(LocalUniverseAccount, Automation);
