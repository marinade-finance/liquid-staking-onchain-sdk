#![allow(unused_imports)]

use anchor_lang::prelude::*;
use std::{convert::TryFrom, fmt::Display, str::FromStr};

//-----------------------------------------------------
#[derive(
    Clone, Copy, Debug, Default, AnchorSerialize, AnchorDeserialize, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct Fee {
    pub basis_points: u32,
}

// impl Display for Fee {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}%", self.basis_points as f32 / 100.0)
//     }
// }

// impl Fee {
//     pub fn from_basis_points(basis_points: u32) -> Self {
//         Self { basis_points }
//     }

//     /// generic check, capped Fee
//     pub fn check_max(&self, max_basis_points: u32) -> Result<(), CommonError> {
//         if self.basis_points > max_basis_points {
//             Err(CommonError::FeeTooHigh)
//         } else {
//             Ok(())
//         }
//     }
//     /// base check, Fee <= 100%
//     pub fn check(&self) -> Result<(), CommonError> {
//         self.check_max(10_000)
//     }

//     pub fn apply(&self, lamports: u64) -> u64 {
//         // LMT no error possible
//         (lamports as u128 * self.basis_points as u128 / 10_000_u128) as u64
//     }
// }

// impl TryFrom<f64> for Fee {
//     type Error = CommonError;

//     fn try_from(n: f64) -> Result<Self, CommonError> {
//         let basis_points_i = (n * 100.0).floor() as i64; // 4.5% => 450 basis_points
//         let basis_points =
//             u32::try_from(basis_points_i).map_err(|_| CommonError::CalculationFailure)?;
//         let fee = Fee::from_basis_points(basis_points);
//         fee.check()?;
//         Ok(fee)
//     }
// }

// impl FromStr for Fee {
//     type Err = CommonError; // TODO: better error

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         f64::try_into(s.parse().map_err(|_| CommonError::CalculationFailure)?)
//     }
// }
