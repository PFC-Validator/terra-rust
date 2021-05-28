# Terra Rust

This is a WIP.

No security audit has been performed.

There are currently 3 interesting things
 
* [Terra-Rust](https://github.com/PFC-Validator/terra-rust/blob/main/src/main.rs) A cross-platform CLI tool
* [Terra-Rust-API](https://crates.io/crates/terra-rust-api) an API you can integrate into your own code
* [Terra-Rust-Wallet](https://crates.io/crates/terra-rust-wallet) a secure OS agnostic wallet, using [keyring](https://crates.io/crates/keyring)

## Randomness
The API is currently using random numbers via     
`
let mut rng = rand::thread_rng();
`
## Disclaimer

This may steal your money.

This is not investment advice.

Do you own research

## Why?

I built this for 2 main reasons.
1. There was no easy way for me to get the default terra-cli to work on Windows
1. I wanted a RUST api to use in other things. The CLI is just cream on top.

# Environment Variables
some things are used often and repeatedly, so we decided to use environment variables.

**TERRARUST_LCD** sets the LCD URL. e.g. https://tequila-lcd.terra.dev 

**TERRARUST_CHAIN** set the CHAIN to use e.g. tequila-0004

**TERRARUST_SEED_PHRASE** the passphrase used in combination with the 24-words to generate the private key

**TERRARUST_WALLET** the default wallet to use

**TERRARUST_GAS_PRICES** the gas price to use. e.g. 50ukrw

**TERRARUST_GAS_ADJUSTMENT** the gas adjustment multiplier to use

you can also set these in a file called '.env' if you prefer
# Documentation
* [API docs](https://docs.rs/terra-rust-api) are available here
* [Wallet docs](https://docs.rs/terra-rust-wallet) 
on first install you may want to
```
$ terra-rust wallet create default
```

# Help ?
```
$ terra-rust --help
```
If you think this was useful, feel free to delegate to the [PFC](https://station.terra.money/validator/terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q) validator. It will help defray the costs.

[PFC](https://twitter.com/PFC_Validator) - Terra/Luna is Pretty Freaking Cool right... feel free to drop me a line 

# Contribute
Feel free to submit patches/comments/Pull Requests.

We have also set up a [Discord](https://discord.gg/zKVWs4HhJD) channel to discuss this, and other PFC things
