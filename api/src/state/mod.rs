mod automation;
mod config;
mod dimension;
mod drill;
mod excavation;
mod grid;
mod miner;
mod navigator;
mod stake;
mod treasury;

pub use automation::*;
pub use config::*;
pub use dimension::*;
pub use drill::*;
pub use excavation::*;
pub use grid::*;
pub use miner::*;
pub use navigator::*;
pub use stake::*;
pub use treasury::*;

use steel::*;

use crate::consts::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum LocalUniverseAccount {
    Automation = 99,
    Config = 100,
    Dimension = 101,
    Drill = 102,
    Excavation = 103,
    Grid = 104,
    Miner = 105,
    Navigator = 106,
    Stake = 107,
    Treasury = 108,
}

/// PDA for automation (per authority per dimension).
pub fn automation_pda(authority: Pubkey, dimension_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[AUTOMATION, authority.as_ref(), &dimension_id.to_le_bytes()],
        &crate::ID,
    )
}

/// PDA for global config.
pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::ID)
}

/// PDA for a dimension.
pub fn dimension_pda(id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DIMENSION, &id.to_le_bytes()], &crate::ID)
}

/// PDA for a drill (global per dimension).
pub fn drill_pda(dimension_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DRILL, &dimension_id.to_le_bytes()], &crate::ID)
}

/// PDA for an excavation (per dimension per tick, closeable).
pub fn excavation_pda(dimension_id: u64, tick_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[EXCAVATION, &dimension_id.to_le_bytes(), &tick_id.to_le_bytes()],
        &crate::ID,
    )
}

/// PDA for the global grid.
pub fn grid_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[GRID], &crate::ID)
}

/// PDA for a miner (per dimension per authority).
pub fn miner_pda(dimension_id: u64, authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(
        &[MINER, &dimension_id.to_le_bytes(), authority.as_ref()],
        &crate::ID,
    )
}

/// PDA for a navigator (per authority).
pub fn navigator_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[NAVIGATOR, authority.as_ref()], &crate::ID)
}

/// PDA for a stake account.
pub fn stake_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[STAKE, authority.as_ref()], &crate::ID)
}

/// PDA for the treasury.
pub fn treasury_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[TREASURY], &crate::ID)
}
