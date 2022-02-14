use anyhow::Result;
use clap::{Arg, Parser};
use terra_rust_api::core_types::Coin;
use terra_rust_api::GasOptions;

/// your terra swiss army knife
#[derive(Parser)]

pub struct Cli<T: clap::FromArgMatches + clap::Subcommand> {
    #[clap(
        name = "lcd",
        env = "TERRARUST_LCD",
        default_value = "https://lcd.terra.dev",
        short,
        long = "lcd-client-url",
        help = "https://lcd.terra.dev is main-net, https://bombay-lcd.terra.dev"
    )]
    // Terra cli Client daemon
    pub lcd: String,
    #[clap(
        name = "fcd",
        env = "TERRARUST_FCD",
        default_value = "https://fcd.terra.dev",
        long = "fcd-client-url",
        help = "https://fcd.terra.dev is main-net. currently only used to fetch gas prices"
    )]
    // Terra cli Client daemon
    pub fcd: String,
    #[clap(
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "columbus-5",
        short,
        long = "chain",
        help = "bombay-12 is testnet, columbus-5 is main-net"
    )]
    pub chain_id: String,
    // Wallet name
    #[clap(
        name = "wallet",
        env = "TERRARUST_WALLET",
        default_value = "default",
        short,
        long = "wallet",
        help = "the default wallet to look for keys in"
    )]
    pub wallet: String,
    #[clap(
        name = "seed",
        env = "TERRARUST_SEED_PHRASE",
        default_value = "",
        short,
        long = "seed",
        help = "the seed phrase to use with this private key"
    )]
    pub seed: String,
    #[clap(
        name = "fees",
        default_value = "",
        short,
        long = "fees",
        help = "the fees to use. This will override gas parameters if specified."
    )]
    pub fees: String,
    #[clap(
        name = "gas",
        default_value = "auto",
        long = "gas",
        help = "the gas amount to use 'auto' to estimate"
    )]
    pub gas: String,
    #[clap(
        name = "gas-prices",
        env = "TERRARUST_GAS_PRICES",
        default_value = "auto",
        long = "gas-prices",
        help = "the gas price to use to calculate fee. Format is NNNtoken eg. 1000uluna. note we only support a single price for now. if auto. it will use FCD"
    )]
    pub gas_price: String,
    #[clap(
        name = "gas-denom",
        env = "TERRARUST_GAS_DENOM",
        default_value = "ukrw",
        long = "gas-denom",
        help = "the denomination/currency to use to pay fee. Format is uXXXX."
    )]
    pub gas_price_denom: String,
    #[clap(
        name = "gas-adjustment",
        default_value = "1.4",
        env = "TERRARUST_GAS_ADJUSTMENT",
        long = "gas-adjustment",
        help = "the adjustment to multiply the estimate to calculate the fee"
    )]
    pub gas_adjustment: f64,
    #[clap(short, long, parse(from_flag))]
    pub debug: std::sync::atomic::AtomicBool,

    #[clap(subcommand)]
    pub cmd: T,
}
impl<T: clap::FromArgMatches + clap::Subcommand> Cli<T> {
    pub async fn gas_opts(&self) -> Result<GasOptions> {
        if self.gas_price == "auto" {
            let client = reqwest::Client::new();
            let gas_opts = GasOptions::create_with_fcd(
                &client,
                &self.fcd,
                &self.gas_price_denom,
                self.gas_adjustment,
            )
            .await?;
            if let Some(gas_price) = &gas_opts.gas_price {
                log::info!("Using Gas price of {}", gas_price);
            }

            Ok(gas_opts)
        } else {
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
}
#[allow(dead_code)]
pub fn gen_cli_read_only<'a>(app_name: &'a str, bin_name: &'a str) -> clap::App<'a> {
    clap::App::new(app_name)
        .bin_name(bin_name)
        .arg(
            Arg::new("lcd")
                .long("lcd")
                .value_name("lcd")
                .takes_value(true)
                .env("TERRARUST_LCD")
                .default_value("https://lcd.terra.dev")
                .help("https://lcd.terra.dev is main-net, https://bombay-lcd.terra.dev"),
        )
        .arg(
            Arg::new("chain")
                .long("chain")
                .takes_value(true)
                .value_name("chain")
                .env("TERRARUST_CHAIN")
                .default_value("columbus-5")
                .help("bombay-12 is testnet, columbus-5 is main-net"),
        )
}
