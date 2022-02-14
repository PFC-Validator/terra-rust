use anyhow::Result;
use clap::Parser;
use terra_rust_api::Terra;

/// Block commands
#[derive(Parser)]
pub struct BlockCommand {
    #[clap(name = "height", default_value = "latest", help = "height (optional)")]
    height: String,
}
impl BlockCommand {
    pub async fn parse(self, terra: &Terra) -> Result<()> {
        let block = if self.height.to_lowercase().trim() == "latest" {
            terra.tendermint().blocks().await
        } else {
            let height: u64 = self.height.parse::<u64>()?;
            terra.tendermint().blocks_at_height(height).await
        }?;
        println!("{:#?}", block);
        Ok(())
    }
}
/// Tendermint ValidatorSets commands
#[derive(Parser)]
pub struct ValidatorSetsCommand {
    #[clap(name = "height", default_value = "latest", help = "height (optional)")]
    height: String,
    #[clap(name = "page", default_value = "0", help = "page (optional)")]
    page: usize,
    #[clap(name = "limit", default_value = "9999", help = "limit (optional)")]
    limit: usize,
}
impl ValidatorSetsCommand {
    pub async fn parse(self, terra: &Terra) -> Result<()> {
        let vset = if self.height.to_lowercase().trim() == "latest" {
            terra
                .tendermint()
                .validatorsets(self.page, self.limit)
                .await
        } else {
            let height: u64 = self.height.parse::<u64>()?;
            terra
                .tendermint()
                .validatorsets_at_height(height, self.page, self.limit)
                .await
        }?;
        println!("{:#?}", vset);
        Ok(())
    }
}
