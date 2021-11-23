use anyhow::Result;
use dotenv::dotenv;
use secp256k1::{All, Secp256k1};
use std::path::Path;
use terra_rust_api::core_types::Coin;
use terra_rust_api::{GasOptions, Message, PrivateKey, Terra};

use structopt::StructOpt;
use terra_rust_api::client::tx_types::TXResultSync;
use terra_rust_api::messages::wasm::MsgInstantiateContract;
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
        default_value = "https://lcd.terra.dev",
        short,
        long = "lcd-client-url",
        help = "https://lcd.terra.dev is main-net, https://bombay-lcd.terra.dev"
    )]
    // Terra cli Client daemon
    lcd: String,
    #[structopt(
        name = "fcd",
        env = "TERRARUST_FCD",
        default_value = "https://fcd.terra.dev",
        long = "fcd-client-url",
        help = "https://fcd.terra.dev is main-net. currently only used to fetch gas prices"
    )]
    // Terra cli Client daemon
    fcd: String,
    #[structopt(
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "columbus-5",
        short,
        long = "chain",
        help = "bombay-12 is testnet, columbus-5 is main-net"
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
        default_value = "auto",
        long = "gas-prices",
        help = "the gas price to use to calculate fee. Format is NNNtoken eg. 1000uluna. note we only support a single price for now. if auto. it will use FCD"
    )]
    gas_price: String,
    #[structopt(
        name = "gas-denom",
        env = "TERRARUST_GAS_DENOM",
        default_value = "ukrw",
        long = "gas-denom",
        help = "the denomination/currency to use to pay fee. Format is uXXXX."
    )]
    gas_price_denom: String,
    #[structopt(
        name = "gas-adjustment",
        default_value = "1.4",
        env = "TERRARUST_GAS_ADJUSTMENT",
        long = "gas-adjustment",
        help = "the adjustment to multiply the estimate to calculate the fee"
    )]
    gas_adjustment: f64,
    #[structopt(name = "sender", help = "the sender account")]
    sender: String,
    #[structopt(name = "code-id", help = "code id")]
    code_id: u64,
    #[structopt(
        name = "admin",
        long = "admin",
        help = "the admin account",
        default_value = ""
    )]
    admin: String,
    #[structopt(
        name = "coins",
        long = "coins",
        help = "initial coins",
        default_value = ""
    )]
    coins: String,

    #[structopt(name = "json", help = "the json init file.")]
    json: String,
}
impl Cli {
    pub async fn gas_opts(&self) -> Result<GasOptions> {
        if self.gas_price == "auto" {
            //            let terra = Terra::lcd_client_no_tx(&self.lcd, &self.chain_id).await?;
            //      let fcd = terra.fcd(&self.fcd);
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

async fn run() -> anyhow::Result<()> {
    let cli: Cli = Cli::from_args();

    let gas_opts: GasOptions = cli.gas_opts().await?;
    let terra = Terra::lcd_client(&cli.lcd, &cli.chain_id, &gas_opts, None);
    let secp = Secp256k1::new();
    let wallet = Wallet::create(&cli.wallet);

    let seed: Option<&str> = if cli.seed.is_empty() {
        None
    } else {
        Some(&cli.seed)
    };
    let admin: Option<String> = if cli.admin.is_empty() {
        None
    } else {
        if cli.admin.starts_with("terra1") {
            Some(cli.admin)
        } else {
            let admin_key = wallet.get_public_key(&secp, &cli.admin, seed)?;
            let admin_account = admin_key.account()?;
            Some(admin_account.clone())
        }
    };
    let coins: Vec<Coin> = if cli.coins.is_empty() {
        vec![]
    } else {
        Coin::parse_coins(&cli.coins)?
    };

    let from_key = wallet.get_private_key(&secp, &cli.sender, seed)?;

    let resp = init_code(
        &terra,
        &secp,
        &from_key,
        admin,
        cli.code_id,
        &Path::new(&cli.json),
        coins,
    )
    .await?;
    log::info!("{:?}", &resp);

    Ok(())
}
async fn init_code<'a>(
    terra: &'a Terra,
    secp: &Secp256k1<All>,
    from_key: &PrivateKey,
    admin: Option<String>,
    code_id: u64,
    init_file: &Path,
    init_coins: Vec<Coin>,
) -> anyhow::Result<TXResultSync> {
    let sender = from_key.public_key(secp).account()?;
    let init_message =
        MsgInstantiateContract::create_from_file(&sender, admin, code_id, init_file, init_coins)?;
    log::info!("INIT = {}", serde_json::to_string_pretty(&init_message)?);
    let init_messages: Vec<Message> = vec![init_message];
    let resp = terra
        .submit_transaction_sync(
            &secp,
            &from_key,
            init_messages,
            Some(format!(
                "PFC-{}/{}",
                NAME.unwrap_or("TERRARUST"),
                VERSION.unwrap_or("DEV")
            )),
        )
        .await?;

    log::info!("{:?}", &resp);
    Ok(resp)
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
