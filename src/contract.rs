use crate::errors::Result;
use serde_json::Value;
use structopt::StructOpt;
use terra_rust_api::Terra;
use terra_rust_wallet::Wallet;
#[derive(StructOpt)]
pub enum ContractCommand {
    #[structopt(name = "codes", about = "Get code info based on the code ID")]
    Codes {
        #[structopt(name = "codeId", help = "code id you want to obtain info about")]
        code_id: u64,
    },
    #[structopt(name = "info", about = "Get code info based on the contract address")]
    Info {
        #[structopt(
            name = "contract_address",
            help = "code address you want to obtain info about"
        )]
        contract_address: String,
    },

    #[structopt(name = "query", about = "query contract with json")]
    Query {
        #[structopt(name = "contract_address", help = "code address you want to query")]
        contract_address: String,
        #[structopt(name = "query", help = "json formatted query message")]
        query: String,
    },
    #[structopt(name = "raw", about = "query via key/subkey")]
    Raw {
        #[structopt(name = "contract_address", help = "code address you want to query")]
        contract_address: String,
        #[structopt(name = "key", help = "the key you want to query")]
        key: String,
        #[structopt(name = "subkey", help = "the sub key you want to query")]
        subkey: Option<String>,
    },
    #[structopt(name = "parameters", about = "Get parameters for contracts")]
    Parameters,
}

pub async fn contract_cmd_parse<'a>(
    terra: &Terra<'a>,
    _wallet: &Wallet<'a>,
    _seed: Option<&str>,
    cmd: ContractCommand,
) -> Result<()> {
    match cmd {
        ContractCommand::Codes { code_id } => {
            let code_result = terra.wasm().codes(code_id).await?;
            println!("{}", serde_json::to_string_pretty(&code_result)?);
            Ok(())
        }
        ContractCommand::Info { contract_address } => {
            let code_result = terra.wasm().info(&contract_address).await?;
            println!("{}", serde_json::to_string_pretty(&code_result)?);
            Ok(())
        }
        ContractCommand::Parameters => {
            let code_result = terra.wasm().parameters().await?;
            println!("{}", serde_json::to_string_pretty(&code_result)?);
            Ok(())
        }
        ContractCommand::Query {
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
        ContractCommand::Raw {
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
