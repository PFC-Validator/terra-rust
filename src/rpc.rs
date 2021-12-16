use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;

#[derive(StructOpt)]
pub struct RPCCommand {
    #[structopt(
        name = "endpoint",
        long = "rpc-endpoint",
        about = "RPC endpoint",
        env = "TERRARUST_RPC_ENDPOINT",
        default_value = "http://127.0.0.1:26657"
    )]
    endpoint: String,
    #[structopt(subcommand)]
    cmd: RPCSubCommand,
}

#[derive(StructOpt)]
pub enum RPCSubCommand {
    #[structopt(name = "status", about = "status via RPC")]
    Status,
    #[structopt(name = "net-info", about = "network information from RPC endpoint")]
    NetInfo,
    #[structopt(
        name = "unconfirmed-txs",
        about = "Get the list of unconfirmed transactions"
    )]
    UnconfirmedTXS,
    #[structopt(name = "block", about = "Get the block at a given height")]
    Block { height: Option<u64> },
    #[structopt(name = "block-results", about = "Get the block at a given height")]
    BlockResults { height: Option<u64> },
}

pub async fn rpc_cmd_parse(terra: &Terra, cmd: RPCCommand) -> Result<()> {
    match cmd.cmd {
        RPCSubCommand::Status => {
            let resp = terra.rpc(&cmd.endpoint).status().await?;

            println!("{}", serde_json::to_string(&resp)?)
        }
        RPCSubCommand::NetInfo => {
            let resp = terra.rpc(&cmd.endpoint).net_info().await?;

            println!("{}", serde_json::to_string(&resp)?)
        }
        RPCSubCommand::UnconfirmedTXS => {
            let resp = terra.rpc(&cmd.endpoint).unconfirmed_txs().await?;

            println!("{}", serde_json::to_string(&resp)?)
        }
        RPCSubCommand::Block { height } => {
            let resp = match height {
                Some(h) => terra.rpc(&cmd.endpoint).block_at_height(h).await?,
                None => terra.rpc(&cmd.endpoint).block().await?,
            };

            println!("{}", serde_json::to_string(&resp)?)
        }
        RPCSubCommand::BlockResults { height } => {
            let resp = match height {
                Some(h) => terra.rpc(&cmd.endpoint).block_results_at_height(h).await?,
                None => terra.rpc(&cmd.endpoint).block_results().await?,
            };

            println!("{}", serde_json::to_string(&resp)?)
        }
    }
    Ok(())
}
