use steel::*;
use localuniverse_api::{
    consts::*,
    state::*,
};

pub fn process_claim_luxite(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [
        signer_info,
        miner_info,
        navigator_info,
        drill_info,
        mint_info,
        recipient_info,
        treasury_info,
        treasury_tokens_info,
        system_program,
        token_program,
        associated_token_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    miner_info
        .is_type::<Miner>(&localuniverse_api::ID)?
        .is_writable()?;

    let miner = miner_info.as_account::<Miner>(&localuniverse_api::ID)?;
    let dimension_id = miner.dimension_id;

    miner_info.has_seeds(
        &[MINER, &dimension_id.to_le_bytes(), signer_info.key.as_ref()],
        &localuniverse_api::ID,
    )?;

    assert!(miner.authority == *signer_info.key, "Not authorized");

    navigator_info
        .is_type::<Navigator>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(
            &[NAVIGATOR, signer_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    drill_info
        .is_type::<Drill>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(
            &[DRILL, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    mint_info.has_address(&MINT_ADDRESS)?;

    treasury_info
        .is_type::<Treasury>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    treasury_tokens_info.as_associated_token_account(treasury_info.key, mint_info.key)?;

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

    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
    let drill = drill_info.as_account_mut::<Drill>(&localuniverse_api::ID)?;
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    let amount = miner.claim_luxite(&clock, drill, treasury);

    if amount == 0 {
        return Ok(());
    }

    let navigator = navigator_info.as_account_mut::<Navigator>(&localuniverse_api::ID)?;
    navigator.lifetime_rewards_luxite += amount;

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
