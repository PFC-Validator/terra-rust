use anyhow::Result;
use clap::{Parser, Subcommand};
use serde_json::Value;
use terra_rust_api::Terra;

#[derive(Subcommand)]
enum ContractEnum {
    #[clap(name = "codes", about = "Get code info based on the code ID")]
    Codes {
        #[clap(name = "codeId", help = "code id you want to obtain info about")]
        code_id: u64,
    },
    #[clap(name = "info", about = "Get code info based on the contract address")]
    Info {
        #[clap(
            name = "contract_address",
            help = "code address you want to obtain info about"
        )]
        contract_address: String,
    },

    #[clap(name = "query", about = "query contract with json")]
    Query {
        #[clap(name = "contract_address", help = "code address you want to query")]
        contract_address: String,
        #[clap(name = "query", help = "json formatted query message")]
        query: String,
    },
    #[clap(name = "raw", about = "query via key/subkey")]
    Raw {
        #[clap(name = "contract_address", help = "code address you want to query")]
        contract_address: String,
        #[clap(name = "key", help = "the key you want to query")]
        key: String,
        #[clap(name = "subkey", help = "the sub key you want to query")]
        subkey: Option<String>,
    },
    #[clap(name = "parameters", about = "Get parameters for contracts")]
    Parameters,
}
#[derive(Parser)]
/// WASM Module / Smart Contract commands
pub struct ContractCommand {
    #[clap(subcommand)]
    command: ContractEnum,
}
impl ContractCommand {
    pub async fn parse(self, terra: &Terra) -> Result<()> {
        match self.command {
            ContractEnum::Codes { code_id } => {
                let code_result = terra.wasm().codes(code_id).await?;
                println!("{}", serde_json::to_string_pretty(&code_result)?);
                Ok(())
            }
            ContractEnum::Info { contract_address } => {
                let code_result = terra.wasm().info(&contract_address).await?;
                println!("{}", serde_json::to_string_pretty(&code_result)?);
                Ok(())
            }
            ContractEnum::Parameters => {
                let code_result = terra.wasm().parameters().await?;
                println!("{}", serde_json::to_string_pretty(&code_result)?);
                Ok(())
            }
            ContractEnum::Query {
                contract_address,
                query,
            } => {
                let code_result = terra
                    .wasm()
                    .query::<Value>(&contract_address, &query)
                    .await?;
                println!("{}", serde_json::to_string_pretty(&code_result)?);
                Ok(())
            }
            ContractEnum::Raw {
                contract_address,
                key,
                subkey,
            } => {
                let (key, value) = terra
                    .wasm()
                    .query_raw(&contract_address, &key, &subkey)
                    .await?;
                println!("{} {}", key, value);
                Ok(())
            }
        }
    }
}
