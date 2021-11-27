#![allow(unused)]
use super::*;
fn main() {
    pub fn deposit_stake_account(
        state: &impl Located<State>,
        stake_account: Pubkey,
        stake_authority: Pubkey,
        mint_to: Pubkey,
        validator_index: u32,
        validator_vote: Pubkey,
        rent_payer: Pubkey,
    ) -> Instruction {
        let accounts = accounts::DepositStakeAccount {
            state: state.key(),
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            stake_list: *state.as_ref().stake_system.stake_list_address(),
            stake_account,
            stake_authority,
            duplication_flag: ValidatorRecord::find_duplication_flag(&state.key(), &validator_vote).0,
            rent_payer,
            msol_mint: state.as_ref().msol_mint,
            mint_to,
            msol_mint_authority: state.msol_mint_authority(),
            clock: clock::id(),
            rent: rent::id(),
            system_program: system_program::ID,
            token_program: token::ID,
            stake_program: stake::program::ID,
        }
        .to_account_metas(None);

        let data = instruction::DepositStakeAccount { validator_index };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}