use super::*;

/// Deposit SOL into Marinade and deliver mSOL to the user
/// This can be viewed also as: SWAP SOL->mSOL with zero fee
/// Before the end of the epoch, the bot will stake or unstake based on the delta from deposit vs unstakes
pub fn deposit(
    state: &impl Located<State>,
    transfer_from: Pubkey,
    mint_to: Pubkey,
    lamports: u64,
) -> Instruction {
    let accounts = accounts::Deposit {
        state: state.key(),
        msol_mint: state.as_ref().msol_mint,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        liq_pool_msol_leg_authority: state.liq_pool_msol_leg_authority(),
        reserve_pda: state.reserve_address(),
        transfer_from,
        mint_to,
        msol_mint_authority: state.msol_mint_authority(),
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::Deposit { lamports };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}