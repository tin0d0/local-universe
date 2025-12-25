use serde::{Deserialize, Serialize};
use steel::*;

use crate::state::grid_pda;

use super::LocalUniverseAccount;

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Pod, Zeroable, Serialize, Deserialize)]
pub struct Grid {
    /// The current tick number.
    pub tick_id: u64,

    /// The slot at which the current round starts mining.
    pub start_slot: u64,

    /// The slot at which the current round ends mining.
    pub end_slot: u64,

    /// The current epoch id.
    pub epoch_id: u64,
}

impl Grid {
    pub fn pda(&self) -> (Pubkey, u8) {
        grid_pda()
    }
}

account!(LocalUniverseAccount, Grid);
