#![cfg_attr(not(debug_assertions), deny(warnings))]

//! Marinade Program SDK
pub use ::marinade_finance; // reexport contract crate

use ::marinade_finance::*;
use anchor_lang::solana_program::{
    instruction::Instruction,
    pubkey::Pubkey,
    stake, system_program,
    sysvar::{clock, rent},
};
/// instruction helpers to be used by:
/// * other on-chain programs
/// * cli tools
/// * integration tests
use anchor_lang::{InstructionData, ToAccountMetas};
use anchor_spl::token;
use marinade_finance::{
    liq_pool::LiqPoolHelpers,
    located::Located,
    state::StateHelpers,
    validator_system::ValidatorRecord,
};


//functions have been exported to the folder callable_functions
pub mod callable_functions;