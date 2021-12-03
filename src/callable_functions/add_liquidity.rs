use super::*;

/// Adds Liquidity to the mSOL/SOL Liquidity pool
/// The User only add SOL liquidity. There's NO need to add 50% SOL and 50% mSOL
/// Because the user adds only SOL to the pool,
/// the mSOL/SOL Liquidity pool is free from impermanent/divergent loss, meaning
/// that in no case the user can receive less SOL value than the value they put in
pub fn add_liquidity(
    state: &impl Located<State>,
    transfer_from: Pubkey,
    mint_to: Pubkey,
    lamports: u64,
) -> Instruction {
    let accounts = accounts::AddLiquidity {
        state: state.key(),
        lp_mint: state.as_ref().liq_pool.lp_mint,
        lp_mint_authority: state.lp_mint_authority(),
        // msol_mint: state.as_ref().msol_mint,
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        transfer_from,
        mint_to,
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::AddLiquidity { lamports };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}
