use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
//use crate::errors::Result;

#[derive(StructOpt)]
pub struct BlockCommand {
    #[structopt(name = "height", default_value = "latest", help = "height (optional)")]
    height: String,
}
#[derive(StructOpt)]
pub struct ValidatorSetsCommand {
    #[structopt(name = "height", default_value = "latest", help = "height (optional)")]
    height: String,
    #[structopt(name = "page", default_value = "0", help = "page (optional)")]
    page: usize,
    #[structopt(name = "limit", default_value = "9999", help = "limit (optional)")]
    limit: usize,
}

pub async fn block_cmd_parse(terra: &Terra<'_>, cmd: BlockCommand) -> Result<()> {
    let block = if cmd.height.to_lowercase().trim() == "latest" {
        terra.tendermint().blocks().await
    } else {
        let height: u64 = cmd.height.parse::<u64>()?;
        terra.tendermint().blocks_at_height(height).await
    }?;
    println!("{:#?}", block);
    Ok(())
}

pub async fn validator_sets_cmd_parse(terra: &Terra<'_>, cmd: ValidatorSetsCommand) -> Result<()> {
    let vset = if cmd.height.to_lowercase().trim() == "latest" {
        terra.tendermint().validatorsets(cmd.page, cmd.limit).await
    } else {
        let height: u64 = cmd.height.parse::<u64>()?;
        terra
            .tendermint()
            .validatorsets_at_height(height, cmd.page, cmd.limit)
            .await
    }?;
    println!("{:#?}", vset);
    Ok(())
}
