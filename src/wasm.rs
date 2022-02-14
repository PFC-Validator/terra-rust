use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;

use serde_json::Value;

#[derive(Subcommand)]
enum WasmEnum {
    #[clap(name = "query", about = "exec a query against a smart contract")]
    Query {
        /// the contract address
        contract: String,
        /// the json to send
        json: String,
    },
}
/// WASM commands
#[derive(Parser)]
pub struct WasmCommand {
    #[clap(subcommand)]
    command: WasmEnum,
}
impl WasmCommand {
    pub async fn parse(self, terra: &Terra) -> Result<()> {
        match self.command {
            WasmEnum::Query { contract, json } => {
                let resp = terra.wasm().query::<Value>(&contract, &json).await?;
                println!("{}", serde_json::to_string_pretty(&resp)?);
            }
        };
        Ok(())
    }
}
