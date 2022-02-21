use clap::{Parser, Subcommand};
use terra_rust_api::Terra;

/// Input to the /txs/XXXX query
#[derive(Subcommand)]
enum TxEnum {
    #[clap(name = "hash", about = "look up TX by hash")]
    Hash {
        #[clap(name = "hash", help = "hash to inquire about")]
        /// The hash to inquire about
        hash: String,
    },
    #[clap(name = "block", about = "look up TXs in a block")]
    Block {
        #[clap(name = "height", help = "block height to inquire about")]
        height: u64,
        offset: Option<u64>,
        limit: Option<u64>,
    },
}
#[derive(Parser)]
/// Transaction Commands
pub struct TxCommand {
    #[clap(subcommand)]
    command: TxEnum,
}
impl TxCommand {
    pub async fn parse(self, terra: &Terra) -> anyhow::Result<()> {
        match self.command {
            TxEnum::Hash { hash } => {
                let tx = terra.tx().get_v1(&hash).await?;
                println!("{}", serde_json::to_string_pretty(&tx)?);
            }
            TxEnum::Block {
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
}
