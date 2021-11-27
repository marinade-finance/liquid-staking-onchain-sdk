#![allow(unused)]
use super::*;
fn main() {
    pub fn stake_reserve(
        state: &impl Located<State>,
        validator_index: u32,
        validator_vote: Pubkey,
        stake_account: Pubkey,
    ) -> Instruction {
        let accounts = accounts::StakeReserve {
            state: state.key(),
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            stake_list: *state.as_ref().stake_system.stake_list_address(),
            validator_vote,
            reserve_pda: state.reserve_address(),
            stake_account,
            stake_deposit_authority: state.stake_deposit_authority(),
            clock: clock::ID,
            epoch_schedule: epoch_schedule::ID,
            rent: rent::ID,
            stake_history: stake_history::ID,
            stake_config: stake::config::ID,
            system_program: system_program::ID,
            stake_program: stake::program::ID,
        }
        .to_account_metas(None);

        let data = instruction::StakeReserve { validator_index };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}