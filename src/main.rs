/*!
 CLI for terrad networks
*/
// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;
#![warn(missing_docs)]
use dotenv::dotenv;

use structopt::StructOpt;
mod bank;
mod contract;

mod auth;
mod distribution;
mod keys;
mod market;
mod oracle;
mod slashing;
mod staking;
mod tendermint;
mod validator;
mod wallet;

use anyhow::Result;

use crate::auth::{auth_cmd_parse, AuthCommand};
use crate::bank::{bank_cmd_parse, BankCommand};
use crate::contract::{contract_cmd_parse, ContractCommand};
use crate::distribution::{distribution_cmd_parse, DistributionCommand};
use crate::keys::{key_cmd_parse, KeysCommand};
use crate::market::{market_cmd_parse, MarketCommand};
use crate::oracle::{oracle_cmd_parse, OracleCommand};
use crate::slashing::{slashing_cmd_parse, SlashingCommand};
use crate::staking::{staking_cmd_parse, StakingCommand};
use crate::tendermint::{
    block_cmd_parse, validator_sets_cmd_parse, BlockCommand, ValidatorSetsCommand,
};
use crate::validator::{validator_cmd_parse, ValidatorCommand};
use crate::wallet::{wallet_cmd_parse, WalletCommand};
use terra_rust_api::core_types::Coin;
use terra_rust_api::{GasOptions, Terra};
use terra_rust_wallet::Wallet;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(StructOpt)]
struct Cli {
    #[structopt(
        name = "lcd",
        env = "TERRARUST_LCD",
        default_value = "https://tequila-lcd.terra.dev",
        short,
        long = "lcd-client-url",
        help = "https://lcd.terra.dev is main-net"
    )]
    // Terra cli Client daemon
    lcd: String,
    #[structopt(
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "tequila-0004",
        short,
        long = "chain",
        help = "tequila-0004 is testnet, columbus-4 is main-net"
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
        env = "TERRARUST_SEED_PHRASE",
        default_value = "",
        short,
        long = "seed",
        help = "the seed phrase to use with this private key"
    )]
    seed: String,
    #[structopt(
        name = "fees",
        default_value = "",
        short,
        long = "fees",
        help = "the fees to use. This will override gas parameters if specified."
    )]
    fees: String,
    #[structopt(
        name = "gas",
        default_value = "auto",
        long = "gas",
        help = "the gas amount to use 'auto' to estimate"
    )]
    gas: String,
    #[structopt(
        name = "gas-prices",
        env = "TERRARUST_GAS_PRICES",
        default_value = "180.0ukrw",
        long = "gas-prices",
        help = "the gas price to use to calculate fee. Format is NNNtoken eg. 1000uluna. note we only support a single price for now"
    )]
    gas_price: String,
    #[structopt(
        name = "gas-adjustment",
        default_value = "1.4",
        env = "TERRARUST_GAS_ADJUSTMENT",
        long = "gas-adjustment",
        help = "the adjustment to multiply the estimate to calculate the fee"
    )]
    gas_adjustment: f64,
    #[structopt(short, long, parse(from_flag))]
    debug: std::sync::atomic::AtomicBool,

    #[structopt(subcommand)]
    cmd: Command,
}
impl Cli {
    pub fn gas_opts(&self) -> Result<GasOptions> {
        let fees = Coin::parse(&self.fees)?;

        let gas_str = &self.gas;
        let (estimate_gas, gas) = if gas_str == "auto" {
            (true, None)
        } else {
            let g = &self.gas.parse::<u64>()?;
            (false, Some(*g))
        };
        let gas_price = Coin::parse(&self.gas_price)?;
        let gas_adjustment = Some(self.gas_adjustment);
        Ok(GasOptions {
            fees,
            estimate_gas,
            gas,
            gas_price,
            gas_adjustment,
        })
    }
}
#[derive(StructOpt)]
enum Command {
    /// Key Operations
    Keys(KeysCommand),
    /// validator operations
    Validator(ValidatorCommand),
    /// Market Operations
    Market(MarketCommand),
    /// Auth operations
    Auth(AuthCommand),
    /// wallet ops
    Wallet(WalletCommand),
    /// Bank Transactions
    Bank(BankCommand),
    /// Oracle Transactions
    Oracle(OracleCommand),
    /// Block commands
    Block(BlockCommand),
    /// Transaction Commands
    Tx(TxCommand),
    /// Slashing Commands
    Slashing(SlashingCommand),
    /// Staking Commands
    Staking(StakingCommand),
    /// Staking Commands
    Distribution(DistributionCommand),
    /// WASM Module / Smart Contract commands
    Contract(ContractCommand),
    /// Tendermint ValidatorSets commands
    ValidatorSets(ValidatorSetsCommand),
}

/// Input to the /txs/XXXX query
#[derive(StructOpt)]
pub struct TxCommand {
    #[structopt(name = "hash", help = "hash to inquire about")]
    /// The hash to inquire about
    hash: String,
}

async fn run() -> anyhow::Result<()> {
    let cli: Cli = Cli::from_args();

    let gas_opts: GasOptions = cli.gas_opts()?;
    let t = Terra::lcd_client(
        &cli.lcd,
        &cli.chain_id,
        &gas_opts,
        Some(cli.debug.into_inner()),
    )
    .await?;
    let seed: Option<&str> = if cli.seed.is_empty() {
        None
    } else {
        Some(&cli.seed)
    };
    let wallet = Wallet::create(&cli.wallet);
    match cli.cmd {
        Command::Keys(key_cmd) => key_cmd_parse(&t, &wallet, seed, key_cmd),
        Command::Bank(bank_cmd) => bank_cmd_parse(&t, &wallet, seed, bank_cmd).await,
        Command::Oracle(cmd) => oracle_cmd_parse(&t, &wallet, seed, cmd).await,
        Command::Validator(cmd) => validator_cmd_parse(&t, cmd).await,
        Command::Block(cmd) => block_cmd_parse(&t, cmd).await,
        Command::Contract(cmd) => contract_cmd_parse(&t, &wallet, seed, cmd).await,
        Command::Market(cmd) => market_cmd_parse(&t, &wallet, seed, cmd).await,

        Command::Tx(cmd) => {
            let resp = t.tx().get(&cmd.hash).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
            Ok(())
        }
        Command::Auth(auth_cmd) => auth_cmd_parse(&t, &wallet, seed, auth_cmd).await,
        Command::Wallet(wallet_cmd) => wallet_cmd_parse(&t, &wallet, seed, wallet_cmd),
        Command::Slashing(cmd) => slashing_cmd_parse(&t, &wallet, seed, cmd).await,
        Command::Staking(cmd) => staking_cmd_parse(&t, &wallet, seed, cmd).await,
        Command::Distribution(cmd) => distribution_cmd_parse(&t, &wallet, seed, cmd).await,
        Command::ValidatorSets(cmd) => validator_sets_cmd_parse(&t, cmd).await,
    }
}
#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv().ok();

    if let Err(ref err) = run().await {
        log::error!("{}", err);
        err.chain()
            .skip(1)
            .for_each(|cause| log::error!("because: {}", cause));

        // The backtrace is not always generated. Try to run this example
        // with `$env:RUST_BACKTRACE=1`.
        //    if let Some(backtrace) = e.backtrace() {
        //        log::debug!("backtrace: {:?}", backtrace);
        //    }

        ::std::process::exit(1);
    }
}
