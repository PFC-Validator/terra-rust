use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;

#[derive(StructOpt)]
pub enum FCDCommand {
    #[structopt(name = "gas-prices", about = "gas prices to use to calculate fees")]
    GasPrices,
}

pub async fn fcd_cmd_parse(terra: &Terra<'_>, fcd_url: &str, cmd: FCDCommand) -> Result<()> {
    match cmd {
        FCDCommand::GasPrices => {
            let resp = terra.fcd(fcd_url).gas_prices().await?;

            println!("{}", serde_json::to_string(&resp)?)
        }
    }
    Ok(())
}
