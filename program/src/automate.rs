use steel::*;

use localuniverse_api::{
    consts::*,
    instruction::Automate,
    state::*,
};

/// Sets up or updates automation for a dimension. Pass executor = Pubkey::default() to close.
pub fn process_automate(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = Automate::try_from_bytes(data)?;
    let dimension_id = u64::from_le_bytes(args.dimension_id);
    let amount = u64::from_le_bytes(args.amount);
    let deposit = u64::from_le_bytes(args.deposit);
    let fee = u64::from_le_bytes(args.fee);
    let reload = u64::from_le_bytes(args.reload);

    let clock = Clock::get()?;

    let [signer_info, automation_info, executor_info, miner_info, dimension_info, system_program] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    automation_info
        .is_writable()?
        .has_seeds(
            &[AUTOMATION, signer_info.key.as_ref(), &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    dimension_info
        .is_type::<Dimension>(&localuniverse_api::ID)?
        .has_seeds(
            &[DIMENSION, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    miner_info
        .is_writable()?
        .has_seeds(
            &[MINER, &dimension_id.to_le_bytes(), signer_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    system_program.is_program(&system_program::ID)?;

    // Create miner if needed
    if miner_info.data_is_empty() {
        create_program_account::<Miner>(
            miner_info,
            system_program,
            signer_info,
            &localuniverse_api::ID,
            &[MINER, &dimension_id.to_le_bytes(), signer_info.key.as_ref()],
        )?;

        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.authority = *signer_info.key;
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
    } else {
        miner_info
            .as_account::<Miner>(&localuniverse_api::ID)?
            .assert(|m| m.authority == *signer_info.key)?;
    }

    // Top up checkpoint fee if needed
    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
    if miner.checkpoint_fee == 0 {
        miner.checkpoint_fee = CHECKPOINT_FEE;
        miner_info.collect(CHECKPOINT_FEE, signer_info)?;
    }

    // Close automation if executor is default pubkey
    if *executor_info.key == Pubkey::default() {
        if !automation_info.data_is_empty() {
            automation_info
                .as_account::<Automation>(&localuniverse_api::ID)?
                .assert(|a| a.authority == *signer_info.key)?;

            automation_info.close(signer_info)?;
        }
        return Ok(());
    }

    // Create automation if needed
    if automation_info.data_is_empty() {
        create_program_account::<Automation>(
            automation_info,
            system_program,
            signer_info,
            &localuniverse_api::ID,
            &[AUTOMATION, signer_info.key.as_ref(), &dimension_id.to_le_bytes()],
        )?;

        let automation = automation_info.as_account_mut::<Automation>(&localuniverse_api::ID)?;
        automation.authority = *signer_info.key;
        automation.dimension_id = dimension_id;
        automation.balance = 0;
    } else {
        automation_info
            .as_account::<Automation>(&localuniverse_api::ID)?
            .assert(|a| a.authority == *signer_info.key)?;
    }

    // Update automation settings
    let automation = automation_info.as_account_mut::<Automation>(&localuniverse_api::ID)?;
    automation.amount = amount;
    automation.executor = *executor_info.key;
    automation.fee = fee;
    automation.reload = reload;

    // Deposit SOL
    if deposit > 0 {
        automation.balance += deposit;
        automation_info.collect(deposit, signer_info)?;
    }

    Ok(())
}
