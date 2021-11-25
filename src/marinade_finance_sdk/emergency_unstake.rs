#![allow(unused)]
use super::*;
fn main() {
    pub fn emergency_unstake(
        state: &impl Located<State>,
        stake_account: Pubkey,
        stake_index: u32,
        validator_index: u32,
    ) -> Instruction {
        let accounts = accounts::EmergencyUnstake {
            state: state.key(),
            validator_manager_authority: state.as_ref().validator_system.manager_authority,
            validator_list: *state.as_ref().validator_system.validator_list_address(),
            stake_list: *state.as_ref().stake_system.stake_list_address(),
            stake_account,
            stake_deposit_authority: state.stake_deposit_authority(),

            clock: clock::ID,

            stake_program: stake::program::ID,
        }
        .to_account_metas(None);

        let data = instruction::EmergencyUnstake {
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