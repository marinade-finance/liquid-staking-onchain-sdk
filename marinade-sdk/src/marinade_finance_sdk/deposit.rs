#![allow(unused)]
use super::*;

fn main() {
    pub fn deposit(
        state: &impl super::Located<super::State>,
        transfer_from: super::Pubkey,
        mint_to: super::Pubkey,
        lamports: u64,
    ) -> super::Instruction {
        let accounts = super::accounts::Deposit {
            state: state.key(),
            msol_mint: state.as_ref().msol_mint,
            liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
            liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
            liq_pool_msol_leg_authority: state.liq_pool_msol_leg_authority(),
            reserve_pda: state.reserve_address(),
            transfer_from,
            mint_to,
            msol_mint_authority: state.msol_mint_authority(),
            system_program: super::system_program::ID,
            token_program: super::token::ID,
        }
        .to_account_metas(None);

        let data = super::instruction::Deposit { lamports };

        super::Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}