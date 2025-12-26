use const_crypto::ed25519;
use solana_program::{pubkey, pubkey::Pubkey};

/// The authority allowed to initialize the program.
pub const ADMIN_ADDRESS: Pubkey = pubkey!("HHjpcGGn14vVK6Uqnakih88T1s3y3vmxLoh6Ua12tcX5");

/// The decimal precision of the LUXITE token.
pub const TOKEN_DECIMALS: u8 = 6;

/// One LUXITE token, denominated in indivisible units.
pub const ONE_LUXITE: u64 = 10u64.pow(TOKEN_DECIMALS as u32); // 1_000_000

/// The duration of one minute, in seconds.
pub const ONE_MINUTE: i64 = 60;

/// The duration of one hour, in seconds.
pub const ONE_HOUR: i64 = 60 * ONE_MINUTE;

/// The duration of one day, in seconds.
pub const ONE_DAY: i64 = 24 * ONE_HOUR;

/// The number of seconds for when the winning square expires.
pub const ONE_WEEK: i64 = 7 * ONE_DAY;

/// The number of slots in one week.
pub const ONE_MINUTE_SLOTS: u64 = 150;

/// The number of slots in one hour.
pub const ONE_HOUR_SLOTS: u64 = 60 * ONE_MINUTE_SLOTS;

/// The number of slots in 12 hours.
pub const TWELVE_HOURS_SLOTS: u64 = 12 * ONE_HOUR_SLOTS;

/// The number of slots in one day.
pub const ONE_DAY_SLOTS: u64 = 24 * ONE_HOUR_SLOTS;

/// The number of slots in one week.
pub const ONE_WEEK_SLOTS: u64 = 7 * ONE_DAY_SLOTS;

/// The number of slots in one tick.
pub const TICK_DURATION_SLOTS: u64 = ONE_MINUTE_SLOTS;

/// The number of slots for breather between rounds.
pub const INTERMISSION_SLOTS: u64 = 35;

/// Emission rate per tick in basis points (14 bps = ~0.0014% per tick, ~0.2% per day).
pub const TICK_EMISSION_BPS: u64 = 14;

/// The maximum token supply (1 billion).
pub const MAX_SUPPLY: u64 = ONE_LUXITE * 1_000_000_000;

/// The seed of the config account PDA.
pub const CONFIG: &[u8] = b"config";

/// Seed for automation PDA.
pub const AUTOMATION: &[u8] = b"automation";

/// The seed of the grid account PDA.
pub const GRID: &[u8] = b"grid";

/// The seed of the dimension account PDA.
pub const DIMENSION: &[u8] = b"dimension";

/// The seed of the drill account PDA.
pub const DRILL: &[u8] = b"drill";

/// Seed for excavation PDA.
pub const EXCAVATION: &[u8] = b"excavation";

/// The seed of the drill account PDA.
pub const NAVIGATOR: &[u8] = b"navigator";

/// The seed of the miner account PDA.
pub const MINER: &[u8] = b"miner";

/// The seed of the treasury account PDA.
pub const TREASURY: &[u8] = b"treasury";

/// The seed of the stake account PDA.
pub const STAKE: &[u8] = b"stake";

/// Program id for const pda derivations
const PROGRAM_ID: [u8; 32] = unsafe { *(&crate::id() as *const Pubkey as *const [u8; 32]) };

/// The address of the config account.
pub const CONFIG_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[CONFIG], &PROGRAM_ID).0);

/// The address of the mint account.
pub const MINT_ADDRESS: Pubkey = pubkey!("LUXvvdZyhKyuRHackWFghcJB3L6DjQH2SAvEjmaksRu");

/// The address of the sol mint account.
pub const SOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");

/// The address of the treasury account.
pub const TREASURY_ADDRESS: Pubkey =
    Pubkey::new_from_array(ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).0);

/// The address of the treasury account.
pub const TREASURY_BUMP: u8 = ed25519::derive_program_address(&[TREASURY], &PROGRAM_ID).1;

/// Minimum SOL deployed for full hit rate (0.1 SOL in lamports)
pub const MIN_DEPLOYED_FOR_FULL_RATE: u64 = 100_000_000;

/// The fee paid to bots if they checkpoint a user.
pub const CHECKPOINT_FEE: u64 = 10_000; // 0.00001 SOL

/// Denominator for basis point calculations (100% = 10,000 bps).
pub const DENOMINATOR_BPS: u64 = 10_000;

/// Fee charged on SOL deployment in basis points (1% = 100 bps).
pub const DEPLOY_FEE_BPS: u64 = 100;

/// Amount paid to bots per transaction for auto-compounding staking yield, in lamports.
pub const COMPOUND_FEE_PER_TRANSACTION: u64 = 7_000;

/// The fee paid to the admin for each transaction.
pub const ADMIN_FEE: u64 = 300; // 3%
                                
/// The fee paid to the admin for each dimension scan.
pub const DIMENSION_SCAN_FEE: u64 = 1_000_000_000; // 0.1 SOL

/// The address to receive the admin fee.
pub const ADMIN_FEE_COLLECTOR: Pubkey = pubkey!("Eb3BaMhYbcgcuFUnxtEkVzeFMpppLyFHQeEiM4XgGDJ5");

/// The swap program used for buybacks.
pub const SWAP_PROGRAM: Pubkey = pubkey!("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4");

/// The address which can call the bury and wrap instructions.
pub const BUYBACK_AUTHORITY: Pubkey = pubkey!("EXKfBEgkcaHTmAkuwaynrkRYf7LQ4YFKKhzPUJRJ4aGg");
