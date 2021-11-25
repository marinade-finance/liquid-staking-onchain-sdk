#![allow(unused)]
use super::*;
pub fn main() {
    // use marinade_finance::liq_pool::LiqPoolHelpers;
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
}