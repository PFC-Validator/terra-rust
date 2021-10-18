use dotenv::dotenv;
use rust_decimal::Decimal;
use secp256k1::Secp256k1;
use serde::Serialize;
use structopt::StructOpt;
use terra_rust_api::core_types::Coin;
use terra_rust_api::terra_u64_format;
use terra_rust_api::{Message, MsgExecuteContract};
use terra_rust_wallet::Wallet;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

#[derive(StructOpt)]
struct Cli {
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

    #[structopt(name = "from", help = "the account to sign from")]
    from: String,
    #[structopt(name = "message", help = "The address of the contract")]
    message: String,
}

async fn run() -> anyhow::Result<()> {
    let cli: Cli = Cli::from_args();

    let secp = Secp256k1::new();
    let wallet = Wallet::create(&cli.wallet);

    let seed: Option<&str> = if cli.seed.is_empty() {
        None
    } else {
        Some(&cli.seed)
    };
    //let msg = r#"random/{"token_uri":"https://www.merriam-webster.com/dictionary/token4","image":null,"image_data":null,"external_url":null,"description":null,"name":null,"attributes":[{"display_type":null,"trait_type":"gender","value":"female"},{"display_type":null,"trait_type":"name","value":"James T. Kirk"}],"background_color":null,"animation_url":null,"youtube_url":null}"#;
    let msg = cli.message;
    let from_key = wallet.get_private_key(&secp, &cli.from, seed)?;
    // let signature = from_key.sign(&secp, &cli.message)?;
    let signature = from_key.sign(&secp, &msg)?;

    println!("Message={}", &msg);
    println!("Signature={}", signature.signature);
    println!("PubKeySig={}", signature.pub_key.value);

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
