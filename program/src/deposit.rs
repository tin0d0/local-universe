use steel::*;
use localuniverse_api::{
    consts::*,
    state::*,
};

pub fn process_deposit(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = localuniverse_api::instruction::Deposit::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);
    let compound_fee = u64::from_le_bytes(args.compound_fee);
    let clock = Clock::get()?;

    let [
        signer_info,
        payer_info,
        mint_info,
        sender_info,
        stake_info,
        stake_tokens_info,
        treasury_info,
        system_program,
        token_program,
        associated_token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;
    payer_info.is_signer()?;

    mint_info.has_address(&MINT_ADDRESS)?;

    let sender = sender_info
        .is_writable()?
        .as_associated_token_account(signer_info.key, &MINT_ADDRESS)?;

    stake_info
        .is_writable()?
        .has_seeds(
            &[STAKE, signer_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    let treasury = treasury_info
        .as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    system_program.is_program(&system_program::ID)?;
    token_program.is_program(&spl_token::ID)?;
    associated_token_program.is_program(&spl_associated_token_account::ID)?;

    let stake = if stake_info.data_is_empty() {
        create_account(
            stake_info,
            system_program,
            payer_info,
            std::mem::size_of::<Stake>() + 8,
            &localuniverse_api::ID,
        )?;

        let stake = stake_info.as_account_mut::<Stake>(&localuniverse_api::ID)?;
        stake.authority = *signer_info.key;
        stake.balance = 0;
        stake.rewards_factor = treasury.stake_rewards_factor;
        stake.rewards = 0;
        stake.lifetime_rewards = 0;
        stake.compound_fee_reserve = 0;
        stake.last_claim_at = 0;
        stake.last_deposit_at = 0;
        stake.last_withdraw_at = 0;
        stake
    } else {
        stake_info
            .as_account_mut::<Stake>(&localuniverse_api::ID)?
            .assert_mut(|s| s.authority == *signer_info.key)?
    };

    if stake_tokens_info.data_is_empty() {
        create_associated_token_account(
            signer_info,
            stake_info,
            stake_tokens_info,
            mint_info,
            system_program,
            token_program,
            associated_token_program,
        )?;
    } else {
        stake_tokens_info.as_associated_token_account(stake_info.key, mint_info.key)?;
    }

    let amount = stake.deposit(amount, &clock, treasury, &sender);

    transfer(
        signer_info,
        sender_info,
        stake_tokens_info,
        token_program,
        amount,
    )?;

    if compound_fee > 0 {
        stake.compound_fee_reserve += compound_fee;
        stake_info.collect(compound_fee, signer_info)?;
    }

    let stake_tokens = stake_tokens_info.as_associated_token_account(stake_info.key, mint_info.key)?;
    assert!(stake_tokens.amount() >= stake.balance);

    Ok(())
}
