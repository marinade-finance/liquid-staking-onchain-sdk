#![allow(unused)]
use super::*;
fn main() {
    pub fn remove_validator(
        state: &impl Located<State>,
        index: u32,
        validator_vote: Pubkey,
    ) -> Instruction {
        let accounts = accounts::RemoveValidator {
            state: state.key(),
            manager_authority: state.as_ref().validator_system.manager_authority,
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            duplication_flag: ValidatorRecord::find_duplication_flag(&state.key(), &validator_vote).0,
            operational_sol_account: state.as_ref().operational_sol_account,
        }
        .to_account_metas(None);

        let data = instruction::RemoveValidator {
            index,
            validator_vote,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}