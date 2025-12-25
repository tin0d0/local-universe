use steel::*;
use solana_program::sysvar::slot_hashes;
use localuniverse_api::{
    consts::*,
    instruction::*,
    state::*,
    event::*,
};

pub fn process_scan(accounts: &[AccountInfo], data: &[u8]) -> ProgramResult {
    let args = Scan::try_from_bytes(data)?;
    let dimension_id = u64::from_le_bytes(args.dimension_id);

    let [
        signer_info,
        config_info,
        dimension_info,
        drill_info,
        navigator_info,
        fee_collector_info,
        system_program,
        slot_hashes_info,
    ] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
    };

    signer_info.is_signer()?;

    config_info
        .is_type::<Config>(&localuniverse_api::ID)?
        .has_seeds(&[CONFIG], &localuniverse_api::ID)?;

    let config = config_info.as_account::<Config>(&localuniverse_api::ID)?;

    fee_collector_info.has_address(&config.fee_collector)?;
    slot_hashes_info.has_address(&slot_hashes::ID)?;

    dimension_info
        .is_empty()?
        .is_writable()?
        .has_seeds(
            &[DIMENSION, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    drill_info
        .is_empty()?
        .is_writable()?
        .has_seeds(
            &[DRILL, &dimension_id.to_le_bytes()],
            &localuniverse_api::ID,
        )?;

    navigator_info
        .is_writable()?
        .has_seeds(
            &[NAVIGATOR, signer_info.key.as_ref()],
            &localuniverse_api::ID,
        )?;

    // Transfer scan fee
    if config.scan_fee > 0 {
        solana_program::program::invoke(
            &solana_program::system_instruction::transfer(
                signer_info.key,
                fee_collector_info.key,
                config.scan_fee,
            ),
            &[signer_info.clone(), fee_collector_info.clone()],
        )?;
    }

    // Generate richness from slot hash
    let slot_hashes_data = slot_hashes_info.try_borrow_data()?;
    let hash_start = 8 + 8;
    let random_bytes = &slot_hashes_data[hash_start..hash_start + 4];

    let random_u32 = u32::from_le_bytes([
        random_bytes[0],
        random_bytes[1],
        random_bytes[2],
        random_bytes[3],
    ]);

    let roll = random_u32 % 10000;

    let richness: u32 = if roll < 8000 {
        let range_roll = random_u32 % 250_000_001;
        750_000_000 + range_roll
    } else if roll < 9500 {
        let range_roll = random_u32 % 250_000_001;
        500_000_000 + range_roll
    } else if roll < 9900 {
        let range_roll = random_u32 % 250_000_001;
        250_000_000 + range_roll
    } else if roll < 9990 {
        let range_roll = random_u32 % 150_000_001;
        100_000_000 + range_roll
    } else if roll < 9999 {
        let range_roll = random_u32 % 80_000_001;
        20_000_000 + range_roll
    } else {
        random_u32 % 20_000_001
    };

    let clock = Clock::get()?;

    // Create dimension account
    create_program_account::<Dimension>(
        dimension_info,
        system_program,
        signer_info,
        &localuniverse_api::ID,
        &[DIMENSION, &dimension_id.to_le_bytes()],
    )?;

    let dimension = dimension_info.as_account_mut::<Dimension>(&localuniverse_api::ID)?;
    dimension.authority = *signer_info.key;
    dimension.discoverer = *signer_info.key;
    dimension.id = dimension_id;
    dimension.richness = richness;
    dimension.scanned_at = clock.unix_timestamp;

    // Create drill account
    create_program_account::<Drill>(
        drill_info,
        system_program,
        signer_info,
        &localuniverse_api::ID,
        &[DRILL, &dimension_id.to_le_bytes()],
    )?;

    let drill = drill_info.as_account_mut::<Drill>(&localuniverse_api::ID)?;
    drill.dimension_id = dimension_id;
    drill.total_deployed = 0;
    drill.miner_count = 0;
    drill.depth = 0;
    drill.tick_id = 0;
    drill.slot_hash = [0u8; 32];

    // Create navigator if needed
    if navigator_info.data_is_empty() {
        create_program_account::<Navigator>(
            navigator_info,
            system_program,
            signer_info,
            &localuniverse_api::ID,
            &[NAVIGATOR, signer_info.key.as_ref()],
        )?;

        let navigator = navigator_info.as_account_mut::<Navigator>(&localuniverse_api::ID)?;
        navigator.authority = *signer_info.key;
        navigator.lifetime_dimensions_discovered = 1;
        navigator.lifetime_rewards_luxite = 0;
        navigator.lifetime_deployed = 0;
        navigator.created_at = clock.unix_timestamp;
    } else {
        let navigator = navigator_info.as_account_mut::<Navigator>(&localuniverse_api::ID)?;
        navigator.lifetime_dimensions_discovered += 1;
    }

    ScanEvent {
        disc: LocalUniverseEvent::Scan as u64,
        dimension_id,
        scanner: *signer_info.key,
        richness: richness as u64,
        ts: clock.unix_timestamp,
    }
    .log();

    Ok(())
}
