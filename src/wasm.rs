use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;

use serde_json::Value;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum WasmCommand {
    #[structopt(name = "query", about = "exec a query against a smart contract")]
    Query {
        /// the contract address
        contract: String,
        /// the json to send
        json: String,
    },
}

pub async fn wasm_cmd_parse<'a>(
    terra: &Terra<'a>,
    _wallet: &Wallet<'a>,
    _seed: Option<&str>,
    wasm_cmd: WasmCommand,
) -> Result<()> {
    match wasm_cmd {
        WasmCommand::Query { contract, json } => {
            let resp = terra.wasm().query::<Value>(&contract, &json).await?;
            println!("{}", serde_json::to_string_pretty(&resp)?);
        }
    };
    Ok(())
}
