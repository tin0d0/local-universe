use steel::*;
use solana_program::sysvar::slot_hashes;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
    event::*,
};

/// Processes a drill for the current tick. Called per active drill.
pub fn process_excavate(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [
        signer_info,
        grid_info,
        dimension_info,
        drill_info,
        treasury_info,
        slot_hashes_info,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer
    signer_info.is_signer()?;

    // Validate grid
    grid_info
        .is_type::<Grid>(&localuniverse_api::ID)?
        .has_seeds(&[GRID], &localuniverse_api::ID)?;

    let grid = grid_info.as_account::<Grid>(&localuniverse_api::ID)?;

    // Validate dimension
    dimension_info
        .is_type::<Dimension>(&localuniverse_api::ID)?;

    let dimension = dimension_info.as_account::<Dimension>(&localuniverse_api::ID)?;
    let dimension_id = dimension.id;
    let richness = dimension.richness;

    dimension_info.has_seeds(
        &[DIMENSION, &dimension_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    // Validate drill
    drill_info
        .is_type::<Drill>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(
            &[DRILL, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    let drill = drill_info.as_account::<Drill>(&localuniverse_api::ID)?;

    // Check drill hasn't been processed yet and was active
    let previous_tick = grid.tick_id - 1;
    if drill.tick_id != previous_tick || drill.total_deployed == 0 {
        return Ok(());
    }

    // Validate treasury
    treasury_info
        .is_type::<Treasury>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

    // Validate slot hashes
    slot_hashes_info.has_address(&slot_hashes::ID)?;

    // Get slot hash for RNG
    let slot_hashes_data = slot_hashes_info.try_borrow_data()?;
    let hash_start = 8 + 8;
    let hash_bytes: [u8; 32] = slot_hashes_data[hash_start..hash_start + 32]
        .try_into()
        .unwrap();

    // Update drill
    let drill = drill_info.as_account_mut::<Drill>(&localuniverse_api::ID)?;
    drill.slot_hash = hash_bytes;
    drill.tick_id = grid.tick_id;
    drill.depth += 1;

    // Calculate RNG
    let Some(rng) = drill.rng() else {
        return Ok(());
    };

    // Determine if drill hit based on richness
    let roll = rng % 1_000_000_000;
    let did_hit = roll > richness as u64;

    let mut luxite_distributed: u64 = 0;

    if did_hit {
        let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

        let tick_emission = treasury.luxite_balance
            .checked_mul(TICK_EMISSION_BPS)
            .unwrap_or(0)
            .checked_div(10_000_000)
            .unwrap_or(0);

        if tick_emission > 0 && treasury.luxite_balance >= tick_emission {
            treasury.luxite_balance -= tick_emission;
            treasury.total_emitted += tick_emission;

            // Distribute to drill using rewards_factor
            drill.rewards_factor += Numeric::from_fraction(tick_emission, drill.total_deployed);
            drill.total_unclaimed += tick_emission;
            drill.lifetime_rewards_luxite += tick_emission;
        }
    }

    // Emit excavate event
    ExcavateEvent {
        disc: LocalUniverseEvent::Excavate as u64,
        dimension_id,
        tick_id: previous_tick,
        richness: richness as u64,
        luxite_distributed,
        total_deployed: drill.total_deployed,
        miner_count: drill.miner_count,
        depth: drill.depth,
        ts: clock.unix_timestamp,
    }
    .log();

    Ok(())
}
