#![allow(unused)]
use super::*;
fn main() {
    pub fn deactivate_stake(
        state: &impl Located<State>,
        stake_account: Pubkey,
        split_stake_account: Pubkey,
        split_stake_rent_payer: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> Instruction {
        let accounts = accounts::DeactivateStake {
            state: state.key(),
            reserve_pda: state.reserve_address(),
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            stake_list: *state.as_ref().stake_system.stake_list_address(),
            stake_account,
            stake_deposit_authority: state.stake_deposit_authority(),
            split_stake_account,
            split_stake_rent_payer,

            clock: clock::ID,
            rent: rent::ID,
            epoch_schedule: epoch_schedule::ID,
            stake_history: stake_history::id(),

            system_program: system_program::ID,
            stake_program: stake::program::ID,
        }
        .to_account_metas(None);

        let data = instruction::DeactivateStake {
            stake_index,
            validator_index,
        };

        Instruction {
            program_id: marinade_finance::ID,
            accounts,
            data: data.data(),
        }
    }
}