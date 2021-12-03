use super::*;

/// Removes Liquidity from the mSOL/SOL Liquidity pool
/// User can receive SOL & mSOL in proportion of the current pool composition
/// In no case the user can receive less SOL value than the value they put in
/// The mSOL/SOL Liquidity pool is free from impermanent/divergent loss
pub fn remove_liquidity(
    state: &impl Located<State>,
    burn_from: Pubkey,
    burn_from_authority: Pubkey,
    transfer_sol_to: Pubkey,
    transfer_msol_to: Pubkey,
    tokens: u64,
) -> Instruction {
    let accounts = accounts::RemoveLiquidity {
        state: state.key(),
        lp_mint: state.as_ref().liq_pool.lp_mint,
        // msol_mint: state.as_ref().msol_mint,
        burn_from,
        burn_from_authority, //owner acc is also token owner
        transfer_sol_to,
        transfer_msol_to,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        liq_pool_msol_leg_authority: state.liq_pool_msol_leg_authority(),
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::RemoveLiquidity { tokens };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}
