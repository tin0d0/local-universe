use std::str::FromStr;

use localuniverse_api::prelude::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    compute_budget::ComputeBudgetInstruction,
    native_token::lamports_to_sol,
    pubkey::Pubkey,
    signature::{read_keypair_file, Signer},
    transaction::Transaction,
};
use spl_associated_token_account::get_associated_token_address;
use spl_token::amount_to_ui_amount;
use steel::{AccountDeserialize, Clock};

#[tokio::main]
async fn main() {
    let payer =
        read_keypair_file(&std::env::var("KEYPAIR").expect("Missing KEYPAIR env var")).unwrap();

    let rpc = RpcClient::new(std::env::var("RPC").expect("Missing RPC env var"));

    match std::env::var("COMMAND")
        .expect("Missing COMMAND env var")
        .as_str()
    {
        "clock" => log_clock(&rpc).await.unwrap(),
        "config" => log_config(&rpc).await.unwrap(),
        "grid" => log_grid(&rpc).await.unwrap(),
        "treasury" => log_treasury(&rpc).await.unwrap(),
        "dimension" => log_dimension(&rpc).await.unwrap(),
        "drill" => log_drill(&rpc).await.unwrap(),
        "excavation" => log_excavation(&rpc).await.unwrap(),
        "miner" => log_miner(&rpc, &payer).await.unwrap(),
        "navigator" => log_navigator(&rpc, &payer).await.unwrap(),
        "stake" => log_stake(&rpc, &payer).await.unwrap(),
        "initialize" => initialize(&rpc, &payer).await.unwrap(),
        "scan" => scan(&rpc, &payer).await.unwrap(),
        "deploy" => deploy(&rpc, &payer).await.unwrap(),
        "tick" => tick(&rpc, &payer).await.unwrap(),
        "excavate" => excavate(&rpc, &payer).await.unwrap(),
        "checkpoint" => checkpoint(&rpc, &payer).await.unwrap(),
        "close" => close(&rpc, &payer).await.unwrap(),
        "claim_luxite" => claim_luxite(&rpc, &payer).await.unwrap(),
        "claim_sol" => claim_sol(&rpc, &payer).await.unwrap(),
        "deposit" => deposit(&rpc, &payer).await.unwrap(),
        "withdraw" => withdraw(&rpc, &payer).await.unwrap(),
        "claim_yield" => claim_yield(&rpc, &payer).await.unwrap(),
        "automation" => log_automation(&rpc, &payer).await.unwrap(),
        "automate" => automate(&rpc, &payer).await.unwrap(),
        "reload-sol" => reload_sol(&rpc, &payer).await.unwrap(),
        "fund_treasury" => fund_treasury(&rpc, &payer).await.unwrap(),
        "keys" => keys().await.unwrap(),
        _ => panic!("Invalid command"),
    };
}

// ============================================================================
// Logging commands
// ============================================================================

async fn log_clock(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let clock = get_clock(rpc).await?;
    println!("Clock");
    println!("  slot: {}", clock.slot);
    println!("  unix_timestamp: {}", clock.unix_timestamp);
    Ok(())
}

async fn log_config(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let config_address = config_pda().0;
    let config = get_config(rpc).await?;
    println!("Config");
    println!("  address: {}", config_address);
    println!("  admin: {}", config.admin);
    Ok(())
}

async fn log_grid(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let grid_address = grid_pda().0;
    let grid = get_grid(rpc).await?;
    let clock = get_clock(rpc).await?;
    println!("Grid");
    println!("  address: {}", grid_address);
    println!("  tick_id: {}", grid.tick_id);
    println!("  start_slot: {}", grid.start_slot);
    println!("  end_slot: {}", grid.end_slot);
    println!("  epoch_id: {}", grid.epoch_id);
    println!(
        "  time_remaining: {} sec",
        (grid.end_slot.saturating_sub(clock.slot) as f64) * 0.4
    );
    Ok(())
}

async fn log_treasury(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let treasury_address = treasury_pda().0;
    let treasury = get_treasury(rpc).await?;
    let treasury_tokens = get_associated_token_address(&treasury_address, &MINT_ADDRESS);

    println!("Treasury");
    println!("  address: {}", treasury_address);
    println!("  tokens: {}", treasury_tokens);
    println!(
        "  sol_balance: {} SOL",
        lamports_to_sol(treasury.sol_balance)
    );
    println!(
        "  luxite_balance: {} LUXITE",
        amount_to_ui_amount(treasury.luxite_balance, TOKEN_DECIMALS)
    );
    println!(
        "  total_staked: {} LUXITE",
        amount_to_ui_amount(treasury.total_staked, TOKEN_DECIMALS)
    );
    println!(
        "  total_burned: {} LUXITE",
        amount_to_ui_amount(treasury.total_burned, TOKEN_DECIMALS)
    );
    println!(
        "  total_emitted: {} LUXITE",
        amount_to_ui_amount(treasury.total_emitted, TOKEN_DECIMALS)
    );
    println!(
        "  total_unclaimed: {} LUXITE",
        amount_to_ui_amount(treasury.total_unclaimed, TOKEN_DECIMALS)
    );
    println!(
        "  total_refined: {} LUXITE",
        amount_to_ui_amount(treasury.total_refined, TOKEN_DECIMALS)
    );
    println!(
        "  stake_rewards_factor: {:?}",
        treasury.stake_rewards_factor
    );
    println!(
        "  miner_rewards_factor: {:?}",
        treasury.miner_rewards_factor
    );
    Ok(())
}

async fn log_dimension(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let dimension_address = dimension_pda(id).0;
    let dimension = get_dimension(rpc, id).await?;

    // Calculate hit chance from richness (roll must be > richness to hit)
    let hit_chance = 100.0 - (dimension.richness as f64 / 10_000_000.0);

    println!("Dimension");
    println!("  address: {}", dimension_address);
    println!("  id: {}", dimension.id);
    println!("  authority: {}", dimension.authority);
    println!("  discoverer: {}", dimension.discoverer);
    println!(
        "  richness: {} ({:.2}% hit chance)",
        dimension.richness, hit_chance
    );
    println!("  scanned_at: {}", dimension.scanned_at);
    Ok(())
}

async fn log_drill(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let drill_address = drill_pda(id).0;
    let drill = get_drill(rpc, id).await?;

    println!("Drill");
    println!("  address: {}", drill_address);
    println!("  dimension_id: {}", drill.dimension_id);
    println!("  depth: {}", drill.depth);
    println!(
        "  lifetime_deployed: {} SOL",
        lamports_to_sol(drill.lifetime_deployed)
    );
    println!(
        "  lifetime_rewards_luxite: {} LUXITE",
        amount_to_ui_amount(drill.lifetime_rewards_luxite, TOKEN_DECIMALS)
    );
    Ok(())
}

async fn log_excavation(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let dimension_id = std::env::var("ID").expect("Missing ID env var");
    let dimension_id = u64::from_str(&dimension_id).expect("Invalid ID");

    let tick_id = std::env::var("TICK").expect("Missing TICK env var");
    let tick_id = u64::from_str(&tick_id).expect("Invalid TICK");

    let excavation_address = excavation_pda(dimension_id, tick_id).0;

    match get_excavation(rpc, dimension_id, tick_id).await {
        Ok(excavation) => {
            let clock = get_clock(rpc).await?;
            let is_expired = clock.slot >= excavation.expires_at;
            let is_processed = excavation.slot_hash != [0; 32];

            let status = if !is_processed {
                "PENDING"
            } else if excavation.did_hit == 1 {
                "HIT"
            } else {
                "MISS"
            };

            println!("Excavation");
            println!("  address: {}", excavation_address);
            println!("  id: {}", excavation.id);
            println!("  dimension_id: {}", excavation.dimension_id);
            println!("  status: {}", status);
            println!(
                "  total_deployed: {} SOL",
                lamports_to_sol(excavation.total_deployed)
            );
            println!("  total_miners: {}", excavation.total_miners);
            println!(
                "  luxite_distributed: {} LUXITE",
                amount_to_ui_amount(excavation.luxite_distributed, TOKEN_DECIMALS)
            );
            println!("  expires_at: {} (expired: {})", excavation.expires_at, is_expired);
            println!("  rent_payer: {}", excavation.rent_payer);

            // Show time until expiry
            if is_processed && !is_expired {
                let slots_remaining = excavation.expires_at.saturating_sub(clock.slot);
                let seconds_remaining = (slots_remaining as f64) * 0.4;
                let hours_remaining = seconds_remaining / 3600.0;
                println!("  time_until_expiry: {:.2} hours", hours_remaining);
            }
        }
        Err(_) => {
            println!("Excavation");
            println!("  address: {}", excavation_address);
            println!("  status: NOT FOUND (not created or already closed)");
        }
    }

    Ok(())
}

async fn log_miner(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let authority = std::env::var("AUTHORITY").unwrap_or(payer.pubkey().to_string());
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let miner_address = miner_pda(id, authority).0;
    let miner = get_miner(rpc, id, authority).await?;

    let needs_checkpoint = miner.checkpoint_id != miner.excavation_id && miner.excavation_id > 0;

    println!("Miner");
    println!("  address: {}", miner_address);
    println!("  authority: {}", miner.authority);
    println!("  dimension_id: {}", miner.dimension_id);
    println!("  --- Current Excavation ---");
    println!("  excavation_id: {}", miner.excavation_id);
    println!("  checkpoint_id: {}", miner.checkpoint_id);
    println!("  needs_checkpoint: {}", needs_checkpoint);
    println!("  deployed: {} SOL", lamports_to_sol(miner.deployed));
    println!(
        "  checkpoint_fee: {} SOL",
        lamports_to_sol(miner.checkpoint_fee)
    );
    println!("  --- Pending Rewards ---");
    println!(
        "  rewards_sol: {} SOL",
        lamports_to_sol(miner.rewards_sol)
    );
    println!(
        "  rewards_luxite: {} LUXITE",
        amount_to_ui_amount(miner.rewards_luxite, TOKEN_DECIMALS)
    );
    println!(
        "  refined_luxite: {} LUXITE",
        amount_to_ui_amount(miner.refined_luxite, TOKEN_DECIMALS)
    );
    println!("  rewards_factor: {:?}", miner.rewards_factor);
    println!("  --- Lifetime Stats ---");
    println!(
        "  lifetime_deployed: {} SOL",
        lamports_to_sol(miner.lifetime_deployed)
    );
    println!(
        "  lifetime_rewards_sol: {} SOL",
        lamports_to_sol(miner.lifetime_rewards_sol)
    );
    println!(
        "  lifetime_rewards_luxite: {} LUXITE",
        amount_to_ui_amount(miner.lifetime_rewards_luxite, TOKEN_DECIMALS)
    );
    println!("  --- Timestamps ---");
    println!("  last_claim_luxite_at: {}", miner.last_claim_luxite_at);
    println!("  last_claim_sol_at: {}", miner.last_claim_sol_at);
    Ok(())
}

async fn log_automation(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let authority = std::env::var("AUTHORITY").unwrap_or(payer.pubkey().to_string());
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");

    let automation_address = automation_pda(authority, id).0;

    match get_automation(rpc, authority, id).await {
        Ok(automation) => {
            println!("Automation");
            println!("  address: {}", automation_address);
            println!("  authority: {}", automation.authority);
            println!("  dimension_id: {}", automation.dimension_id);
            println!("  executor: {}", automation.executor);
            println!(
                "  amount: {} SOL",
                lamports_to_sol(automation.amount)
            );
            println!(
                "  balance: {} SOL",
                lamports_to_sol(automation.balance)
            );
            println!(
                "  fee: {} SOL",
                lamports_to_sol(automation.fee)
            );
            println!("  reload: {}", automation.reload == 1);
        }
        Err(_) => {
            println!("Automation");
            println!("  address: {}", automation_address);
            println!("  status: NOT FOUND");
        }
    }

    Ok(())
}

async fn log_navigator(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let authority = std::env::var("AUTHORITY").unwrap_or(payer.pubkey().to_string());
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");
    let navigator_address = navigator_pda(authority).0;
    let navigator = get_navigator(rpc, authority).await?;
    println!("Navigator");
    println!("  address: {}", navigator_address);
    println!("  authority: {}", navigator.authority);
    println!(
        "  lifetime_dimensions_discovered: {}",
        navigator.lifetime_dimensions_discovered
    );
    println!(
        "  lifetime_deployed: {} SOL",
        lamports_to_sol(navigator.lifetime_deployed)
    );
    println!(
        "  lifetime_rewards_luxite: {} LUXITE",
        amount_to_ui_amount(navigator.lifetime_rewards_luxite, TOKEN_DECIMALS)
    );
    println!(
        "  lifetime_rewards_sol: {} SOL",
        lamports_to_sol(navigator.lifetime_rewards_sol)
    );
    Ok(())
}

async fn log_stake(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let authority = std::env::var("AUTHORITY").unwrap_or(payer.pubkey().to_string());
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");
    let stake_address = stake_pda(authority).0;
    let stake = get_stake(rpc, authority).await?;
    println!("Stake");
    println!("  address: {}", stake_address);
    println!("  authority: {}", stake.authority);
    println!(
        "  balance: {} LUXITE",
        amount_to_ui_amount(stake.balance, TOKEN_DECIMALS)
    );
    println!(
        "  rewards: {} LUXITE",
        amount_to_ui_amount(stake.rewards, TOKEN_DECIMALS)
    );
    println!(
        "  lifetime_rewards: {} LUXITE",
        amount_to_ui_amount(stake.lifetime_rewards, TOKEN_DECIMALS)
    );
    println!(
        "  compound_fee_reserve: {} SOL",
        lamports_to_sol(stake.compound_fee_reserve)
    );
    println!("  last_deposit_at: {}", stake.last_deposit_at);
    println!("  last_withdraw_at: {}", stake.last_withdraw_at);
    println!("  last_claim_at: {}", stake.last_claim_at);
    Ok(())
}

async fn keys() -> Result<(), anyhow::Error> {
    println!("Keys");
    println!("  config: {}", config_pda().0);
    println!("  grid: {}", grid_pda().0);
    println!("  treasury: {}", treasury_pda().0);
    println!("  mint: {}", MINT_ADDRESS);
    Ok(())
}

// ============================================================================
// Action commands
// ============================================================================

async fn initialize(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let ix = localuniverse_api::sdk::initialize(payer.pubkey());
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Initialized!");
    Ok(())
}

async fn scan(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let ix = localuniverse_api::sdk::scan(payer.pubkey(), id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Scanned dimension {}!", id);
    Ok(())
}

async fn deploy(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let amount = std::env::var("AMOUNT").expect("Missing AMOUNT env var");
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");

    // Optional: deploy on behalf of another authority (automation mode)
    let authority = std::env::var("AUTHORITY").unwrap_or(payer.pubkey().to_string());
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");

    // Get current tick from grid
    let grid = get_grid(rpc).await?;

    // Calculate fee for display
    let fee = amount * DEPLOY_FEE_BPS / DENOMINATOR_BPS;
    let amount_at_risk = amount - fee;

    let ix = localuniverse_api::sdk::deploy(payer.pubkey(), authority, id, grid.tick_id, amount);
    submit_transaction(rpc, payer, &[ix]).await?;

    if authority == payer.pubkey() {
        println!(
            "Deployed {} SOL to dimension {} tick {} ({} SOL fee, {} SOL at risk)!",
            lamports_to_sol(amount),
            id,
            grid.tick_id,
            lamports_to_sol(fee),
            lamports_to_sol(amount_at_risk)
        );
    } else {
        println!(
            "Automation deployed {} SOL to dimension {} tick {} for {}!",
            lamports_to_sol(amount),
            id,
            grid.tick_id,
            authority
        );
    }
    Ok(())
}

async fn tick(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let ix = localuniverse_api::sdk::tick(payer.pubkey());
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Tick!");
    Ok(())
}

async fn excavate(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");

    // Get current tick, excavate the previous one
    let grid = get_grid(rpc).await?;
    let previous_tick_id = grid.tick_id.saturating_sub(1);

    let ix = localuniverse_api::sdk::excavate(payer.pubkey(), id, previous_tick_id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Excavated dimension {} tick {}!", id, previous_tick_id);
    Ok(())
}

async fn checkpoint(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");

    // Optional: checkpoint for a different authority (bot mode)
    let authority = std::env::var("AUTHORITY").unwrap_or(payer.pubkey().to_string());
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");

    // Get miner's excavation_id
    let miner = get_miner(rpc, id, authority).await?;

    let ix = localuniverse_api::sdk::checkpoint(payer.pubkey(), authority, id, miner.excavation_id);
    submit_transaction(rpc, payer, &[ix]).await?;

    if authority == payer.pubkey() {
        println!("Checkpointed dimension {} excavation {}!", id, miner.excavation_id);
    } else {
        println!(
            "Checkpointed dimension {} excavation {} for {} (bot mode)!",
            id, miner.excavation_id, authority
        );
    }
    Ok(())
}

async fn close(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let dimension_id = std::env::var("ID").expect("Missing ID env var");
    let dimension_id = u64::from_str(&dimension_id).expect("Invalid ID");

    let tick_id = std::env::var("TICK").expect("Missing TICK env var");
    let tick_id = u64::from_str(&tick_id).expect("Invalid TICK");

    // Get rent payer from excavation
    let excavation = get_excavation(rpc, dimension_id, tick_id).await?;

    let ix = localuniverse_api::sdk::close(payer.pubkey(), dimension_id, tick_id, excavation.rent_payer);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!(
        "Closed excavation {} on dimension {}!",
        tick_id, dimension_id
    );
    Ok(())
}

async fn claim_luxite(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let ix = localuniverse_api::sdk::claim_luxite(payer.pubkey(), id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Claimed LUXITE from dimension {}!", id);
    Ok(())
}

async fn claim_sol(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let ix = localuniverse_api::sdk::claim_sol(payer.pubkey(), id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Claimed SOL from dimension {}!", id);
    Ok(())
}

async fn deposit(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let amount = std::env::var("AMOUNT").expect("Missing AMOUNT env var");
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");
    let compound_fee = std::env::var("COMPOUND_FEE").unwrap_or("0".to_string());
    let compound_fee = u64::from_str(&compound_fee).expect("Invalid COMPOUND_FEE");
    let ix = localuniverse_api::sdk::deposit(payer.pubkey(), payer.pubkey(), amount, compound_fee);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Deposited {} LUXITE!", amount);
    Ok(())
}

async fn withdraw(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let amount = std::env::var("AMOUNT").expect("Missing AMOUNT env var");
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");
    let ix = localuniverse_api::sdk::withdraw(payer.pubkey(), amount);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Withdrew {} LUXITE!", amount);
    Ok(())
}

async fn claim_yield(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let amount = std::env::var("AMOUNT").expect("Missing AMOUNT env var");
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");
    let ix = localuniverse_api::sdk::claim_yield(payer.pubkey(), amount);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Claimed {} LUXITE yield!", amount);
    Ok(())
}

async fn fund_treasury(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let amount = std::env::var("AMOUNT").expect("Missing AMOUNT env var");
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");
    let ix = localuniverse_api::sdk::fund_treasury(payer.pubkey(), amount);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Funded treasury with {} LUXITE!", amount);
    Ok(())
}

async fn automate(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");

    let executor = std::env::var("EXECUTOR").expect("Missing EXECUTOR env var");
    let executor = Pubkey::from_str(&executor).expect("Invalid EXECUTOR");

    let amount = std::env::var("AMOUNT").unwrap_or("0".to_string());
    let amount = u64::from_str(&amount).expect("Invalid AMOUNT");

    let deposit = std::env::var("DEPOSIT").unwrap_or("0".to_string());
    let deposit = u64::from_str(&deposit).expect("Invalid DEPOSIT");

    let fee = std::env::var("FEE").unwrap_or("0".to_string());
    let fee = u64::from_str(&fee).expect("Invalid FEE");

    let reload = std::env::var("RELOAD").unwrap_or("0".to_string());
    let reload = reload == "1" || reload.to_lowercase() == "true";

    let ix = localuniverse_api::sdk::automate(
        payer.pubkey(),
        executor,
        id,
        amount,
        deposit,
        fee,
        reload,
    );
    submit_transaction(rpc, payer, &[ix]).await?;

    if executor == Pubkey::default() {
        println!("Closed automation for dimension {}!", id);
    } else {
        println!(
            "Setup automation for dimension {}: {} SOL/tick, {} SOL deposited, executor: {}",
            id,
            lamports_to_sol(amount),
            lamports_to_sol(deposit),
            executor
        );
    }
    Ok(())
}

async fn reload_sol(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");

    let authority = std::env::var("AUTHORITY").expect("Missing AUTHORITY env var");
    let authority = Pubkey::from_str(&authority).expect("Invalid AUTHORITY");

    let ix = localuniverse_api::sdk::reload_sol(payer.pubkey(), authority, id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Reloaded SOL for dimension {} authority {}!", id, authority);
    Ok(())
}

async fn get_automation(
    rpc: &RpcClient,
    authority: Pubkey,
    dimension_id: u64,
) -> Result<Automation, anyhow::Error> {
    let address = automation_pda(authority, dimension_id).0;
    let account = rpc.get_account(&address).await?;
    let automation = Automation::try_from_bytes(&account.data)?;
    Ok(*automation)
}

// ============================================================================
// Helpers
// ============================================================================

async fn get_clock(rpc: &RpcClient) -> Result<Clock, anyhow::Error> {
    let data = rpc.get_account_data(&solana_sdk::sysvar::clock::ID).await?;
    let clock = bincode::deserialize::<Clock>(&data)?;
    Ok(clock)
}

async fn get_config(rpc: &RpcClient) -> Result<Config, anyhow::Error> {
    let address = config_pda().0;
    let account = rpc.get_account(&address).await?;
    let config = Config::try_from_bytes(&account.data)?;
    Ok(*config)
}

async fn get_grid(rpc: &RpcClient) -> Result<Grid, anyhow::Error> {
    let address = grid_pda().0;
    let account = rpc.get_account(&address).await?;
    let grid = Grid::try_from_bytes(&account.data)?;
    Ok(*grid)
}

async fn get_treasury(rpc: &RpcClient) -> Result<Treasury, anyhow::Error> {
    let address = treasury_pda().0;
    let account = rpc.get_account(&address).await?;
    let treasury = Treasury::try_from_bytes(&account.data)?;
    Ok(*treasury)
}

async fn get_dimension(rpc: &RpcClient, id: u64) -> Result<Dimension, anyhow::Error> {
    let address = dimension_pda(id).0;
    let account = rpc.get_account(&address).await?;
    let dimension = Dimension::try_from_bytes(&account.data)?;
    Ok(*dimension)
}

async fn get_drill(rpc: &RpcClient, id: u64) -> Result<Drill, anyhow::Error> {
    let address = drill_pda(id).0;
    let account = rpc.get_account(&address).await?;
    let drill = Drill::try_from_bytes(&account.data)?;
    Ok(*drill)
}

async fn get_excavation(
    rpc: &RpcClient,
    dimension_id: u64,
    tick_id: u64,
) -> Result<Excavation, anyhow::Error> {
    let address = excavation_pda(dimension_id, tick_id).0;
    let account = rpc.get_account(&address).await?;
    let excavation = Excavation::try_from_bytes(&account.data)?;
    Ok(*excavation)
}

async fn get_miner(
    rpc: &RpcClient,
    dimension_id: u64,
    authority: Pubkey,
) -> Result<Miner, anyhow::Error> {
    let address = miner_pda(dimension_id, authority).0;
    let account = rpc.get_account(&address).await?;
    let miner = Miner::try_from_bytes(&account.data)?;
    Ok(*miner)
}

async fn get_navigator(rpc: &RpcClient, authority: Pubkey) -> Result<Navigator, anyhow::Error> {
    let address = navigator_pda(authority).0;
    let account = rpc.get_account(&address).await?;
    let navigator = Navigator::try_from_bytes(&account.data)?;
    Ok(*navigator)
}

async fn get_stake(rpc: &RpcClient, authority: Pubkey) -> Result<Stake, anyhow::Error> {
    let address = stake_pda(authority).0;
    let account = rpc.get_account(&address).await?;
    let stake = Stake::try_from_bytes(&account.data)?;
    Ok(*stake)
}

async fn submit_transaction(
    rpc: &RpcClient,
    payer: &solana_sdk::signer::keypair::Keypair,
    instructions: &[solana_sdk::instruction::Instruction],
) -> Result<solana_sdk::signature::Signature, anyhow::Error> {
    let blockhash = rpc.get_latest_blockhash().await?;
    let mut all_instructions = vec![
        ComputeBudgetInstruction::set_compute_unit_limit(400_000),
        ComputeBudgetInstruction::set_compute_unit_price(100_000),
    ];
    all_instructions.extend_from_slice(instructions);
    let transaction = Transaction::new_signed_with_payer(
        &all_instructions,
        Some(&payer.pubkey()),
        &[payer],
        blockhash,
    );

    match rpc.send_and_confirm_transaction(&transaction).await {
        Ok(signature) => {
            println!("Transaction: {}", signature);
            Ok(signature)
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}
