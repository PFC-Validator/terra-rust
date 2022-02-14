use anyhow::Result;
use clap::{Parser, Subcommand};
use dotenv::dotenv;
use rust_decimal::Decimal;
use secp256k1::Secp256k1;
use serde::Serialize;
use terra_rust::Cli;
use terra_rust_api::core_types::Coin;
use terra_rust_api::terra_u64_format;
use terra_rust_api::{GasOptions, Message, MsgExecuteContract, Terra};
use terra_rust_wallet::Wallet;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(Subcommand)]
pub enum Swap {
    Swap(SwapCmd),
}
#[derive(Parser, Debug)]
pub struct SwapCmd {
    #[clap(
        name = "contract",
        help = "the contract",
        long = "contract",
        env = "TERRARUST_CONTRACT"
    )]
    pub contract: String,
    #[clap(
        name = "sender",
        long = "sender",
        help = "the sender account",
        env = "TERRARUST_SENDER"
    )]
    pub sender: String,
    #[clap(name = "coins", long = "coins")]
    pub coins: Option<String>,

    #[clap(
        name = "belief-price",
        help = "exchange rate/price you believe is true"
    )]
    belief_price: Decimal,
    #[clap(name = "amount", help = "amount")]
    amount: u64,
    #[clap(name = "denom", help = "Denomination")]
    denom: String,
    #[clap(
        name = "max-spread",
        long = "max-spread",
        help = "exchange rate/price you believe is true"
    )]
    max_spread: Decimal,
}
async fn run() -> Result<()> {
    let cli = Cli::<Swap>::parse();
    let gas_opts: GasOptions = cli.gas_opts().await?;
    let terra = Terra::lcd_client(&cli.lcd, &cli.chain_id, &gas_opts, None);
    let secp = Secp256k1::new();
    let wallet = Wallet::create(&cli.wallet);

    let seed: Option<&str> = if cli.seed.is_empty() {
        None
    } else {
        Some(&cli.seed)
    };
    match cli.cmd {
        Swap::Swap(swap) => {
            let coins: Vec<Coin> = if let Some(coin_str) = swap.coins {
                Coin::parse_coins(&coin_str)?
            } else {
                vec![]
            };

            let from_key = wallet.get_private_key(&secp, &swap.sender, seed)?;

            let from_public_key = from_key.public_key(&secp);

            let store_message = MsgSwap::create(
                &from_public_key.account()?,
                &swap.contract,
                swap.belief_price,
                swap.max_spread,
                swap.amount,
                &swap.denom,
                &coins,
            )?;
            let messages: Vec<Message> = vec![store_message];

            let json = serde_json::to_string(&messages)?;
            log::info!("Message:\n{}", json);
            let resp = terra
                .submit_transaction_sync(
                    &secp,
                    &from_key,
                    messages,
                    Some(format!(
                        "PFC-{}/{}",
                        NAME.unwrap_or("TERRARUST"),
                        VERSION.unwrap_or("DEV")
                    )),
                )
                .await?;

            let hash = resp.txhash;
            log::info!("{}", hash);
        }
    }

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
