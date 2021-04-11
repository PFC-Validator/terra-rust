// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;
use dotenv::dotenv;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::env;
use structopt::StructOpt;

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
    #[structopt(subcommand)]
    cmd: Command,
}
#[derive(StructOpt)]
enum Command {
    #[structopt(name = "keys")]
    // Key Operations
    Keys(Keys),
    // validator operations
    Validator(Validator),
    // Market Operations
    Market(Market),
}
#[derive(StructOpt)]
enum Keys {
    #[structopt(name = "parse")]
    Parse {
        #[structopt(name = "hex")]
        // hex public key to convert
        hex: String,
    },
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
mod errors;
use crate::errors::Result;

use terra_rust_api::core_types::Coin;
use terra_rust_api::Terra;

async fn run() -> Result<bool> {
    let cli = Cli::from_args();
    let t = Terra::lcd_client(&cli.lcd, &cli.chain_id).await?;
    match cli.cmd {
        Command::Keys(keycmd) => match keycmd {
            Keys::Parse { hex } => {
                println!("{}", hex)
            }
        },
        Command::Validator(valcmd) => match valcmd {
            Validator::List => {
                let list = t.staking().validators().await?;
                if !list.result.is_empty() {
                    let v1 = list.result.get(0).unwrap();
                    println!("{:#?}", v1);
                }
            }
            Validator::Describe { validator } => {
                let v = t.staking().validator(&validator).await?;
                println!("{:#?}", v);
            }
        },
        Command::Market(marketcmd) => match marketcmd {
            Market::Swap { denom, ask, amount } => {
                let coin = Coin::create(&denom, amount);
                let sw = t.market().swap(&coin, &ask).await?;

                println!("{:#?}", sw);
            }
        },
    }

    Ok(true)
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
