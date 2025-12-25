use steel::*;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
};

/// Compounds yield from the staking contract. Called by bots.
pub fn process_compound_yield(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [
        signer_info,
        mint_info,
        stake_info,
        stake_tokens_info,
        treasury_info,
        treasury_tokens_info,
        system_program,
        token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer
    signer_info.is_signer()?;

    // Validate mint
    mint_info.has_address(&MINT_ADDRESS)?;

    // Validate stake - must have fee reserve and not claimed recently
    let stake = stake_info
        .as_account_mut::<Stake>(&localuniverse_api::ID)?
        .assert_mut(|s| s.compound_fee_reserve >= COMPOUND_FEE_PER_TRANSACTION)?
        .assert_mut(|s| s.last_claim_at + ONE_DAY < clock.unix_timestamp)?;

    // Validate stake tokens
    stake_tokens_info
        .is_writable()?
        .as_associated_token_account(stake_info.key, mint_info.key)?;

    // Validate treasury
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    // Validate treasury tokens
    let treasury_tokens = treasury_tokens_info
        .is_writable()?
        .as_associated_token_account(treasury_info.key, mint_info.key)?;

    // Validate programs
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;

    // Claim all yield
    let amount = stake.claim(u64::MAX, &clock, treasury);

    // Re-deposit into stake
    let amount = stake.deposit(amount, &clock, treasury, &treasury_tokens);

    // Transfer LUXITE from treasury to stake
    transfer_signed(
        treasury_info,
        treasury_tokens_info,
        stake_tokens_info,
        token_program,
        amount,
        &[TREASURY],
    )?;

    // Pay bot fee
    stake.compound_fee_reserve -= COMPOUND_FEE_PER_TRANSACTION;
    stake_info.send(COMPOUND_FEE_PER_TRANSACTION, signer_info);

    Ok(())
}
