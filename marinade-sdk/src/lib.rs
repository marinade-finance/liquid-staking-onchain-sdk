use anchor_lang::prelude::*;

pub mod utility;

use crate::{instructions::*, processor::*};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod marinade_sdk {
    use super::*;
#![cfg_attr(not(debug_assertions), deny(warnings))]

pub mod marinade_finance_sdk {

    //! Marinade Program SDK
    pub use ::marinade_finance; // reexport contract crate

    pub use ::marinade_finance::*;
    pub use anchor_lang::solana_program::{
        instruction::Instruction,
        pubkey::Pubkey,
        stake, system_program,
        sysvar::{clock, epoch_schedule, rent, stake_history},
    };
    /// instruction helpers to be used by:
    /// * other on-chain programs
    /// * cli tools
    /// * integration tests
    pub use anchor_lang::{InstructionData, ToAccountMetas};
    pub use anchor_spl::token;
    pub use marinade_finance::{
        liq_pool::{LiqPool, LiqPoolHelpers},
        located::Located,
        stake_system::StakeSystemHelpers,
        state::StateHelpers,
        validator_system::ValidatorRecord,
    };


    //  pub mod update_cooling_down;  //TODO
 
 

    pub struct InitializeInput {
        pub state: Pubkey,
        pub stake_list: Pubkey,
        pub validator_list: Pubkey,
        pub msol_mint: Pubkey,
        pub admin_authority: Pubkey,
        pub operational_sol_account: Pubkey,
        pub validator_manager_authority: Pubkey,
        // pub treasury_sol_account: Pubkey,
        pub treasury_msol_account: Pubkey,
        pub lp_mint: Pubkey,
        pub liq_pool_msol_leg: Pubkey,
        pub min_stake: u64,
        pub reward_fee: Fee,
        pub lp_liquidity_target: u64,
        pub lp_max_fee: Fee,
        pub lp_min_fee: Fee,
        pub lp_treasury_cut: Fee,
        pub additional_stake_record_space: u32,
        pub additional_validator_record_space: u32,
        pub slots_for_stake_delta: u64,
    }
    pub fn set_lp_params(
        state: &impl Located<State>,
        min_fee: Fee,
        max_fee: Fee,
        liquidity_target: u64,
    ) -> Instruction {
        let accounts = accounts::SetLpParams {
            state: state.key(),
            admin_authority: state.as_ref().admin_authority,
        }
        .to_account_metas(None);

        let data = instruction::SetLpParams {
            min_fee,
            max_fee,
            liquidity_target,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}
    ///register refer code to referral program
    // pub fn initialize(
    //     ctx: Context<Initialize>,
    //     ref_code: String,
    //     referral_bump: u8,
    //     beneficiary_bump: u8,
    // ) -> ProgramResult {
    //     process_initialize(ctx, ref_code, referral_bump, beneficiary_bump)
    // }

    // ///update admin
    // pub fn update_partner(ctx: Context<UpdatePartner>) -> ProgramResult {
    //     process_update_partner(ctx)
    // }

    // ///update referral emergency pause
    // pub fn pause(ctx: Context<Pause>, pause: bool) -> ProgramResult {
    //     process_pause(ctx, pause)
    // }

    // ///deposit SOL
    // pub fn deposit(ctx: Context<Deposit>, lamports: u64) -> ProgramResult {
    //     process_deposit(ctx, lamports)
    // }

    // ///liquid-unstake mSOL
    // pub fn liquid_unstake(ctx: Context<LiquidUnstake>, msol_amount: u64) -> ProgramResult {
    //     process_liquid_unstake(ctx, msol_amount)
    // }

    #![cfg_attr(not(debug_assertions), deny(warnings))]
}
