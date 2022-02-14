use anyhow::Result;
use clap::Arg;
use dotenv::dotenv;
use secp256k1::Secp256k1;
use terra_rust_api::core_types::Coin;
use terra_rust_api::{GasOptions, Message, MsgExecuteContract, Terra};
use terra_rust_cli::{gas_opts, gen_cli, wallet_from_args};

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

async fn run() -> Result<()> {
    let cli = gen_cli("terra exec", "terra-exec")
        .args(&[
            Arg::new("contract")
                .long("contract")
                .takes_value(true)
                .value_name("contract")
                .env("TERRARUST_CONTRACT")
                .required(true)
                .help("the contract address"),
            Arg::new("sender")
                .long("sender")
                .takes_value(true)
                .value_name("sender")
                .env("TERRARUST_SENDER")
                .required(true)
                .help("the sender account"),
            Arg::new("coins")
                .long("coins")
                .takes_value(true)
                .value_name("coins")
                .required(false)
                .help("coins you want to send (optional)"),
            Arg::new("json")
                .takes_value(true)
                .value_name("json")
                .required(true)
                .help("json string"),
        ])
        .get_matches();

    let gas_opts: GasOptions = gas_opts(&cli).await?;
    println!("GAS OPTS {:?}", gas_opts);
    let lcd = cli.value_of("lcd").expect("lcd be in the CLI");
    let chain_id = cli.value_of("chain").expect("chain be in the CLI");
    let json_str = cli.value_of("json").expect("json be in the CLI");
    let seed = cli.value_of("seed");
    let sender = cli.value_of("sender").expect("Need someone to exec from");
    let coins_str = cli.value_of("coins");
    let contract = cli.value_of("contract").expect("Need a contract");

    let terra = Terra::lcd_client(lcd, chain_id, &gas_opts, None);

    let json: serde_json::Value = serde_json::from_str(json_str)?;

    let secp = Secp256k1::new();
    let wallet = wallet_from_args(&cli)?;

    let from_key = wallet.get_private_key(&secp, sender, seed)?;
    let from_public_key = from_key.public_key(&secp);

    let coins = if let Some(coins) = coins_str {
        Coin::parse_coins(coins)?
    } else {
        vec![]
    };

    let exec_message = MsgExecuteContract::create_from_value(
        &from_public_key.account()?,
        contract,
        &json,
        &coins,
    )?;
    let messages: Vec<Message> = vec![exec_message];

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

    log::debug!("{:?}", &resp.txhash);
    if chain_id.contains("bombay") {
        println!(
            "https://finder.extraterrestrial.money/testnet/tx/{}",
            resp.txhash
        );
    } else {
        println!(
            "https://finder.extraterrestrial.money/mainnet/tx/{}",
            resp.txhash
        );
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
