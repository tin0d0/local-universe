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

    if excavation_info.data_is_empty() {
        sol_log("Excavation closed, forfeiting rewards");
        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.checkpoint_id = miner.excavation_id;
        miner.deployed = 0;
        return Ok(());
    }

    excavation_info.is_writable()?;

    let excavation = excavation_info.as_account::<Excavation>(&localuniverse_api::ID)?;

    // Stale unprocessed excavation - can never be processed, forfeit rewards
    let is_stale = !excavation.is_processed() && excavation.id + 1 < grid.tick_id;
    if is_stale {
        sol_log("Excavation stale (unprocessed), forfeiting rewards");
        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.checkpoint_id = miner.excavation_id;
        miner.deployed = 0;
        return Ok(());
    }

    // Current tick or pending processing - wait
    if excavation.id == grid.tick_id || !excavation.is_processed() {
        sol_log("Excavation not yet processed");
        return Ok(());
    }

    if clock.slot >= excavation.expires_at {
        sol_log("Excavation expired, forfeiting rewards");
        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.checkpoint_id = miner.excavation_id;
        miner.deployed = 0;
        return Ok(());
    }

    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
    let deployed = miner.deployed;

    let mut bot_fee: u64 = 0;
    let is_bot = *signer_info.key != miner.authority;
    let in_bot_window = clock.slot >= excavation.expires_at.saturating_sub(TWELVE_HOURS_SLOTS);

    if is_bot && in_bot_window && miner.checkpoint_fee > 0 {
        bot_fee = miner.checkpoint_fee;
        miner.checkpoint_fee = 0;
    }

    let mut rewards_sol: u64 = 0;
    let mut rewards_luxite: u64 = 0;

    if excavation.hit() && deployed > 0 {
        rewards_sol = deployed;

        if excavation.luxite_distributed > 0 && excavation.total_deployed > 0 {
            rewards_luxite = ((excavation.luxite_distributed as u128 * deployed as u128)
                / excavation.total_deployed as u128) as u64;
        }
    }

    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;
    miner.update_rewards(treasury);

    miner.checkpoint_id = miner.excavation_id;
    miner.deployed = 0;

    miner.rewards_sol += rewards_sol;
    miner.rewards_luxite += rewards_luxite;

    treasury.total_unclaimed += rewards_luxite;

    if rewards_sol > 0 {
        excavation_info.send(rewards_sol, miner_info);
    }

    if bot_fee > 0 {
        miner_info.send(bot_fee, signer_info);
    }

    Ok(())
}
