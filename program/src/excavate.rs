use steel::*;
use solana_program::sysvar::slot_hashes;

use localuniverse_api::{
    consts::*,
    state::*,
};

/// Processes an excavation. Determines hit or miss based on RNG vs richness.
pub fn process_excavate(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [signer_info, grid_info, dimension_info, drill_info, excavation_info, treasury_info, slot_hashes_info] =
        accounts
    else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    let grid = grid_info
        .has_seeds(&[GRID], &localuniverse_api::ID)?
        .as_account::<Grid>(&localuniverse_api::ID)?;

    let dimension = dimension_info
        .as_account::<Dimension>(&localuniverse_api::ID)?;
    let dimension_id = dimension.id;
    let richness = dimension.richness;

    dimension_info.has_seeds(
        &[DIMENSION, &dimension_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    drill_info
        .is_writable()?
        .has_seeds(
            &[DRILL, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    // Process the previous tick's excavation
    let previous_tick_id = grid.tick_id.saturating_sub(1);

    excavation_info
        .is_writable()?
        .has_seeds(
            &[EXCAVATION, &dimension_id.to_le_bytes(), &previous_tick_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    // Return early if excavation doesn't exist
    if excavation_info.data_is_empty() {
        return Ok(());
    }

    let excavation = excavation_info.as_account::<Excavation>(&localuniverse_api::ID)?;

    // Already processed or no activity
    if excavation.slot_hash != [0; 32] || excavation.total_deployed == 0 {
        return Ok(());
    }

    let sol_deployed = excavation.total_deployed;

    treasury_info
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    slot_hashes_info.has_address(&slot_hashes::ID)?;

    // Get slot hash for RNG
    let slot_hashes_data = slot_hashes_info.try_borrow_data()?;
    let hash_start = 8 + 8;
    let hash_bytes: [u8; 32] = slot_hashes_data[hash_start..hash_start + 32]
        .try_into()
        .unwrap();

    // Update excavation
    let excavation = excavation_info.as_account_mut::<Excavation>(&localuniverse_api::ID)?;
    excavation.slot_hash = hash_bytes;
    excavation.expires_at = clock.slot + ONE_DAY_SLOTS;

    let Some(rng) = excavation.rng() else {
        return Ok(());
    };

    // Apply penalty if below minimum SOL threshold
    let effective_richness = if sol_deployed >= MIN_DEPLOYED_FOR_FULL_RATE {
        richness as u64
    } else {
        let scale = MIN_DEPLOYED_FOR_FULL_RATE
            .checked_div(sol_deployed)
            .unwrap_or(10);
        (richness as u64).saturating_mul(scale).min(999_999_999)
    };

    // Roll against richness
    let roll = rng % 1_000_000_000;
    let did_hit = roll > effective_richness;

    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;
    let drill = drill_info.as_account_mut::<Drill>(&localuniverse_api::ID)?;

    if did_hit {
        // === HIT ===
        excavation.did_hit = 1;

        // Calculate LUXITE emission
        let emission = treasury
            .luxite_balance
            .checked_mul(TICK_EMISSION_BPS)
            .unwrap_or(0)
            .checked_div(10_000)
            .unwrap_or(0);

        if emission > 0 && treasury.luxite_balance >= emission {
            excavation.luxite_distributed = emission;
            treasury.luxite_balance -= emission;
            treasury.total_emitted += emission;
            drill.lifetime_rewards_luxite += emission;
        }

        // SOL stays on excavation for checkpoint claims
    } else {
        // === MISS ===
        excavation.did_hit = 0;

        // Send all SOL to treasury
        excavation_info.send(sol_deployed, treasury_info);
        treasury.sol_balance += sol_deployed;
    }

    // Update drill depth
    drill.depth += 1;

    Ok(())
}
