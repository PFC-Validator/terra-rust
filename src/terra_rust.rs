/*!
 CLI for terrad networks
*/
// (Buf) Uncomment these lines to have the output buffered, this can provide
// better performance but is not always intuitive behaviour.
// use std::io::BufWriter;
#![warn(missing_docs)]
use clap::{Parser, Subcommand};
use dotenv::dotenv;
mod bank;
mod contract;

mod auth;
mod cli;
mod code;
mod distribution;
mod fcd;
mod keys;
mod market;
mod oracle;
mod rpc;
mod slashing;
mod staking;
mod tendermint;
mod tx;
mod validator;
mod wallet;
mod wasm;

use crate::auth::AuthCommand;
use crate::bank::BankCommand;
use crate::code::CodeCommand;
use crate::contract::ContractCommand;
use crate::distribution::DistributionCommand;
use crate::fcd::FCDCommand;
use crate::keys::KeysCommand;
use crate::market::MarketCommand;
use crate::oracle::OracleCommand;
use crate::rpc::RPCCommand;
use crate::slashing::SlashingCommand;
use crate::staking::StakingCommand;
use crate::tendermint::{BlockCommand, ValidatorSetsCommand};
use crate::tx::TxCommand;
use crate::validator::ValidatorCommand;
use crate::wallet::WalletCommand;
use crate::wasm::WasmCommand;
use terra_rust_api::{GasOptions, Terra};
use terra_rust_wallet::Wallet;

/// VERSION number of package
///
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(Subcommand)]
#[allow(clippy::upper_case_acronyms)]
enum Command {
    Keys(KeysCommand),
    Validator(ValidatorCommand),
    Market(MarketCommand),
    Auth(AuthCommand),
    Wallet(WalletCommand),
    Bank(BankCommand),
    Oracle(OracleCommand),
    Block(BlockCommand),
    Tx(TxCommand),
    Slashing(SlashingCommand),
    Staking(StakingCommand),
    Distribution(DistributionCommand),
    Contract(ContractCommand),
    ValidatorSets(ValidatorSetsCommand),
    RPC(RPCCommand),
    FCD(FCDCommand),
    WASM(WasmCommand),
    CODE(CodeCommand),
}

async fn run() -> anyhow::Result<()> {
    let cli = cli::Cli::<Command>::parse();

    let gas_opts: GasOptions = cli.gas_opts().await?;
    let t = Terra::lcd_client(
        &cli.lcd,
        &cli.chain_id,
        &gas_opts,
        Some(cli.debug.into_inner()),
    );
    let seed: Option<&str> = if cli.seed.is_empty() {
        None
    } else {
        Some(&cli.seed)
    };
    let wallet = Wallet::create(&cli.wallet);
    match cli.cmd {
        Command::Keys(key_cmd) => key_cmd.parse(&wallet, seed),
        Command::Bank(bank_cmd) => bank_cmd.parse(&t, &wallet, seed).await,
        Command::Oracle(cmd) => cmd.parse(&t, &wallet, seed).await,
        Command::Validator(cmd) => cmd.parse(&t, &wallet, seed).await,
        Command::Block(cmd) => cmd.parse(&t).await,
        Command::Contract(cmd) => cmd.parse(&t).await,
        Command::Market(cmd) => cmd.parse(&t, &wallet, seed).await,
        Command::Tx(cmd) => cmd.parse(&t).await,
        Command::Auth(auth_cmd) => auth_cmd.parse(&t, &wallet, seed).await,
        Command::Wallet(cmd) => cmd.parse(&wallet),
        Command::Slashing(cmd) => cmd.parse(&t, &wallet, seed).await,
        Command::Staking(cmd) => cmd.parse(&t, &wallet, seed).await,
        Command::Distribution(cmd) => cmd.parse(&t, &wallet, seed).await,
        Command::ValidatorSets(cmd) => cmd.parse(&t).await,
        Command::RPC(cmd) => cmd.parse(&t).await,
        Command::FCD(cmd) => cmd.parse(&t, &cli.fcd).await,
        Command::WASM(cmd) => cmd.parse(&t).await,
        Command::CODE(cmd) => cmd.parse(&t, &wallet, seed).await,
    }
}
#[tokio::main]
async fn main() {
    dotenv().ok();
    env_logger::init();

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
