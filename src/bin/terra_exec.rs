use anyhow::Result;
use clap::{Arg, ArgMatches};
use dotenv::dotenv;
use secp256k1::Secp256k1;
use terra_rust_api::core_types::Coin;
use terra_rust_api::{Message, MsgExecuteContract};
use terra_rust_cli::cli_helpers;
//use tokio::runtime::Handle;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

async fn run() -> Result<()> {
    let app = cli_helpers::gen_cli("terra exec", "terra-exec").args(&[
        Arg::new("contract")
            .long("contract")
            .takes_value(true)
            .value_name("contract")
            .env("TERRARUST_CONTRACT")
            .required(true)
            .help("the contract address"),
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
    ]);

    Ok(run_it(&app.get_matches()).await?)
}
pub async fn run_it(cli: &ArgMatches) -> Result<()> {
    //let cli = app.get_matches();

    //  let wallet = cli_helpers::wallet_from_args(&cli)?;
    let terra = cli_helpers::lcd_from_args(cli).await?;

    let json_str = cli.value_of("json").expect("json be in the CLI");
    //let json_str = cli.value_of("json").expect("json be in the CLI");
    let coins_str = cli.value_of("coins");
    let contract = cli.value_of("contract").expect("Need a contract");

    let json: serde_json::Value = cli_helpers::get_json_block(json_str)?;

    let secp = Secp256k1::new();

    let from_key = cli_helpers::get_private_key(&secp, cli)?;
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
    if terra.chain_id.contains("bombay") {
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
        eprintln!("{}", err);
        //log::error!("{}", err);
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
