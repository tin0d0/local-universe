use steel::*;
use localuniverse_api::{
    consts::*,
    state::*,
};

pub fn process_initialize(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [
        signer_info,
        config_info,
        grid_info,
        treasury_info,
        mint_info,
        treasury_tokens_info,
        system_program,
        token_program,
        associated_token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?.has_address(&ADMIN_ADDRESS)?;

    config_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[CONFIG], &localuniverse_api::ID)?;

    grid_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[GRID], &localuniverse_api::ID)?;

    treasury_info
        .is_empty()?
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    mint_info.has_address(&MINT_ADDRESS)?;

    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    // Create config PDA
    create_program_account::<Config>(
        config_info,
        system_program,
        signer_info,
        &localuniverse_api::ID,
        &[CONFIG],
    )?;

    let config = config_info.as_account_mut::<Config>(&localuniverse_api::ID)?;
    config.admin = *signer_info.key;
    config.fee_collector = ADMIN_FEE_COLLECTOR;
    config.scan_fee = DIMENSION_SCAN_FEE;

    // Create grid PDA
    create_program_account::<Grid>(
        grid_info,
        system_program,
        signer_info,
        &localuniverse_api::ID,
        &[GRID],
    )?;

    let grid = grid_info.as_account_mut::<Grid>(&localuniverse_api::ID)?;
    grid.tick_id = 0;
    grid.start_slot = clock.slot + 1;
    grid.end_slot = grid.start_slot + TICK_DURATION_SLOTS;
    grid.epoch_id = 0;

    // Create treasury PDA
    create_program_account::<Treasury>(
        treasury_info,
        system_program,
        signer_info,
        &localuniverse_api::ID,
        &[TREASURY],
    )?;

    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;
    treasury.sol_balance = 0;
    treasury.luxite_balance = 0;
    treasury.miner_rewards_factor = Numeric::ZERO;
    treasury.stake_rewards_factor = Numeric::ZERO;
    treasury.total_refined = 0;
    treasury.total_staked = 0;
    treasury.total_unclaimed = 0;
    treasury.total_emitted = 0;
    treasury.total_burned = 0;

    // Create treasury token account
    if treasury_tokens_info.data_is_empty() {
        create_associated_token_account(
            signer_info,
            treasury_info,
            treasury_tokens_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    }

    Ok(())
}
