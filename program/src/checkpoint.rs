use steel::*;
use solana_program::log::sol_log;

use localuniverse_api::{
    consts::*,
    state::*,
};

/// Checkpoints a miner's rewards after their excavation is processed.
pub fn process_checkpoint(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [signer_info, grid_info, excavation_info, miner_info, treasury_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    let grid = grid_info
        .has_seeds(&[GRID], &localuniverse_api::ID)?
        .as_account::<Grid>(&localuniverse_api::ID)?;

    miner_info
        .is_type::<Miner>(&localuniverse_api::ID)?
        .is_writable()?;

    let miner = miner_info.as_account::<Miner>(&localuniverse_api::ID)?;
    let dimension_id = miner.dimension_id;
    let miner_excavation_id = miner.excavation_id;

    miner_info.has_seeds(
        &[MINER, &dimension_id.to_le_bytes(), miner.authority.as_ref()],
        &localuniverse_api::ID,
    )?;

    // Already checkpointed
    if miner.checkpoint_id == miner.excavation_id {
        return Ok(());
    }

    excavation_info.has_seeds(
        &[EXCAVATION, &dimension_id.to_le_bytes(), &miner_excavation_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    treasury_info
        .is_type::<Treasury>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    // Excavation account closed - miner forfeits rewards
    if excavation_info.data_is_empty() {
        sol_log("Excavation closed, forfeiting rewards");
        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.checkpoint_id = miner.excavation_id;
        miner.deployed = 0;
        return Ok(());
    }

    excavation_info.is_writable()?;

    let excavation = excavation_info.as_account::<Excavation>(&localuniverse_api::ID)?;

    // Excavation is current tick or not yet processed
    if excavation.id == grid.tick_id || !excavation.is_processed() {
        sol_log("Excavation not yet processed");
        return Ok(());
    }

    // Excavation expired - miner forfeits rewards
    if clock.slot >= excavation.expires_at {
        sol_log("Excavation expired, forfeiting rewards");
        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.checkpoint_id = miner.excavation_id;
        miner.deployed = 0;
        return Ok(());
    }

    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
    let deployed = miner.deployed;

    // Calculate bot fee (if within 12 hours of expiry and signer is not authority)
    let mut bot_fee: u64 = 0;
    let is_bot = *signer_info.key != miner.authority;
    let in_bot_window = clock.slot >= excavation.expires_at.saturating_sub(TWELVE_HOURS_SLOTS);

    if is_bot && in_bot_window && miner.checkpoint_fee > 0 {
        bot_fee = miner.checkpoint_fee;
        miner.checkpoint_fee = 0;
    }

    // Calculate rewards
    let mut rewards_sol: u64 = 0;
    let mut rewards_luxite: u64 = 0;

    if excavation.hit() && deployed > 0 {
        // SOL return (1:1)
        rewards_sol = deployed;

        // LUXITE share (proportional to deployed)
        if excavation.luxite_distributed > 0 && excavation.total_deployed > 0 {
            rewards_luxite = ((excavation.luxite_distributed as u128 * deployed as u128)
                / excavation.total_deployed as u128) as u64;
        }
    }

    // Update claim fee redistribution BEFORE adding new rewards
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;
    miner.update_rewards(treasury);

    // Mark checkpointed
    miner.checkpoint_id = miner.excavation_id;
    miner.deployed = 0;

    // Add rewards
    miner.rewards_sol += rewards_sol;
    miner.rewards_luxite += rewards_luxite;

    // Track unclaimed LUXITE globally
    treasury.total_unclaimed += rewards_luxite;

    // Transfer SOL from excavation to miner
    if rewards_sol > 0 {
        excavation_info.send(rewards_sol, miner_info);
    }

    // Pay bot fee
    if bot_fee > 0 {
        miner_info.send(bot_fee, signer_info);
    }

    Ok(())
}
