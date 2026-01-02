mod automate;
mod buyback;
mod checkpoint;
mod claim_luxite;
mod claim_sol;
mod claim_yield;
mod close;
mod compound_yield;
mod deploy;
mod deposit;
mod excavate;
mod fund_treasury;
mod initialize;
mod reload_sol;
mod scan;
mod set_admin;
mod tick;
mod withdraw;
mod wrap;
mod set_scan_fee;

use automate::*;
use buyback::*;
use checkpoint::*;
use claim_luxite::*;
use claim_sol::*;
use claim_yield::*;
use close::*;
use compound_yield::*;
use deploy::*;
use deposit::*;
use excavate::*;
use fund_treasury::*;
use initialize::*;
use reload_sol::*;
use scan::*;
use set_admin::*;
use tick::*;
use withdraw::*;
use wrap::*;
use set_scan_fee::*;

use localuniverse_api::instruction::LocalUniverseInstruction;
use solana_security_txt::security_txt;
use steel::*;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    data: &[u8],
) -> ProgramResult {
    let (ix, data) = parse_instruction(&localuniverse_api::ID, program_id, data)?;

    match ix {
        // Dimension
        LocalUniverseInstruction::Scan => process_scan(accounts, data),

        // Drill
        LocalUniverseInstruction::Tick => process_tick(accounts, data),
        LocalUniverseInstruction::Excavate => process_excavate(accounts, data),

        // Miner
        LocalUniverseInstruction::Deploy => process_deploy(accounts, data),
        LocalUniverseInstruction::Checkpoint => process_checkpoint(accounts, data),
        LocalUniverseInstruction::ClaimLUXITE => process_claim_luxite(accounts, data),
        LocalUniverseInstruction::ClaimSOL => process_claim_sol(accounts, data),
        LocalUniverseInstruction::Close => process_close(accounts, data),

        // Staker
        LocalUniverseInstruction::Deposit => process_deposit(accounts, data),
        LocalUniverseInstruction::Withdraw => process_withdraw(accounts, data),
        LocalUniverseInstruction::ClaimYield => process_claim_yield(accounts, data),
        LocalUniverseInstruction::CompoundYield => process_compound_yield(accounts, data),

        // Automation
        LocalUniverseInstruction::Automate => process_automate(accounts, data),
        LocalUniverseInstruction::ReloadSOL => process_reload_sol(accounts, data),

        // Admin
        LocalUniverseInstruction::Initialize => process_initialize(accounts, data),
        LocalUniverseInstruction::SetAdmin => process_set_admin(accounts, data),
        LocalUniverseInstruction::Buyback => process_buyback(accounts, data),
        LocalUniverseInstruction::Wrap => process_wrap(accounts, data),
        LocalUniverseInstruction::FundTreasury => process_fund_treasury(accounts, data),
        LocalUniverseInstruction::SetScanFee => process_set_scan_fee(accounts, data),
    }
}

entrypoint!(process_instruction);


security_txt! {
    name: "LOCAL UNIVERSE",
    project_url: "https://localuniverse.io",
    contacts: "email:tino@localuniverse.io",
    policy: "https://github.com/tin0d0/local-universe/blob/main/SECURITY.md",
    preferred_languages: "en",
    source_code: "https://github.com/tin0d0/local-universe"
}
