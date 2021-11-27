#![allow(unused)]
use super::*;
fn main() {
    pub fn config_marinade(state: &impl Located<State>, params: ConfigMarinadeParams) -> Instruction {
        let accounts = accounts::ConfigMarinade {
            state: state.key(),
            admin_authority: state.as_ref().admin_authority,
        }
        .to_account_metas(None);

        let data = instruction::ConfigMarinade { params };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}