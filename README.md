# Marinade Finance on-chain SDK

# DEPRECATED
## please refer to https://github.com/marinade-finance/liquid-staking-referral-program for an example on how to integrate Marinade from another on-chain program

------

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

