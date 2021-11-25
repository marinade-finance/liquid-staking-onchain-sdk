#![allow(unused)]
use super::*;
fn main() {
    pub fn add_validator(
        state: &impl Located<State>,
        validator_vote: Pubkey,
        score: u32,
        rent_payer: Pubkey,
    ) -> Instruction {
        let accounts = accounts::AddValidator {
            state: state.key(),
            manager_authority: state.as_ref().validator_system.manager_authority,
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            validator_vote,
            duplication_flag: ValidatorRecord::find_duplication_flag(&state.key(), &validator_vote).0,
            rent_payer,
            clock: clock::ID,
            rent: rent::ID,
            system_program: system_program::ID,
        }
        .to_account_metas(None);

        let data = instruction::AddValidator { score };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }

}