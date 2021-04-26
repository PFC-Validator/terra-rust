// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
#![allow(missing_docs)]
/*!
This crate provides an interface into the Terra LCD HTTP service.
# PFC

This work is sponsored by the PFC (Pretty Freaking Cool) Validator,
feel free to delegate to the [PFC](https://station.terra.money/validator/terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q) validator.

It will help defray the costs.

# Warning
This uses crytpographic routines that have not gone through any security audit.

The manner which it stores private keys may be unsecure/subject to hacks, and if you use it, may put the assets behind those keys at risk.

This is ALPHA software.

# Usage
```toml
[dependencies]
terra-rust-api="0.1"


```
*/
mod client;
/// Error Messages
pub mod errors;
mod keys;
/// definitions of the different type of Messages we have implemented
pub mod messages;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate erased_serde;
extern crate rustc_serialize;
//
//#[macro_use]
extern crate error_chain;
extern crate reqwest;

pub use client::{auth_types, client_types, core_types, market_types, staking_types};
pub use client::{GasOptions, Terra};
pub use keys::{PrivateKey, PublicKey};
pub use messages::bank;
