use steel::*;
use solana_program::sysvar::slot_hashes;
use localuniverse_api::{
    consts::*,
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

    signer_info.is_signer()?;

    grid_info
        .is_type::<Grid>(&localuniverse_api::ID)?
        .has_seeds(&[GRID], &localuniverse_api::ID)?;

    let grid = grid_info.as_account::<Grid>(&localuniverse_api::ID)?;

    dimension_info
        .is_type::<Dimension>(&localuniverse_api::ID)?;

    let dimension = dimension_info.as_account::<Dimension>(&localuniverse_api::ID)?;
    let dimension_id = dimension.id;
    let richness = dimension.richness;

    dimension_info.has_seeds(
        &[DIMENSION, &dimension_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

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

    // Store deployed amount before mutating
    let sol_deployed = drill.total_deployed;

    treasury_info
        .is_type::<Treasury>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(&[TREASURY], &localuniverse_api::ID)?;

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

    // Apply damper if below minimum SOL threshold
    // Effective richness increases (harder to hit) when underfunded
    let effective_richness = if sol_deployed >= MIN_DEPLOYED_FOR_FULL_RATE {
        richness as u64
    } else {
        // Scale richness up proportionally (worse odds)
        // At 0.5 SOL: richness doubled (half the hit chance)
        // At 0.1 SOL: richness 10x (1/10th hit chance)
        let scale = MIN_DEPLOYED_FOR_FULL_RATE
            .checked_div(sol_deployed)
            .unwrap_or(10);
        let scaled = (richness as u64).saturating_mul(scale);
        scaled.min(999_999_999) // Cap at 99.99% miss rate
    };

    // Determine if drill hit based on effective richness
    let roll = rng % 1_000_000_000;
    let did_hit = roll > effective_richness;

    let mut luxite_distributed: u64 = 0;

    // Update treasury
    let treasury = treasury_info.as_account_mut::<Treasury>(&localuniverse_api::ID)?;

    if did_hit {
        let tick_emission = treasury.luxite_balance
            .checked_mul(TICK_EMISSION_BPS)
            .unwrap_or(0)
            .checked_div(10_000_000)
            .unwrap_or(0);

        if tick_emission > 0 && treasury.luxite_balance >= tick_emission {
            luxite_distributed = tick_emission;
            treasury.luxite_balance -= tick_emission;
            treasury.total_emitted += tick_emission;
            treasury.total_unclaimed += tick_emission;

            drill.rewards_factor += Numeric::from_fraction(tick_emission, sol_deployed);
            drill.total_unclaimed += tick_emission;
            drill.lifetime_rewards_luxite += tick_emission;
        }
    }

    // Transfer SOL from drill to treasury
    drill_info.send(sol_deployed, treasury_info);
    treasury.sol_balance += sol_deployed;

    // Reset drill deployed for next tick
    drill.total_deployed = 0;
    drill.miner_count = 0;

    ExcavateEvent {
        disc: LocalUniverseEvent::Excavate as u64,
        dimension_id,
        tick_id: previous_tick,
        richness: richness as u64,
        luxite_distributed,
        total_deployed: sol_deployed,
        miner_count: drill.miner_count,
        depth: drill.depth,
        ts: clock.unix_timestamp,
    }
    .log();

    Ok(())
}
