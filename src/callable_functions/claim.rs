use super::*;

/// Claim instruction: a user claims a Ticket-account
/// This is done once tickets are due, meaning enough time has passed for the
/// bot to complete the unstake process and transfer the requested SOL to reserve_pda.
/// Checks that transfer request amount is less than total requested for unstake and then transfer SOL to beneficiary
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