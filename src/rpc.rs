use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;

#[derive(Parser)]
/// Tendermint ValidatorSets commands/RPC endpoint
pub struct RPCCommand {
    #[clap(
        name = "endpoint",
        long = "rpc-endpoint",
        env = "TERRARUST_RPC_ENDPOINT",
        default_value = "http://127.0.0.1:26657"
    )]
    endpoint: String,
    #[clap(subcommand)]
    cmd: RPCSubCommand,
}

#[derive(Subcommand)]
pub enum RPCSubCommand {
    #[clap(name = "status", about = "status via RPC")]
    Status,
    #[clap(name = "net-info", about = "network information from RPC endpoint")]
    NetInfo,
    #[clap(
        name = "unconfirmed-txs",
        about = "Get the list of unconfirmed transactions"
    )]
    UnconfirmedTXS,
    #[clap(name = "block", about = "Get the block at a given height")]
    Block { height: Option<u64> },
    #[clap(name = "block-results", about = "Get the block at a given height")]
    BlockResults { height: Option<u64> },
}
impl RPCCommand {
    pub async fn parse(self, terra: &Terra) -> Result<()> {
        let rpc_endpoint = terra.rpc(&self.endpoint);
        match self.cmd {
            RPCSubCommand::Status => {
                let resp = rpc_endpoint.status().await?;

                println!("{}", serde_json::to_string(&resp)?)
            }
            RPCSubCommand::NetInfo => {
                let resp = rpc_endpoint.net_info().await?;

                println!("{}", serde_json::to_string(&resp)?)
            }
            RPCSubCommand::UnconfirmedTXS => {
                let resp = rpc_endpoint.unconfirmed_txs().await?;

                println!("{}", serde_json::to_string(&resp)?)
            }
            RPCSubCommand::Block { height } => {
                let resp = match height {
                    Some(h) => rpc_endpoint.block_at_height(h).await?,
                    None => rpc_endpoint.block().await?,
                };

                println!("{}", serde_json::to_string(&resp)?)
            }
            RPCSubCommand::BlockResults { height } => {
                let resp = match height {
                    Some(h) => rpc_endpoint.block_results_at_height(h).await?,
                    None => rpc_endpoint.block_results().await?,
                };

                println!("{}", serde_json::to_string(&resp)?)
            }
        }
        Ok(())
    }
}
