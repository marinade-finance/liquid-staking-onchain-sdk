# Marinade Finance on-chain SDK

This Rust lib will simplify integrating marinade-finance/liquid-staking-program from your Solana program via CPI calls
## Usage
add to your .toml file
```
liquid-staking-onchain-sdk = { git = "https://github.com/marinade-finance/liquid-staking-onchain-sdk" }
```
Add to the begining of your lib.rs to use the functions:
```
use ::liquid_staking_onchain_sdk::*;
```

## Examples

### Read true mSOL/SOL price

Here's an example of an on-chain program reading Marinade state
https://github.com/marinade-finance/liquid-staking-referral-program/blob/c2e04d1d023a5c05d4abafc327f158ede2090371/programs/marinade-referral/src/instructions/liquid_unstake.rs#L42

Once you read Marinade state, you have `marinade_state.msolPrice: u64`, but that's mOSL price in SOL multiplied by 0x1_0000_0000 (shifted), so to obtain the f64 price you should do: `let msol_price: f64 = marinade_state.msolPrice as f64 / 0x1_0000_0000 as f64`, and then you've got the true mSOL/SOL price.

### How much SOL x amount of mSOL represents

You start with the previous example and let's assume you've an amount of mSOL-lamports, then:

`let SOL_lamports = (mSOL_lamports as u128 * marinade_state.msolPrice as u128 / 0x1_0000_0000 as u128) as u64`

Note: mSOL uses 9 decimals, as SOL.

### How much mSOL an amount of SOL represents

`let mSOL_lamports = (mSOL_lamports as u128 * 0x1_0000_0000 as u128 / marinade_state.msolPrice as u128) as u64`

