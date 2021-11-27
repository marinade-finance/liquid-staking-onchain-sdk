#![allow(unused)]
use super::*;
fn main() {
    pub fn config_validator_system(state: &impl Located<State>, extra_runs: u32) -> Instruction {
        let accounts = accounts::ConfigValidatorSystem {
            state: state.key(),
            manager_authority: state.as_ref().validator_system.manager_authority,
        }
        .to_account_metas(None);

        let data = instruction::ConfigValidatorSystem { extra_runs };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}