#![allow(unused)]
use super::*;
fn main() {
    pub fn change_authority(state: &impl Located<State>, data: ChangeAuthorityData) -> Instruction {
        let accounts = accounts::ChangeAuthority {
            state: state.key(),
            admin_authority: state.as_ref().admin_authority,
        }
        .to_account_metas(None);

        let data = instruction::ChangeAuthority { data };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}