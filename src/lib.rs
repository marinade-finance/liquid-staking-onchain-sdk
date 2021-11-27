#![cfg_attr(not(debug_assertions), deny(warnings))]

//! Marinade Program SDK
pub use ::marinade_finance; // reexport contract crate

use ::marinade_finance::*;
use anchor_lang::solana_program::{
    instruction::Instruction,
    pubkey::Pubkey,
    stake, system_program,
    sysvar::{clock, rent},
};
/// instruction helpers to be used by:
/// * other on-chain programs
/// * cli tools
/// * integration tests
use anchor_lang::{InstructionData, ToAccountMetas};
use anchor_spl::token;
use marinade_finance::{
    liq_pool::LiqPoolHelpers,
    located::Located,
    state::StateHelpers,
    validator_system::ValidatorRecord,
};

/// Deposit SOL into Marinade and deliver mSOL to the user
/// This can be viewed also as: SWAP SOL->mSOL with zero fee
/// Before the end of the epoch, the bot will stake or unstake based on the delta from deposit vs unstakes
pub fn deposit(
    state: &impl Located<State>,
    transfer_from: Pubkey,
    mint_to: Pubkey,
    lamports: u64,
) -> Instruction {
    let accounts = accounts::Deposit {
        state: state.key(),
        msol_mint: state.as_ref().msol_mint,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        liq_pool_msol_leg_authority: state.liq_pool_msol_leg_authority(),
        reserve_pda: state.reserve_address(),
        transfer_from,
        mint_to,
        msol_mint_authority: state.msol_mint_authority(),
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::Deposit { lamports };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}

/// Deposit an active-stake-account into Marinade and deliver mSOL to the user
/// Marinade takes control of the stake-account
/// The validator must be in the list for the stake-account to be accepted
/// If it is a concentrated validator, the stake will be rebalanced gradually in the next epochs
pub fn deposit_stake_account(
    state: &impl Located<State>,
    stake_account: Pubkey,
    stake_authority: Pubkey,
    mint_to: Pubkey,
    validator_index: u32,
    validator_vote: Pubkey,
    rent_payer: Pubkey,
) -> Instruction {
    let accounts = accounts::DepositStakeAccount {
        state: state.key(),
        validator_list: *state.as_ref().validator_system.validator_list_address(),
        stake_list: *state.as_ref().stake_system.stake_list_address(),
        stake_account,
        stake_authority,
        duplication_flag: ValidatorRecord::find_duplication_flag(&state.key(), &validator_vote).0,
        rent_payer,
        msol_mint: state.as_ref().msol_mint,
        mint_to,
        msol_mint_authority: state.msol_mint_authority(),
        clock: clock::id(),
        rent: rent::id(),
        system_program: system_program::ID,
        token_program: token::ID,
        stake_program: stake::program::ID,
    }
    .to_account_metas(None);

    let data = instruction::DepositStakeAccount { validator_index };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}


/// "Unstake Now!" functionality
/// Swaps mSOL->SOL in the liquidity pool, paying a fee
/// the fee can vary from 0.3% (normal) to 3% if the liquidity pool is depleted
/// It's similar to an AMM swap except:
///   a) The price is the true price of mSOL
///   b) the operation does not affect the mSOL price  
///   c) the fee is flat 0.3% unless you're making the pool go below target liquidity
pub fn liquid_unstake(
    state: &impl Located<State>,
    get_msol_from: Pubkey,
    get_msol_from_authority: Pubkey,
    transfer_sol_to: Pubkey,
    msol_amount: u64,
) -> Instruction {
    let accounts = accounts::LiquidUnstake {
        state: state.key(),
        msol_mint: state.as_ref().msol_mint,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        get_msol_from,
        get_msol_from_authority,
        transfer_sol_to,
        treasury_msol_account: state.as_ref().treasury_msol_account,
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::LiquidUnstake { msol_amount };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}

/// Adds Liquidity to the mSOL/SOL Liquidity pool
/// The User only add SOL liquidity. There's NO need to add 50% SOL and 50% mSOL
/// Because the user adds only SOL to the pool,
/// the mSOL/SOL Liquidity pool is free from impermanent/divergent loss, meaning
/// that in no case the user can receive less SOL value than the value they put in
pub fn add_liquidity(
    state: &impl Located<State>,
    transfer_from: Pubkey,
    mint_to: Pubkey,
    lamports: u64,
) -> Instruction {
    let accounts = accounts::AddLiquidity {
        state: state.key(),
        lp_mint: state.as_ref().liq_pool.lp_mint,
        lp_mint_authority: state.lp_mint_authority(),
        // msol_mint: state.as_ref().msol_mint,
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        transfer_from,
        mint_to,
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::AddLiquidity { lamports };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}

/// Removes Liquidity from the mSOL/SOL Liquidity pool
/// User can receive SOL & mSOL in proportion of the current pool composition
/// In no case the user can receive less SOL value than the value they put in
/// The mSOL/SOL Liquidity pool is free from impermanent/divergent loss
pub fn remove_liquidity(
    state: &impl Located<State>,
    burn_from: Pubkey,
    burn_from_authority: Pubkey,
    transfer_sol_to: Pubkey,
    transfer_msol_to: Pubkey,
    tokens: u64,
) -> Instruction {
    let accounts = accounts::RemoveLiquidity {
        state: state.key(),
        lp_mint: state.as_ref().liq_pool.lp_mint,
        // msol_mint: state.as_ref().msol_mint,
        burn_from,
        burn_from_authority, //owner acc is also token owner
        transfer_sol_to,
        transfer_msol_to,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        liq_pool_msol_leg_authority: state.liq_pool_msol_leg_authority(),
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::RemoveLiquidity { tokens };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}



/// Starts a delayed-unstake operation
/// creates a delayed-unstake Ticket-account
pub fn order_unstake(
    state: &impl Located<State>,
    burn_msol_from: Pubkey,
    burn_msol_authority: Pubkey, // delegated or owner
    msol_amount: u64,
    new_ticket_account: Pubkey,
) -> Instruction {
    let accounts = accounts::OrderUnstake {
        state: state.key(),
        msol_mint: state.as_ref().msol_mint,
        burn_msol_from,
        burn_msol_authority,
        new_ticket_account,
        token_program: token::ID,
        clock: clock::ID,
        rent: rent::ID,
    }
    .to_account_metas(None);

    let data = instruction::OrderUnstake { msol_amount };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}

/// Claim instruction: a user claims a Ticket-account
/// This is done once tickets are due, meaning enough time has passed for the
/// bot to complete the unstake process and transfer the requested SOL to reserve_pda.
/// Checks that transfer request amount is less than total requested for unstake and then transfer SOL to beneficiary
pub fn claim(
    state: &impl Located<State>,
    ticket_account: Pubkey,
    transfer_sol_to: Pubkey,
) -> Instruction {
    let accounts = accounts::Claim {
        state: state.key(),
        reserve_pda: state.reserve_address(),
        ticket_account,
        transfer_sol_to,
        system_program: system_program::ID,
        clock: clock::ID,
    }
    .to_account_metas(None);

    let data = instruction::Claim {};

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}
