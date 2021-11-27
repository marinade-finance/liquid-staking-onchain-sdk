#![allow(unused)]
use super::*;
fn main() {
    pub fn merge_stakes(
        state: &impl Located<State>,
        destination_stake: Pubkey,
        destination_stake_index: u32,
        source_stake: Pubkey,
        source_stake_index: u32,
        validator_index: u32,
    ) -> Instruction {
        let accounts = accounts::MergeStakes {
            state: state.key(),
            stake_list: *state.as_ref().stake_system.stake_list_address(),
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            destination_stake,
            source_stake,
            stake_deposit_authority: state.stake_deposit_authority(),
            stake_withdraw_authority: state.stake_withdraw_authority(),
            operational_sol_account: state.as_ref().operational_sol_account,

            clock: clock::ID,
            stake_history: stake_history::id(),

            stake_program: stake::program::ID,
        }
        .to_account_metas(None);

        let data = instruction::MergeStakes {
            destination_stake_index,
            source_stake_index,
            validator_index,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}