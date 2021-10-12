use anyhow::Result;
use bitcoin::secp256k1::Secp256k1;
use dotenv::dotenv;
use rust_decimal::Decimal;
use serde::Serialize;
use structopt::StructOpt;
use terra_rust_api::core_types::Coin;
use terra_rust_api::terra_u64_format;
use terra_rust_api::{GasOptions, Message, MsgExecuteContract, Terra};
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
    #[structopt(name = "contract", help = "The address of the contract")]
    contract: String,

    #[structopt(
        name = "coins",
        long = "coins",
        help = "initial coins",
        default_value = ""
    )]
    coins: String,
    #[structopt(
        name = "belief-price",
        help = "exchange rate/price you believe is true"
    )]
    belief_price: Decimal,
    #[structopt(name = "amount", help = "amount")]
    amount: u64,
    #[structopt(name = "denom", help = "Denomination")]
    denom: String,
    #[structopt(
        name = "max-spread",
        long = "max-spread",
        help = "exchange rate/price you believe is true"
    )]
    max_spread: Decimal,
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
    let terra = Terra::lcd_client(&cli.lcd, &cli.chain_id, &gas_opts, None).await?;
    let secp = Secp256k1::new();
    let wallet = Wallet::create(&cli.wallet);

    let seed: Option<&str> = if cli.seed.is_empty() {
        None
    } else {
        Some(&cli.seed)
    };

    let coins: Vec<Coin> = if cli.coins.is_empty() {
        vec![]
    } else {
        Coin::parse_coins(&cli.coins)?
    };

    let from_key = wallet.get_private_key(&secp, &cli.sender, seed)?;

    let from_public_key = from_key.public_key(&secp);

    let store_message = MsgSwap::create(
        &from_public_key.account()?,
        &cli.contract,
        cli.belief_price,
        cli.max_spread,
        cli.amount,
        &cli.denom,
        &coins,
    )?;
    let messages: Vec<Message> = vec![store_message];

    let json = serde_json::to_string(&messages)?;
    log::info!("Message:\n{}", json);
    let resp = terra
        .submit_transaction_sync(
            &secp,
            &from_key,
            &messages,
            Some(format!(
                "PFC-{}/{}",
                NAME.unwrap_or("TERRARUST"),
                VERSION.unwrap_or("DEV")
            )),
        )
        .await?;

    let hash = resp.txhash;
    log::info!("{}", hash);

    Ok(())
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

#[derive(Serialize, Debug)]
pub struct MsgSwapNativeToken {
    pub denom: String,
}
#[derive(Serialize, Debug)]
pub struct MsgSwapOfferInfo {
    pub native_token: MsgSwapNativeToken,
}
#[derive(Serialize, Debug)]
pub struct MsgSwapOfferAsset {
    #[serde(with = "terra_u64_format")]
    pub amount: u64,
    pub info: MsgSwapOfferInfo,
}
#[derive(Serialize, Debug)]
pub struct MsgSwapData {
    pub belief_price: Decimal,
    pub max_spread: Decimal,
    pub offer_asset: MsgSwapOfferAsset,
}
#[derive(Serialize, Debug)]
/// Message: Swap
pub struct MsgSwap {
    pub swap: MsgSwapData,
}

impl MsgSwap {
    pub fn create(
        sender: &str,
        contract: &str,
        belief_price: Decimal,
        max_spread: Decimal,
        amount: u64,
        denom: &str,
        coins: &Vec<Coin>,
    ) -> anyhow::Result<Message> {
        let offer_info = MsgSwapOfferInfo {
            native_token: MsgSwapNativeToken {
                denom: denom.to_string(),
            },
        };
        let offer_asset = MsgSwapOfferAsset {
            amount,
            info: offer_info,
        };

        let swap_data = MsgSwapData {
            belief_price,
            max_spread,
            offer_asset,
        };
        let swap = MsgSwap { swap: swap_data };
        let swap_json = serde_json::to_string(&swap)?;
        MsgExecuteContract::create_from_json(sender, contract, &swap_json, coins)
    }
}
