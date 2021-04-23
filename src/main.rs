// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;
use dotenv::dotenv;
use log::{debug, error, info};
// use serde::{Deserialize, Serialize};
// use std::env;
use structopt::StructOpt;
mod bank;
mod errors;
mod keys;

use crate::errors::Result;

use crate::bank::{bank_cmd_parse, BankCommand};
use crate::keys::{key_cmd_parse, KeysCommand};
use terra_rust_api::core_types::Coin;
use terra_rust_api::Terra;

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        name = "lcd",
        env = "TERRARUST_LCD",
        default_value = "https://tequila-lcd.terra.dev",
        short,
        long = "lcd-client-url",
        help = "https://lcd.terra.dev is mainnet"
    )]
    // Terra cli Client daemon
    lcd: String,
    #[structopt(
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "tequila-0004",
        short,
        long = "chain",
        help = "tequilla-0004 is testnet, columbus-4 is mainnet"
    )]
    chain_id: String,
    // Wallet name
    #[structopt(
        name = "wallet",
        env = "TERRARUST_WALLET",
        default_value = "default",
        short,
        long = "wallet",
        help = "the default wallet to look for keys in"
    )]
    wallet: String,
    #[structopt(
        name = "seed",
        env = "TERRARUST_SEEDPHRASE",
        default_value = "",
        short,
        long = "seed",
        help = "the seed phrase to use with this private key"
    )]
    seed: String,
    #[structopt(subcommand)]
    cmd: Command,
}
#[derive(StructOpt)]
enum Command {
    /// Key Operations
    Keys(KeysCommand),
    /// validator operations
    Validator(Validator),
    /// Market Operations
    Market(Market),
    /// Auth operations
    Auth(Auth),
    /// wallet ops
    Wallets(Wallets),
    /// Bank Transactions
    Bank(BankCommand),
}
#[derive(StructOpt)]
enum Validator {
    #[structopt(name = "list")]
    // list all validators. Including Jailed ones
    List,
    #[structopt(name = "describe")]
    Describe {
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },
}
#[derive(StructOpt)]
enum Market {
    #[structopt(name = "swap")]
    Swap {
        #[structopt(name = "denom", help = "token symbol. remember we are uXXX not XXX")]
        denom: String,
        #[structopt(name = "amount", help = "the amount. remember we are uXXX not XXX")]
        amount: u64,
        #[structopt(name = "ask", help = "what to swap the amount into")]
        ask: String,
    },
}
#[derive(StructOpt)]
enum Auth {
    #[structopt(name = "account")]
    Account {
        #[structopt(name = "address", help = "the address to query")]
        address: String,
    },
}
#[derive(StructOpt)]
enum Wallets {
    #[structopt(name = "create", help = "create a wallet")]
    Create {
        #[allow(dead_code)]
        #[structopt(name = "name", help = "name of the wallet")]
        name: String,
    },
    #[structopt(name = "list", help = "List available wallets")]
    List,
    #[structopt(name = "delete", help = "delete a wallet")]
    Delete {
        #[structopt(name = "name", help = "name of the wallet")]
        #[allow(dead_code)]
        name: String,
    },
}

async fn run() -> Result<()> {
    let cli = Cli::from_args();
    let t = Terra::lcd_client(&cli.lcd, &cli.chain_id).await?;
    let seed: Option<&str> = if cli.seed == "" {
        None
    } else {
        Some(&cli.seed)
    };
    match cli.cmd {
        Command::Keys(key_cmd) => key_cmd_parse(&t, &cli.wallet, seed, key_cmd),
        Command::Bank(bank_cmd) => bank_cmd_parse(&t, &cli.wallet, seed, bank_cmd).await,
        Command::Validator(val_cmd) => match val_cmd {
            Validator::List => {
                let list = t.staking().validators().await?;
                if !list.result.is_empty() {
                    let v1 = list.result.get(0).unwrap();
                    println!("{:#?}", v1);
                }
                Ok(())
            }
            Validator::Describe { validator } => {
                let v = t.staking().validator(&validator).await?;
                println!("{:#?}", v);
                Ok(())
            }
        },
        Command::Market(market_cmd) => match market_cmd {
            Market::Swap { denom, ask, amount } => {
                let coin = Coin::create(&denom, amount);
                let sw = t.market().swap(&coin, &ask).await?;

                println!("{:#?}", sw);
                Ok(())
            }
        },
        Command::Auth(auth_cmd) => match auth_cmd {
            Auth::Account { address } => {
                let sw = t.auth().account(&address).await?;

                println!("{:#?}", sw);
                Ok(())
            }
        },
        Command::Wallets(wallet_cmd) => match wallet_cmd {
            Wallets::Create { .. } => {
                todo!()
            }
            Wallets::List => {
                todo!()
            }
            Wallets::Delete { .. } => {
                todo!()
            }
        },
    }
}
#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();
    if let Err(ref e) = run().await {
        error!("error: {}", e);

        //  $env:RUST_LOG="output_log=info"
        for e in e.iter().skip(1) {
            info!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            debug!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
