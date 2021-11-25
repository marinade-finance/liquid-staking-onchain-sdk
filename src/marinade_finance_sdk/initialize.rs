#![allow(unused)]
use super::*;
fn main() {
    pub fn initialize(
        InitializeInput {
            state,
            stake_list,
            validator_list,
            msol_mint,
            admin_authority,
            operational_sol_account,
            validator_manager_authority,
            // treasury_sol_account,
            treasury_msol_account,
            lp_mint,
            liq_pool_msol_leg,
            min_stake,
            reward_fee,
            lp_liquidity_target,
            lp_max_fee,
            lp_min_fee,
            lp_treasury_cut,
            additional_stake_record_space,
            additional_validator_record_space,
            slots_for_stake_delta,
        }: InitializeInput,
    ) -> Instruction {
        let accounts = accounts::Initialize {
            creator_authority: Initialize::CREATOR_AUTHORITY,
            state,
            reserve_pda: State::find_reserve_address(&state).0,
            stake_list,
            validator_list,
            msol_mint,
            operational_sol_account,
            // treasury_sol_account,
            treasury_msol_account,

            clock: clock::id(),
            rent: rent::id(),
            liq_pool: accounts::LiqPoolInitialize {
                lp_mint,
                sol_leg_pda: LiqPool::find_sol_leg_address(&state).0,
                msol_leg: liq_pool_msol_leg,
            },
        }
        .to_account_metas(None);

        let data = instruction::Initialize {
            data: InitializeData {
                admin_authority,
                validator_manager_authority,
                min_stake,
                reward_fee,
                additional_stake_record_space,
                additional_validator_record_space,
                slots_for_stake_delta,
                liq_pool: LiqPoolInitializeData {
                    lp_liquidity_target,
                    lp_max_fee,
                    lp_min_fee,
                    lp_treasury_cut,
                },
            },
        };
        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}