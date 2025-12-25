use steel::*;
use localuniverse_api::{
    consts::*,
    state::*,
    event::*,
};

/// Swaps SOL for LUXITE, burns 90%, distributes 10% to stakers.
pub fn process_buyback(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let (lu_accounts, swap_accounts) = accounts.split_at(9);

    let [
        signer_info,
        grid_info,
        config_info,
        mint_info,
        treasury_info,
        treasury_luxite_info,
        treasury_sol_info,
        token_program,
        localuniverse_program,
    ] = lu_accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer is buyback authority
    signer_info.is_signer()?.has_address(&BUYBACK_AUTHORITY)?;

    // Validate grid
    grid_info.as_account_mut::<Grid>(&localuniverse_api::ID)?;

    // Validate config
    config_info
        .is_type::<Config>(&localuniverse_api::ID)?
        .has_seeds(&[CONFIG], &localuniverse_api::ID)?;

    // Validate mint
    let luxite_mint = mint_info.has_address(&MINT_ADDRESS)?.as_mint()?;

    // Validate treasury
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    // Validate treasury token accounts
    let treasury_luxite = treasury_luxite_info
        .as_associated_token_account(treasury_info.key, &MINT_ADDRESS)?;

    treasury_sol_info
        .as_associated_token_account(treasury_info.key, &SOL_MINT)?;

    // Validate programs
    token_program.is_program(&spl_token::ID)?;
    localuniverse_program.is_program(&localuniverse_api::ID)?;

    // Sync native token balance
    sync_native(treasury_sol_info)?;

    // Record pre-swap balances
    let treasury_sol = treasury_sol_info
        .as_associated_token_account(treasury_info.key, &SOL_MINT)?;
    let pre_swap_luxite_balance = treasury_luxite.amount();
    let pre_swap_sol_balance = treasury_sol.amount();
    assert!(pre_swap_sol_balance > 0);

    // Record pre-swap mint supply
    let pre_swap_mint_supply = luxite_mint.supply();

    // Record pre-swap treasury lamports
    let pre_swap_treasury_lamports = treasury_info.lamports();

    // Build swap accounts
    let accounts: Vec<AccountMeta> = swap_accounts
        .iter()
        .map(|acc| {
            let is_signer = acc.key == treasury_info.key;
            AccountMeta {
                pubkey: *acc.key,
                is_signer,
                is_writable: acc.is_writable,
            }
        })
        .collect();

    // Build swap account infos
    let accounts_infos: Vec<AccountInfo> = swap_accounts
        .iter()
        .map(|acc| AccountInfo { ..acc.clone() })
        .collect();

    // Invoke swap program
    invoke_signed(
        &Instruction {
            program_id: SWAP_PROGRAM,
            accounts,
            data: data.to_vec(),
        },
        &accounts_infos,
        &localuniverse_api::ID,
        &[TREASURY],
    )?;

    // Verify treasury lamports unchanged
    let post_swap_treasury_lamports = treasury_info.lamports();
    assert_eq!(
        post_swap_treasury_lamports, pre_swap_treasury_lamports,
        "Treasury lamports changed during swap"
    );

    // Verify mint supply unchanged
    let post_swap_mint_supply = mint_info.as_mint()?.supply();
    assert_eq!(
        post_swap_mint_supply, pre_swap_mint_supply,
        "Mint supply changed during swap"
    );

    // Record post-swap balances
    let treasury_luxite = treasury_luxite_info
        .as_associated_token_account(treasury_info.key, &MINT_ADDRESS)?;
    let treasury_sol = treasury_sol_info
        .as_associated_token_account(treasury_info.key, &SOL_MINT)?;
    let post_swap_luxite_balance = treasury_luxite.amount();
    let post_swap_sol_balance = treasury_sol.amount();
    let total_luxite = post_swap_luxite_balance - pre_swap_luxite_balance;

    assert_eq!(post_swap_sol_balance, 0);
    assert!(post_swap_luxite_balance >= pre_swap_luxite_balance);

    // Share 10% with stakers
    let mut shared_amount = 0;
    if treasury.total_staked > 0 {
        shared_amount = total_luxite / 10;
        treasury.stake_rewards_factor += Numeric::from_fraction(shared_amount, treasury.total_staked);
    }

    // Burn 90%
    let burn_amount = total_luxite - shared_amount;
    burn_signed(
        treasury_luxite_info,
        mint_info,
        treasury_info,
        token_program,
        burn_amount,
        &[TREASURY],
    )?;

    // Update treasury
    treasury.total_burned += burn_amount;

    // Emit event
    let clock = Clock::get()?;
    BuybackEvent {
        disc: LocalUniverseEvent::Buyback as u64,
        luxite_burned: burn_amount,
        luxite_shared: shared_amount,
        sol_amount: pre_swap_sol_balance,
        new_circulating_supply: mint_info.as_mint()?.supply(),
        ts: clock.unix_timestamp,
    }
    .log();

    Ok(())
}
