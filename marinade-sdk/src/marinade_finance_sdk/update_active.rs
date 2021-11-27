#![allow(unused)]
use super::*;
fn main() {
    pub fn update_active(
        state: &impl Located<State>,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> Instruction {
        let accounts = accounts::UpdateActive {
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

            validator_list: *state.as_ref().validator_system.validator_list_address(),
        }
        .to_account_metas(None);

        let data = instruction::UpdateActive {
            stake_index,
            validator_index,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}