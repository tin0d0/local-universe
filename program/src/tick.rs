use steel::*;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
    event::*,
};

/// Advances the global tick. Called once when tick ends.
pub fn process_tick(accounts: &[AccountInfo], _data: &[u8]) -> ProgramResult {
    let clock = Clock::get()?;

    let [
        signer_info,
        grid_info,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    // Validate signer
    signer_info.is_signer()?;

    // Validate grid and check tick has ended
    grid_info
        .is_type::<Grid>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(&[GRID], &localuniverse_api::ID)?;

    let grid = grid_info
        .as_account_mut::<Grid>(&localuniverse_api::ID)?
        .assert_mut(|g| clock.slot >= g.end_slot + INTERMISSION_SLOTS)?;

    // Advance grid to next tick
    grid.tick_id += 1;
    grid.start_slot = clock.slot + 1;
    grid.end_slot = grid.start_slot + TICK_DURATION_SLOTS;

    // Emit tick event
    TickEvent {
        disc: LocalUniverseEvent::Tick as u64,
        tick_id: grid.tick_id,
        start_slot: grid.start_slot,
        end_slot: grid.end_slot,
        epoch_id: grid.epoch_id,
        ts: clock.unix_timestamp,
    }
    .log();

    Ok(())
}
