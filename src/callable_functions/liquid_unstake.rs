use super::*;

/// "Unstake Now!" functionality
/// Swaps mSOL->SOL in the liquidity pool, paying a fee
/// the fee can vary from 0.3% (normal) to 3% if the liquidity pool is depleted
/// It's similar to an AMM swap except:
///   a) The price is the true price of mSOL
///   b) the operation does not affect the mSOL price  
///   c) the fee is flat 0.3% unless you're making the pool go below target liquidity
pub fn liquid_unstake(
    state: &impl Located<State>,
    get_msol_from: Pubkey,
    get_msol_from_authority: Pubkey,
    transfer_sol_to: Pubkey,
    msol_amount: u64,
) -> Instruction {
    let accounts = accounts::LiquidUnstake {
        state: state.key(),
        msol_mint: state.as_ref().msol_mint,
        liq_pool_sol_leg_pda: state.liq_pool_sol_leg_address(),
        liq_pool_msol_leg: state.as_ref().liq_pool.msol_leg,
        get_msol_from,
        get_msol_from_authority,
        transfer_sol_to,
        treasury_msol_account: state.as_ref().treasury_msol_account,
        system_program: system_program::ID,
        token_program: token::ID,
    }
    .to_account_metas(None);

    let data = instruction::LiquidUnstake { msol_amount };

    Instruction {
        program_id: marinade_finance::ID,
        accounts,
        data: data.data(),
    }
}