use steel::*;
use solana_program::{log::sol_log, native_token::lamports_to_sol};

use localuniverse_api::{
    consts::*,
    instruction::ReloadSOL,
    state::*,
};

/// Reloads SOL winnings from miner back into automation balance.
pub fn process_reload_sol(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = ReloadSOL::try_from_bytes(data)?;
    let dimension_id = u64::from_le_bytes(args.dimension_id);

    let clock = Clock::get()?;

    let [signer_info, automation_info, miner_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    automation_info
        .is_type::<Automation>(&localuniverse_api::ID)?
        .is_writable()?;

    let automation = automation_info
        .as_account::<Automation>(&localuniverse_api::ID)?
        .assert(|a| a.executor == *signer_info.key)?
        .assert(|a| a.reload > 0)?
        .assert(|a| a.dimension_id == dimension_id)?;

    let authority = automation.authority;

    automation_info.has_seeds(
        &[AUTOMATION, authority.as_ref(), &dimension_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    miner_info
        .is_type::<Miner>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(
            &[MINER, &dimension_id.to_le_bytes(), authority.as_ref()],
            &localuniverse_api::ID,
        )?;

    miner_info
        .as_account::<Miner>(&localuniverse_api::ID)?
        .assert(|m| m.authority == authority)?;

    // Claim SOL from miner
    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
    let amount = miner.claim_sol(&clock);

    if amount == 0 {
        return Ok(());
    }

    // Increment automation balance
    let automation = automation_info.as_account_mut::<Automation>(&localuniverse_api::ID)?;
    automation.balance += amount;

    // Transfer SOL to automation
    miner_info.send(amount, automation_info);

    sol_log(&format!("Reloading {} SOL", lamports_to_sol(amount)));

    Ok(())
}
