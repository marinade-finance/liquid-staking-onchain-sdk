#![allow(unused)]
use super::*;
fn main() {
    pub fn set_validator_score(
        state: &impl Located<State>,
        index: u32,
        validator_vote: Pubkey,
        score: u32,
    ) -> Instruction {
        let accounts = accounts::SetValidatorScore {
            state: state.key(),
            manager_authority: state.as_ref().validator_system.manager_authority,
            validator_list: *state.as_ref().validator_system.validator_list_address(),
        }
        .to_account_metas(None);

        let data = instruction::SetValidatorScore {
            index,
            validator_vote,
            score,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }  
}