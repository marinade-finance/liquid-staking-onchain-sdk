#![allow(unused_imports)]

use anchor_lang::{prelude::*, solana_program::clock};

use crate::{constant::*, error::*, fees::Fee, instructions::*, states::*};

pub fn process_liquid_unstake(ctx: Context<LiquidUnstake>, msol_amount: u64) -> ProgramResult {
    ctx.accounts.state.liq_unstake_amount += msol_amount;
    ctx.accounts.state.liq_unstake_operations += 1;

    // TODO: cpi to marinade main program

    let current_time = clock::Clock::get().unwrap().unix_timestamp;
    let elapsed_time = current_time - ctx.accounts.state.last_transfer_time;

    if elapsed_time as u32 > ctx.accounts.state.transfer_duration {
        // TODO: transfer shared mSOL to partner

        // Clears all accumulators and sets “Last transfer to partner timestamp“
        ctx.accounts.state.last_transfer_time = current_time;
        ctx.accounts.state.deposit_sol_amount += 0;
        ctx.accounts.state.liq_unstake_amount += 0;
        ctx.accounts.state.liq_unstake_operations += 0;
    }

    Ok(())
}
