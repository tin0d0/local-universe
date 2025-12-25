use steel::*;
use solana_program::{native_token::LAMPORTS_PER_SOL, rent::Rent};
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
};

/// Wraps SOL from the treasury into WSOL for swap transactions.
pub fn process_wrap(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = Wrap::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    let [
        signer_info,
        config_info,
        treasury_info,
        treasury_sol_info,
        system_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer is bury authority
    signer_info.is_signer()?.has_address(&BUYBACK_AUTHORITY)?;

    // Validate config
    config_info
        .is_type::<Config>(&localuniverse_api::ID)?
        .has_seeds(&[CONFIG], &localuniverse_api::ID)?;

    // Validate treasury
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    // Validate treasury WSOL account
    treasury_sol_info
        .is_writable()?
        .as_associated_token_account(treasury_info.key, &SOL_MINT)?;

    // Validate programs
    system_program.is_program(&system_program::ID)?;

    // Cap amount at 100 SOL per wrap, treasury balance, or requested amount
    let amount = (LAMPORTS_PER_SOL * 100)
        .min(treasury.sol_balance)
        .min(amount);

    // Send SOL to the WSOL account
    treasury_info.send(amount, treasury_sol_info);

    // Ensure treasury keeps minimum rent balance
    let min_balance = Rent::get()?.minimum_balance(std::mem::size_of::<Treasury>());
    assert!(
        treasury_info.lamports() >= min_balance,
        "Insufficient SOL balance"
    );

    // Update treasury
    treasury.sol_balance -= amount;

    Ok(())
}
