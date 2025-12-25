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
        "miner" => log_miner(&rpc, &payer).await.unwrap(),
        "navigator" => log_navigator(&rpc, &payer).await.unwrap(),
        "stake" => log_stake(&rpc, &payer).await.unwrap(),
        "initialize" => initialize(&rpc, &payer).await.unwrap(),
        "scan" => scan(&rpc, &payer).await.unwrap(),
        "deploy" => deploy(&rpc, &payer).await.unwrap(),
        "tick" => tick(&rpc, &payer).await.unwrap(),
        "excavate" => excavate(&rpc, &payer).await.unwrap(),
        "claim_luxite" => claim_luxite(&rpc, &payer).await.unwrap(),
        "deposit" => deposit(&rpc, &payer).await.unwrap(),
        "withdraw" => withdraw(&rpc, &payer).await.unwrap(),
        "claim_yield" => claim_yield(&rpc, &payer).await.unwrap(),
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
    println!("  sol_balance: {} SOL", lamports_to_sol(treasury.sol_balance));
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
        "  stake_rewards_factor: {}",
        format!("{:?}", treasury.stake_rewards_factor)
    );
    Ok(())
}

async fn log_dimension(rpc: &RpcClient) -> Result<(), anyhow::Error> {
    let id = std::env::var("ID").expect("Missing ID env var");
    let id = u64::from_str(&id).expect("Invalid ID");
    let dimension_address = dimension_pda(id).0;
    let dimension = get_dimension(rpc, id).await?;
    println!("Dimension");
    println!("  address: {}", dimension_address);
    println!("  id: {}", dimension.id);
    println!("  authority: {}", dimension.authority);
    println!("  discoverer: {}", dimension.discoverer);
    println!("  richness: {}", dimension.richness);
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
    println!("  total_deployed: {} SOL", lamports_to_sol(drill.total_deployed));
    println!("  miner_count: {}", drill.miner_count);
    println!("  tick_id: {}", drill.tick_id);
    println!("  depth: {}", drill.depth);
    println!(
        "  rewards_factor: {}",
        format!("{:?}", drill.rewards_factor)
    );
    println!(
        "  total_unclaimed: {} LUXITE",
        amount_to_ui_amount(drill.total_unclaimed, TOKEN_DECIMALS)
    );
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
    println!("Miner");
    println!("  address: {}", miner_address);
    println!("  authority: {}", miner.authority);
    println!("  dimension_id: {}", miner.dimension_id);
    println!("  deployed: {} SOL", lamports_to_sol(miner.deployed));
    println!("  tick_id: {}", miner.tick_id);
    println!(
        "  rewards_luxite: {} LUXITE",
        amount_to_ui_amount(miner.rewards_luxite, TOKEN_DECIMALS)
    );
    println!(
        "  refined_luxite: {} LUXITE",
        amount_to_ui_amount(miner.refined_luxite, TOKEN_DECIMALS)
    );
    println!(
        "  lifetime_deployed: {} SOL",
        lamports_to_sol(miner.lifetime_deployed)
    );
    println!(
        "  lifetime_rewards_luxite: {} LUXITE",
        amount_to_ui_amount(miner.lifetime_rewards_luxite, TOKEN_DECIMALS)
    );
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
        "  lifetime_rewards_luxite: {} LUXITE",
        amount_to_ui_amount(navigator.lifetime_rewards_luxite, TOKEN_DECIMALS)
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
    let ix = localuniverse_api::sdk::deploy(payer.pubkey(), id, amount);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Deployed {} lamports to dimension {}!", amount, id);
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
    let ix = localuniverse_api::sdk::excavate(payer.pubkey(), id);
    submit_transaction(rpc, payer, &[ix]).await?;
    println!("Excavated dimension {}!", id);
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

async fn get_miner(rpc: &RpcClient, dimension_id: u64, authority: Pubkey) -> Result<Miner, anyhow::Error> {
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
