use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
//use crate::errors::Result;

#[derive(StructOpt)]
pub struct BlockCommand {
    #[structopt(name = "height", default_value = "latest", help = "height (optional)")]
    height: String,
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
