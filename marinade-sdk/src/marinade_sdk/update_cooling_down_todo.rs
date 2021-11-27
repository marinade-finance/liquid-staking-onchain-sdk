#![allow(unused)]
use super::*;
fn main() {
       /* TODO:
    pub fn update_cooling_down(
        state: &impl Located<State>,
        stake_account: Pubkey,
        stake_index: u32,
        withdraw_amount: u64,
    ) -> Instruction {
        let accounts = accounts::UpdateCoolingDown {
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
                token_program: spl_token::ID,
                stake_program: stake::program::ID,
            },
        }
        .to_account_metas(None);

        let data = instruction::UpdateCoolingDown {
            stake_index,
            withdraw_amount,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }*/

}