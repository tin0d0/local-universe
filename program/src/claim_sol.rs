use steel::*;

use localuniverse_api::{
    consts::*,
    state::*,
};

/// Claims pending SOL rewards for a miner.
pub fn process_claim_sol(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [signer_info, miner_info, navigator_info] = accounts else {
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

    // Claim rewards
    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
    let amount = miner.claim_sol(&clock);

    if amount == 0 {
        return Ok(());
    }

    // Update navigator lifetime stats
    let navigator = navigator_info.as_account_mut::<Navigator>(&localuniverse_api::ID)?;
    navigator.lifetime_rewards_sol += amount;

    // Transfer SOL from miner account to signer
    miner_info.send(amount, signer_info);

    Ok(())
}
