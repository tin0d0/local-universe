use steel::*;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
};

/// Claims yield from the staking contract.
pub fn process_claim_yield(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = ClaimYield::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    let clock = Clock::get()?;

    let [
        signer_info,
        mint_info,
        recipient_info,
        stake_info,
        treasury_info,
        treasury_tokens_info,
        system_program,
        token_program,
        associated_token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer
    signer_info.is_signer()?;

    // Validate mint
    mint_info.has_address(&MINT_ADDRESS)?;

    // Validate recipient
    recipient_info.is_writable()?;

    // Validate stake
    let stake = stake_info
        .as_account_mut::<Stake>(&localuniverse_api::ID)?
        .assert_mut(|s| s.authority == *signer_info.key)?;

    // Validate treasury
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    // Validate treasury tokens
    treasury_tokens_info
        .is_writable()?
        .as_associated_token_account(treasury_info.key, mint_info.key)?;

    // Validate programs
    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Create recipient token account if needed
    if recipient_info.data_is_empty() {
        create_associated_token_account(
            signer_info,
            signer_info,
            recipient_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    } else {
        recipient_info.as_associated_token_account(signer_info.key, mint_info.key)?;
    }

    // Claim yield from stake account
    let amount = stake.claim(amount, &clock, treasury);

    // Transfer LUXITE to recipient
    transfer_signed(
        treasury_info,
        treasury_tokens_info,
        recipient_info,
        token_program,
        amount,
        &[TREASURY],
    )?;

    Ok(())
}
