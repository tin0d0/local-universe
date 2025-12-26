use steel::*;
use solana_program::rent::Rent;

use localuniverse_api::{
    consts::*,
    state::*,
};

/// Closes an expired excavation account. Returns rent to payer, unclaimed SOL to treasury.
pub fn process_close(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [signer_info, grid_info, excavation_info, rent_payer_info, treasury_info] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    let grid = grid_info
        .has_seeds(&[GRID], &localuniverse_api::ID)?
        .as_account::<Grid>(&localuniverse_api::ID)?;

    excavation_info
        .is_type::<Excavation>(&localuniverse_api::ID)?
        .is_writable()?;

    let excavation = excavation_info.as_account::<Excavation>(&localuniverse_api::ID)?;

    // Must be past tick and expired
    assert!(excavation.id < grid.tick_id, "Excavation is current tick");
    assert!(clock.slot >= excavation.expires_at, "Excavation not expired");
    assert!(excavation.rent_payer == *rent_payer_info.key, "Wrong rent payer");

    let dimension_id = excavation.dimension_id;
    let excavation_id = excavation.id;

    excavation_info.has_seeds(
        &[EXCAVATION, &dimension_id.to_le_bytes(), &excavation_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    rent_payer_info.is_writable()?;

    treasury_info
        .is_type::<Treasury>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    // Calculate unclaimed SOL (anything above rent minimum)
    let size = 8 + std::mem::size_of::<Excavation>();
    let min_rent = Rent::get()?.minimum_balance(size);
    let unclaimed_sol = excavation_info.lamports().saturating_sub(min_rent);

    // Send unclaimed SOL to treasury
    if unclaimed_sol > 0 {
        excavation_info.send(unclaimed_sol, treasury_info);
        let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;
        treasury.sol_balance += unclaimed_sol;
    }

    // Close excavation account, return rent to payer
    excavation_info.close(rent_payer_info)?;

    Ok(())
}
