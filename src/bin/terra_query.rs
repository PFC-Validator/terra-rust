use dotenv::dotenv;
use terra_rust_api::Terra;

use structopt::StructOpt;

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
        name = "chain",
        env = "TERRARUST_CHAIN",
        default_value = "columbus-5",
        short,
        long = "chain",
        help = "bombay-12 is testnet, columbus-5 is main-net"
    )]
    chain_id: String,
    #[structopt(
        name = "contract",
        help = "the contract",
        long = "contract",
        env = "TERRARUST_CONTRACT"
    )]
    contract: String,

    #[structopt(name = "json")]
    json: String,
}
async fn run() -> anyhow::Result<()> {
    let cli: Cli = Cli::from_args();

    let terra = Terra::lcd_client_no_tx(&cli.lcd, &cli.chain_id);
    let json: serde_json::Value = serde_json::from_str(&cli.json)?;

    let qry = terra
        .wasm()
        .query::<serde_json::Value>(&cli.contract, &json.to_string())
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
