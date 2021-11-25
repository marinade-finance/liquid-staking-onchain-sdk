#![allow(unused)]
use super::*;
fn main() {
    pub fn claim(
        state: &impl Located<State>,
        ticket_account: Pubkey,
        transfer_sol_to: Pubkey,
    ) -> Instruction {
        let accounts = accounts::Claim {
            state: state.key(),
            reserve_pda: state.reserve_address(),
            ticket_account,
            transfer_sol_to,
            system_program: system_program::ID,
            clock: clock::ID,
        }
        .to_account_metas(None);

        let data = instruction::Claim {};

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}