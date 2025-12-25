use steel::*;
use localuniverse_api::{
    consts::*,
    state::*,
};

/// Funds the treasury with LUXITE (admin only).
pub fn process_fund_treasury(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = localuniverse_api::instruction::FundTreasury::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    let [
        signer_info,
        mint_info,
        sender_info,
        treasury_info,
        treasury_tokens_info,
        token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate admin
    signer_info.is_signer()?.has_address(&ADMIN_ADDRESS)?;

    // Validate mint
    mint_info.has_address(&MINT_ADDRESS)?;

    // Validate sender token account
    sender_info
        .is_writable()?
        .as_associated_token_account(signer_info.key, &MINT_ADDRESS)?;

    // Validate treasury
    let treasury = treasury_info
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?
        .as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    // Validate treasury tokens
    treasury_tokens_info
        .is_writable()?
        .as_associated_token_account(treasury_info.key, &MINT_ADDRESS)?;

    // Validate token program
    token_program.is_program(&spl_token::ID)?;

    // Transfer LUXITE to treasury
    transfer(
        signer_info,
        sender_info,
        treasury_tokens_info,
        token_program,
        amount,
    )?;

    // Update treasury balance
    treasury.luxite_balance += amount;

    Ok(())
}
