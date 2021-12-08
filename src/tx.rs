use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
//use crate::errors::Result;

/// Input to the /txs/XXXX query
#[derive(StructOpt)]
pub enum TxCommand {
    #[structopt(name = "hash", about = "look up TX by hash")]
    Hash {
        #[structopt(name = "hash", help = "hash to inquire about")]
        /// The hash to inquire about
        hash: String,
    },
    #[structopt(name = "block", about = "look up TXs in a block")]
    Block {
        #[structopt(name = "height", help = "block height to inquire about")]
        height: u64,
        offset: Option<u64>,
        limit: Option<u64>,
    },
}

pub async fn tx_cmd_parse(terra: &Terra, cmd: TxCommand) -> Result<()> {
    match cmd {
        TxCommand::Hash { hash } => {
            let tx = terra.tx().get(&hash).await?;
            println!("{}", serde_json::to_string_pretty(&tx)?);
        }
        TxCommand::Block {
            height,
            offset,
            limit,
        } => {
            let txs = terra.tx().get_txs_in_block(height, offset, limit).await?;
            println!("{}", serde_json::to_string_pretty(&txs)?);
        }
    }
    Ok(())
}
