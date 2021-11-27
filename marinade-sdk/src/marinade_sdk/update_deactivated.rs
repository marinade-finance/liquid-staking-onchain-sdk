#![allow(unused)]
use super::*;
fn main() {
    pub fn update_deactivated(
        state: &impl Located<State>,
        stake_account: Pubkey,
        stake_index: u32,
    ) -> Instruction {
        let accounts = accounts::UpdateDeactivated {
            common: accounts::UpdateCommon {
                state: state.key(),
                stake_list: *state.as_ref().stake_system.stake_list_address(),
                stake_account,
                stake_withdraw_authority: state.stake_withdraw_authority(),
                reserve_pda: state.reserve_address(),
                msol_mint: state.as_ref().msol_mint,
                clock: clock::ID,
                stake_history: stake_history::ID,
                msol_mint_authority: state.msol_mint_authority(),
                treasury_msol_account: state.as_ref().treasury_msol_account,
                token_program: token::ID,
                stake_program: stake::program::ID,
            },
            operational_sol_account: state.as_ref().operational_sol_account,
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        let data = instruction::UpdateDeactivated { stake_index };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}