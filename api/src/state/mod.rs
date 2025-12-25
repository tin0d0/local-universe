mod config;
mod dimension;
mod drill;
mod grid;
mod miner;
mod navigator;
mod stake;
mod treasury;

pub use config::*;
pub use dimension::*;
pub use drill::*;
pub use grid::*;
pub use miner::*;
pub use navigator::*;
pub use stake::*;
pub use treasury::*;

use crate::consts::*;
use steel::*;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
pub enum LocalUniverseAccount {
    Config = 100,
    Grid = 101,
    Dimension = 102,
    Drill = 103,
    Navigator = 104,
    Miner = 105,
    Treasury = 106,
    Stake = 107,
}

pub fn config_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[CONFIG], &crate::ID)
}

pub fn grid_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[GRID], &crate::ID)
}

pub fn dimension_pda(id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DIMENSION, &id.to_le_bytes()], &crate::ID)
}

pub fn drill_pda(dimension_id: u64) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[DRILL, &dimension_id.to_le_bytes()], &crate::ID)
}

pub fn navigator_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[NAVIGATOR, &authority.to_bytes()], &crate::ID)
}

pub fn miner_pda(dimension_id: u64, authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[MINER, &dimension_id.to_le_bytes(), &authority.to_bytes()], &crate::ID)
}

pub fn treasury_pda() -> (Pubkey, u8) {
    Pubkey::find_program_address(&[TREASURY], &crate::ID)
}

pub fn stake_pda(authority: Pubkey) -> (Pubkey, u8) {
    Pubkey::find_program_address(&[STAKE, &authority.to_bytes()], &crate::ID)
}
