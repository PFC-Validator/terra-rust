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
}

pub async fn rpc_cmd_parse(terra: &Terra<'_>, cmd: RPCCommand) -> Result<()> {
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
    }
    Ok(())
}
