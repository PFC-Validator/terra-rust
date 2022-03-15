use dotenv::dotenv;

use clap::Arg;
use terra_rust_cli::cli_helpers;

/// VERSION number of package
pub const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// NAME of package
pub const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

async fn run() -> anyhow::Result<()> {
    let cli = cli_helpers::gen_cli_read_only("terra query", "terra-query")
        .arg(
            Arg::new("contract")
                .long("contract")
                .value_name("contract")
                .takes_value(true)
                .env("TERRARUST_CONTRACT")
                .help("the contract address"),
        )
        .arg(Arg::new("json").takes_value(true).value_name("json"))
        .get_matches();
    let terra = cli_helpers::lcd_no_tx_from_args(&cli)?;
    let contract = cli_helpers::get_arg_value(&cli, "contract")?;
    let json_str = cli_helpers::get_arg_value(&cli, "json")?;
    let secp = secp256k1::Secp256k1::new();
    let wallet = cli_helpers::wallet_opt_from_args(&cli);
    let seed = cli_helpers::seed_from_args(&cli);
    let json: serde_json::Value =
        cli_helpers::get_json_block_expanded(json_str, None, &secp, wallet, seed)?;

    //    let json: serde_json::Value = serde_json::from_str(json_str)?;

    let qry = terra
        .wasm()
        .query::<serde_json::Value>(contract, &json.to_string())
        .await?;
    println!("{}", qry);

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
