use steel::*;
use localuniverse_api::{
    consts::*,
    state::*,
    event::*,
};

pub fn process_deploy(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = localuniverse_api::instruction::Deploy::try_from_bytes(data)?;
    let amount = u64::from_le_bytes(args.amount);

    let [
        signer_info,
        grid_info,
        drill_info,
        miner_info,
        navigator_info,
        system_program,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    let clock = Clock::get()?;

    signer_info.is_signer()?;

    grid_info
        .is_type::<Grid>(&localuniverse_api::ID)?
        .has_seeds(&[GRID], &localuniverse_api::ID)?;

    let grid = grid_info
        .as_account::<Grid>(&localuniverse_api::ID)?
        .assert(|g| clock.slot >= g.start_slot && clock.slot < g.end_slot)?;

    drill_info
        .is_type::<Drill>(&localuniverse_api::ID)?
        .is_writable()?;

    let drill = drill_info.as_account::<Drill>(&localuniverse_api::ID)?;
    let dimension_id = drill.dimension_id;

    drill_info.has_seeds(
        &[DRILL, &dimension_id.to_le_bytes()],
        &localuniverse_api::ID,
    )?;

    miner_info
        .is_writable()?
        .has_seeds(
            &[MINER, &dimension_id.to_le_bytes(), signer_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    navigator_info
        .is_type::<Navigator>(&localuniverse_api::ID)?
        .is_writable()?
        .has_seeds(
            &[NAVIGATOR, signer_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    let is_new_miner = miner_info.data_is_empty();

    if is_new_miner {
        create_account(
            miner_info,
            system_program,
            signer_info,
            std::mem::size_of::<Miner>() + 8,
            &localuniverse_api::ID,
        )?;

        let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;
        miner.authority = *signer_info.key;
        miner.dimension_id = dimension_id;
        miner.deployed = 0;
        miner.rewards_luxite = 0;
        miner.tick_id = grid.tick_id;
        miner.lifetime_deployed = 0;
        miner.last_claim_luxite_at = clock.unix_timestamp;
        miner.lifetime_rewards_luxite = 0;
    }

    let miner = miner_info.as_account_mut::<Miner>(&localuniverse_api::ID)?;

    assert!(miner.authority == *signer_info.key, "Not authorized");

    if miner.tick_id != grid.tick_id {
        miner.deployed = 0;
        miner.tick_id = grid.tick_id;
    }

    let is_first_deploy_this_tick = miner.deployed == 0;

    miner.deployed += amount;
    miner.lifetime_deployed += amount;

    let drill = drill_info.as_account_mut::<Drill>(&localuniverse_api::ID)?;

    if drill.tick_id != grid.tick_id {
        drill.total_deployed = 0;
        drill.miner_count = 0;
        drill.tick_id = grid.tick_id;
    }

    if is_first_deploy_this_tick {
        drill.miner_count += 1;
    }

    drill.total_deployed += amount;
    drill.lifetime_deployed += amount;

    let navigator = navigator_info.as_account_mut::<Navigator>(&localuniverse_api::ID)?;
    navigator.lifetime_deployed += amount;

    solana_program::program::invoke(
        &solana_program::system_instruction::transfer(
            signer_info.key,
            drill_info.key,
            amount,
        ),
        &[signer_info.clone(), drill_info.clone()],
    )?;

    DeployEvent {
        disc: LocalUniverseEvent::Deploy as u64,
        dimension_id,
        authority: *signer_info.key,
        signer: *signer_info.key,
        amount,
        tick_id: grid.tick_id,
        ts: clock.unix_timestamp,
    }
    .log();

    Ok(())
}
