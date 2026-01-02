use steel::*;
use solana_program::{log::sol_log, native_token::lamports_to_sol};

use localuniverse_api::{
    consts::*,
    instruction::Deploy,
    state::*,
};

/// Deploys SOL to a dimension's excavation. Takes 1% fee, rest is at risk.
/// Can be called directly by user, or by executor on behalf of automation.
pub fn process_deploy(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = Deploy::try_from_bytes(data)?;
    let mut amount = u64::from_le_bytes(args.amount);

    let clock = Clock::get()?;

    let [signer_info, authority_info, automation_info, grid_info, dimension_info, drill_info, excavation_info, miner_info, navigator_info, treasury_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    authority_info.is_writable()?;

    let grid = grid_info
        .has_seeds(&[GRID], &localuniverse_api::ID)?
        .as_account::<Grid>(&localuniverse_api::ID)?
        .assert(|g| clock.slot >= g.start_slot && clock.slot < g.end_slot)?;

    let dimension = dimension_info
        .as_account::<Dimension>(&localuniverse_api::ID)?;
    let dimension_id = dimension.id;

    dimension_info.has_seeds(
        &[DIMENSION, &dimension_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    drill_info
        .is_writable()?
        .has_seeds(
            &[DRILL, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    excavation_info
        .is_writable()?
        .has_seeds(
            &[EXCAVATION, &dimension_id.to_le_bytes(), &grid.tick_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    miner_info
        .is_writable()?
        .has_seeds(
            &[MINER, &dimension_id.to_le_bytes(), authority_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    navigator_info
        .is_writable()?
        .has_seeds(
            &[NAVIGATOR, authority_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    treasury_info
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    system_program.is_program(&system_program::ID)?;

    // Check if signer is automation executor
    let automation = if !automation_info.data_is_empty() {
        automation_info
            .is_writable()?
            .has_seeds(
                &[AUTOMATION, authority_info.key.as_ref(), &dimension_id.to_le_bytes()],
                &localuniverse_api::ID,
            )?;

        let automation = automation_info
            .as_account::<Automation>(&localuniverse_api::ID)?
            .assert(|a| a.executor == *signer_info.key)?
            .assert(|a| a.authority == *authority_info.key)?
            .assert(|a| a.dimension_id == dimension_id)?;

        amount = automation.amount;

        Some(automation_info)
    } else {
        assert!(
            *signer_info.key == *authority_info.key,
            "Signer must be authority when no automation"
        );
        None
    };

    // Calculate 1% fee
    let fee = amount
        .checked_mul(DEPLOY_FEE_BPS)
        .unwrap()
        .checked_div(DENOMINATOR_BPS)
        .unwrap();
    let amount_after_fee = amount.checked_sub(fee).unwrap();

    // Create excavation account if first deploy this tick
    if excavation_info.data_is_empty() {
        create_program_account::<Excavation>(
            excavation_info,
            system_program,
            signer_info,
            &localuniverse_api::ID,
            &[EXCAVATION, &dimension_id.to_le_bytes(), &grid.tick_id.to_le_bytes()],
        )?;

        let excavation = excavation_info.as_account_mut::<Excavation>(&localuniverse_api::ID)?;
        excavation.id = grid.tick_id;
        excavation.dimension_id = dimension_id;
        excavation.slot_hash = [0; 32];
        excavation.expires_at = u64::MAX;
        excavation.rent_payer = *signer_info.key;
        excavation.total_deployed = 0;
        excavation.total_miners = 0;
        excavation.did_hit = 0;
        excavation.luxite_distributed = 0;
        excavation.buffer_a = 0;
        excavation.buffer_b = 0;
        excavation.buffer_c = 0;
        excavation.buffer_d = 0;
    }

    // Create miner account if new
    if miner_info.data_is_empty() {
        create_program_account::<Miner>(
            miner_info,
            system_program,
            signer_info,
            &localuniverse_api::ID,
            &[MINER, &dimension_id.to_le_bytes(), authority_info.key.as_ref()],
        )?;

        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.authority = *authority_info.key;
        miner.dimension_id = dimension_id;
        miner.deployed = 0;
        miner.checkpoint_fee = 0;
        miner.checkpoint_id = 0;
        miner.excavation_id = 0;
        miner.rewards_sol = 0;
        miner.rewards_luxite = 0;
        miner.refined_luxite = 0;
        miner.rewards_factor = Numeric::ZERO;
        miner.last_claim_luxite_at = clock.unix_timestamp;
        miner.last_claim_sol_at = clock.unix_timestamp;
        miner.lifetime_rewards_sol = 0;
        miner.lifetime_rewards_luxite = 0;
        miner.lifetime_deployed = 0;
        miner.buffer_a = 0;
        miner.buffer_b = 0;
        miner.buffer_c = 0;
        miner.buffer_d = 0;
    }

    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;

    // Verify miner authority
    if automation.is_some() {
        assert!(miner.authority == *authority_info.key, "Miner authority mismatch");
    } else {
        assert!(miner.authority == *signer_info.key, "Not authorized");
    }

    // Handle excavation transition
    if miner.excavation_id != grid.tick_id {
        // Require checkpoint before moving to new excavation (skip if first ever deploy)
        assert!(
            miner.checkpoint_id == miner.excavation_id || miner.excavation_id == 0,
            "Must checkpoint before deploying to new excavation"
        );

        miner.deployed = 0;
        miner.excavation_id = grid.tick_id;
    }

    // Track if this is miner's first deploy this excavation (for miner count)
    let is_first_deploy = miner.deployed == 0;

    // Update miner totals
    miner.deployed += amount_after_fee;
    miner.lifetime_deployed += amount_after_fee;

    // Top up checkpoint fee if empty
    if miner.checkpoint_fee == 0 {
        miner.checkpoint_fee = CHECKPOINT_FEE;
        miner_info.collect(CHECKPOINT_FEE, signer_info)?;
    }

    // Update excavation
    let excavation = excavation_info.as_account_mut::<Excavation>(&localuniverse_api::ID)?;

    if is_first_deploy {
        excavation.total_miners += 1;
    }

    excavation.total_deployed += amount_after_fee;

    // Update drill
    let drill = drill_info.as_account_mut::<Drill>(&localuniverse_api::ID)?;
    drill.lifetime_deployed += amount_after_fee;

    // Update navigator
    let navigator = navigator_info.as_account_mut::<Navigator>(&localuniverse_api::ID)?;
    navigator.lifetime_deployed += amount_after_fee;

    // Update treasury with fee
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;
    treasury.sol_balance += fee;

    // Transfer SOL
    if let Some(auto_info) = automation {
        let automation = auto_info.as_account_mut::<Automation>(&localuniverse_api::ID)?;
        let automation_fee = automation.fee;

        let total_needed = amount + automation_fee;
        assert!(
            automation.balance >= total_needed,
            "Insufficient automation balance"
        );

        automation.balance -= total_needed;

        auto_info.send(fee, treasury_info);
        auto_info.send(amount_after_fee, excavation_info);
        auto_info.send(automation_fee, signer_info);

        // Close automation if balance too low for another deploy
        if automation.balance < automation.amount + automation.fee {
            auto_info.close(authority_info)?;
        }

        sol_log(&format!(
            "Automation deployed {} SOL to dimension {} (executor fee: {} SOL)",
            lamports_to_sol(amount),
            dimension_id,
            lamports_to_sol(automation_fee)
        ));
    } else {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(signer_info.key, treasury_info.key, fee),
            &[signer_info.clone(), treasury_info.clone()],
        )?;

        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                signer_info.key,
                excavation_info.key,
                amount_after_fee,
            ),
            &[signer_info.clone(), excavation_info.clone()],
        )?;

        sol_log(&format!(
            "Deployed {} SOL to dimension {}",
            lamports_to_sol(amount),
            dimension_id
        ));
    }

    Ok(())
}
