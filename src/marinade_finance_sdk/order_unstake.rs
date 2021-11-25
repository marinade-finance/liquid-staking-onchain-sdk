#![allow(unused)]
use super::*;
fn main() {
    pub fn order_unstake(
        state: &impl Located<State>,
        burn_msol_from: Pubkey,
        burn_msol_authority: Pubkey, // delegated or owner
        msol_amount: u64,
        new_ticket_account: Pubkey,
    ) -> Instruction {
        let accounts = accounts::OrderUnstake {
            state: state.key(),
            msol_mint: state.as_ref().msol_mint,
            burn_msol_from,
            burn_msol_authority,
            new_ticket_account,
            token_program: token::ID,
            clock: clock::ID,
            rent: rent::ID,
        }
        .to_account_metas(None);

        let data = instruction::OrderUnstake { msol_amount };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}