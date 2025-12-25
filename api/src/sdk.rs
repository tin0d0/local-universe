use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;
use steel::*;

use crate::{
    consts::*,
    instruction::*,
    state::*,
};

/// Builds a Scan instruction to discover a new dimension.
pub fn scan(signer: Pubkey, dimension_id: u64) -> Instruction {
    let config_address = config_pda().0;
    let dimension_address = dimension_pda(dimension_id).0;
    let drill_address = drill_pda(dimension_id).0;
    let navigator_address = navigator_pda(signer).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_address, false),
            AccountMeta::new(dimension_address, false),
            AccountMeta::new(drill_address, false),
            AccountMeta::new(navigator_address, false),
            AccountMeta::new(ADMIN_FEE_COLLECTOR, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: Scan {
            dimension_id: dimension_id.to_le_bytes(),
        }
        .to_bytes(),
    }
}

/// Builds a Deploy instruction to add SOL to a drill.
pub fn deploy(signer: Pubkey, dimension_id: u64, amount: u64) -> Instruction {
    let grid_address = grid_pda().0;
    let drill_address = drill_pda(dimension_id).0;
    let miner_address = miner_pda(dimension_id, signer).0;
    let navigator_address = navigator_pda(signer).0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(grid_address, false),
            AccountMeta::new(drill_address, false),
            AccountMeta::new(miner_address, false),
            AccountMeta::new(navigator_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Deploy {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

/// Builds a Tick instruction to advance the global tick.
pub fn tick(signer: Pubkey) -> Instruction {
    let grid_address = grid_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(grid_address, false),
        ],
        data: Tick {}.to_bytes(),
    }
}

/// Builds an Excavate instruction to process a drill for the current tick.
pub fn excavate(signer: Pubkey, dimension_id: u64) -> Instruction {
    let grid_address = grid_pda().0;
    let dimension_address = dimension_pda(dimension_id).0;
    let drill_address = drill_pda(dimension_id).0;
    let treasury_address = treasury_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(grid_address, false),
            AccountMeta::new_readonly(dimension_address, false),
            AccountMeta::new(drill_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(sysvar::slot_hashes::ID, false),
        ],
        data: Excavate {}.to_bytes(),
    }
}

/// Builds a ClaimLUXITE instruction to claim pending mining rewards.
pub fn claim_luxite(signer: Pubkey, dimension_id: u64) -> Instruction {
    let miner_address = miner_pda(dimension_id, signer).0;
    let navigator_address = navigator_pda(signer).0;
    let drill_address = drill_pda(dimension_id).0;
    let treasury_address = treasury_pda().0;
    let treasury_tokens_address = get_associated_token_address(&treasury_address, &MINT_ADDRESS);
    let recipient_address = get_associated_token_address(&signer, &MINT_ADDRESS);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(miner_address, false),
            AccountMeta::new(navigator_address, false),
            AccountMeta::new(drill_address, false),
            AccountMeta::new_readonly(MINT_ADDRESS, false),
            AccountMeta::new(recipient_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: ClaimLUXITE {}.to_bytes(),
    }
}

/// Builds a Deposit instruction to stake LUXITE.
pub fn deposit(signer: Pubkey, payer: Pubkey, amount: u64, compound_fee: u64) -> Instruction {
    let mint_address = MINT_ADDRESS;
    let stake_address = stake_pda(signer).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &MINT_ADDRESS);
    let sender_address = get_associated_token_address(&signer, &MINT_ADDRESS);
    let treasury_address = treasury_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(payer, true),
            AccountMeta::new(mint_address, false),
            AccountMeta::new(sender_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Deposit {
            amount: amount.to_le_bytes(),
            compound_fee: compound_fee.to_le_bytes(),
        }
        .to_bytes(),
    }
}

/// Builds a Withdraw instruction to unstake LUXITE.
pub fn withdraw(signer: Pubkey, amount: u64) -> Instruction {
    let stake_address = stake_pda(signer).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &MINT_ADDRESS);
    let mint_address = MINT_ADDRESS;
    let recipient_address = get_associated_token_address(&signer, &MINT_ADDRESS);
    let treasury_address = treasury_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint_address, false),
            AccountMeta::new(recipient_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Withdraw {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

/// Builds a ClaimYield instruction to claim staking rewards.
pub fn claim_yield(signer: Pubkey, amount: u64) -> Instruction {
    let stake_address = stake_pda(signer).0;
    let mint_address = MINT_ADDRESS;
    let recipient_address = get_associated_token_address(&signer, &MINT_ADDRESS);
    let treasury_address = treasury_pda().0;
    let treasury_tokens_address = get_associated_token_address(&treasury_address, &MINT_ADDRESS);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(mint_address, false),
            AccountMeta::new(recipient_address, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: ClaimYield {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}

/// Builds a CompoundYield instruction for bots to auto-compound staking rewards.
pub fn compound_yield(signer: Pubkey, authority: Pubkey) -> Instruction {
    let stake_address = stake_pda(authority).0;
    let stake_tokens_address = get_associated_token_address(&stake_address, &MINT_ADDRESS);
    let treasury_address = treasury_pda().0;
    let treasury_tokens_address = get_associated_token_address(&treasury_address, &MINT_ADDRESS);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(MINT_ADDRESS, false),
            AccountMeta::new(stake_address, false),
            AccountMeta::new(stake_tokens_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
        ],
        data: CompoundYield {}.to_bytes(),
    }
}

/// Builds a Buyback instruction to swap SOL for LUXITE and burn.
pub fn buyback(signer: Pubkey, swap_accounts: &[AccountMeta], swap_data: &[u8]) -> Instruction {
    let config_address = config_pda().0;
    let grid_address = grid_pda().0;
    let treasury_address = treasury_pda().0;
    let treasury_luxite_address = get_associated_token_address(&treasury_address, &MINT_ADDRESS);
    let treasury_sol_address = get_associated_token_address(&treasury_address, &SOL_MINT);

    let mut accounts = vec![
        AccountMeta::new(signer, true),
        AccountMeta::new(grid_address, false),
        AccountMeta::new(config_address, false),
        AccountMeta::new(MINT_ADDRESS, false),
        AccountMeta::new(treasury_address, false),
        AccountMeta::new(treasury_luxite_address, false),
        AccountMeta::new(treasury_sol_address, false),
        AccountMeta::new_readonly(spl_token::ID, false),
        AccountMeta::new_readonly(crate::ID, false),
    ];

    for account in swap_accounts.iter() {
        let mut acc_clone = account.clone();
        acc_clone.is_signer = false;
        accounts.push(acc_clone);
    }

    let mut data = Buyback {}.to_bytes();
    data.extend_from_slice(swap_data);

    Instruction {
        program_id: crate::ID,
        accounts,
        data,
    }
}

/// Builds an Initialize instruction (admin only).
pub fn initialize(signer: Pubkey) -> Instruction {
    let config_address = config_pda().0;
    let grid_address = grid_pda().0;
    let treasury_address = treasury_pda().0;
    let treasury_tokens_address = get_associated_token_address(&treasury_address, &MINT_ADDRESS);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_address, false),
            AccountMeta::new(grid_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new_readonly(MINT_ADDRESS, false),
            AccountMeta::new(treasury_tokens_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
            AccountMeta::new_readonly(spl_token::ID, false),
            AccountMeta::new_readonly(spl_associated_token_account::ID, false),
        ],
        data: Initialize {}.to_bytes(),
    }
}

/// Builds a SetAdmin instruction (admin only).
pub fn set_admin(signer: Pubkey, new_admin: Pubkey) -> Instruction {
    let config_address = config_pda().0;
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new(config_address, false),
        ],
        data: SetAdmin {
            admin: new_admin.to_bytes(),
        }
        .to_bytes(),
    }
}

/// Builds a Wrap instruction to convert SOL to WSOL for swaps.
pub fn wrap(signer: Pubkey, amount: u64) -> Instruction {
    let config_address = config_pda().0;
    let treasury_address = treasury_pda().0;
    let treasury_sol_address = get_associated_token_address(&treasury_address, &SOL_MINT);
    Instruction {
        program_id: crate::ID,
        accounts: vec![
            AccountMeta::new(signer, true),
            AccountMeta::new_readonly(config_address, false),
            AccountMeta::new(treasury_address, false),
            AccountMeta::new(treasury_sol_address, false),
            AccountMeta::new_readonly(system_program::ID, false),
        ],
        data: Wrap {
            amount: amount.to_le_bytes(),
        }
        .to_bytes(),
    }
}
