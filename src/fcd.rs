use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;
/// FCD commands
#[derive(Parser)]
pub struct FCDCommand {
    #[clap(subcommand)]
    command: FCDEnum,
}
#[derive(Subcommand)]
pub enum FCDEnum {
    #[clap(name = "gas-prices", about = "gas prices to use to calculate fees")]
    GasPrices,
}
impl FCDCommand {
    pub async fn parse(&self, terra: &Terra, fcd_url: &str) -> Result<()> {
        match &self.command {
            FCDEnum::GasPrices => {
                let resp = terra.fcd(fcd_url).gas_prices().await?;

                println!("{}", serde_json::to_string(&resp)?)
            }
        }
        Ok(())
    }
}
