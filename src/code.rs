use anyhow::Result;
use secp256k1::Secp256k1;
use std::path::Path;
use terra_rust_api::Terra;

use crate::{NAME, VERSION};
use clap::{Parser, Subcommand};
use terra_rust_api::core_types::Coin;
use terra_rust_api::messages::wasm::{MsgInstantiateContract, MsgMigrateContract};
use terra_rust_wallet::Wallet;
#[derive(Parser)]
/// set code
pub struct CodeCommand {
    #[clap(subcommand)]
    command: CodeEnum,
}
#[derive(Subcommand)]
pub enum CodeEnum {
    Store {
        #[clap(name = "sender", help = "the sender account")]
        sender: String,
        #[clap(name = "wasm", help = "WASM file to set")]
        wasm: String,
        #[clap(
            name = "retries",
            long = "retries",
            help = "number of retries",
            default_value = "10"
        )]
        retries: usize,
    },
    Instantiate {
        #[clap(name = "sender", help = "the sender account")]
        sender: String,
        #[clap(
            name = "wasm_file_or_code_id",
            help = "WASM file to set or code_id of existing one"
        )]
        wasm: String,
        #[clap(name = "json_file", help = "JSON file to instantiate")]
        json_file: String,
        #[clap(
            name = "coins",
            long = "coins",
            help = "initial coins",
            default_value = ""
        )]
        coins: String,
        #[clap(
            name = "admin",
            help = "the admin account (defaults to same as sender. For no admin specify none)",
            long = "admin",
            default_value = "same"
        )]
        admin: String,
        #[clap(
            name = "retries",
            long = "retries",
            help = "number of retries",
            default_value = "10"
        )]
        retries: usize,
    },
    Migrate {
        #[clap(name = "sender", help = "the sender account")]
        sender: String,
        #[clap(
            name = "contract",
            help = "the contract",
            long = "contract",
            env = "TERRARUST_CONTRACT"
        )]
        contract: String,
        #[clap(
            name = "wasm_file_or_code_id",
            help = "WASM file to set or code_id of existing one"
        )]
        wasm: String,
        #[clap(name = "json_file", long = "json", help = "JSON file to migrate")]
        json_file: Option<String>,

        #[clap(
            name = "retries",
            long = "retries",
            help = "number of retries",
            default_value = "10"
        )]
        retries: usize,
    },
}
impl CodeCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        let secp = Secp256k1::new();
        let memo = Some(format!(
            "PFC-{}/{}",
            NAME.unwrap_or("TERRARUST"),
            VERSION.unwrap_or("DEV")
        ));
        match self.command {
            CodeEnum::Store {
                sender,
                wasm,
                retries,
            } => {
                let from_key = wallet.get_private_key(&secp, &sender, seed)?;
                let hash = terra
                    .wasm()
                    .store(&secp, &from_key, &wasm, memo)
                    .await?
                    .txhash;
                let code_id = get_attribute_tx(
                    terra,
                    &hash,
                    retries,
                    tokio::time::Duration::from_secs(3),
                    "store_code",
                    "code_id",
                )
                .await?;

                println!("Code Id: {}", code_id);
            }
            CodeEnum::Instantiate {
                sender,
                wasm,
                json_file,
                coins,
                admin,
                retries,
            } => {
                let from_key = wallet.get_private_key(&secp, &sender, seed)?;
                let json = Path::new(&json_file);

                let admin_key = if admin.starts_with("terra1") {
                    Some(admin)
                } else if admin == "same" {
                    Some(from_key.public_key(&secp).account()?)
                } else if admin == "none" {
                    todo!("Admin of none is currently not supported")
                } else {
                    let admin_key = wallet.get_public_key(&secp, &admin, seed)?;
                    let admin_account = admin_key.account()?;
                    Some(admin_account)
                };
                let coin_vec: Vec<Coin> = if coins.is_empty() {
                    vec![]
                } else {
                    Coin::parse_coins(&coins)?
                };
                let code_id = if let Ok(code_id) = wasm.parse::<u64>() {
                    code_id
                } else {
                    let hash = terra
                        .wasm()
                        .store(&secp, &from_key, &wasm, memo.clone())
                        .await?
                        .txhash;
                    let code_id = get_attribute_tx(
                        terra,
                        &hash,
                        retries,
                        tokio::time::Duration::from_secs(3),
                        "store_code",
                        "code_id",
                    )
                    .await?;

                    code_id.parse::<u64>()?
                };
                let contents = MsgInstantiateContract::replace_parameters(
                    &from_key.public_key(&secp).account()?,
                    admin_key.clone(),
                    code_id,
                    &std::fs::read_to_string(json)?,
                );

                let hash = terra
                    .wasm()
                    .instantiate(
                        &secp, &from_key, code_id, contents, coin_vec, admin_key, memo,
                    )
                    .await?
                    .txhash;
                let contract = get_attribute_tx(
                    terra,
                    &hash,
                    retries,
                    tokio::time::Duration::from_secs(3),
                    "instantiate_contract",
                    "contract_address",
                )
                .await?;
                println!("Contract: {}", contract);
            }
            CodeEnum::Migrate {
                sender,
                contract,
                wasm,
                json_file,
                retries,
            } => {
                let from_key = wallet.get_private_key(&secp, &sender, seed)?;

                let new_code_id = if let Ok(code_id) = wasm.parse::<u64>() {
                    code_id
                } else {
                    let hash = terra
                        .wasm()
                        .store(&secp, &from_key, &wasm, memo.clone())
                        .await?
                        .txhash;
                    let code_id = get_attribute_tx(
                        terra,
                        &hash,
                        retries,
                        tokio::time::Duration::from_secs(3),
                        "store_code",
                        "code_id",
                    )
                    .await?;
                    code_id.parse::<u64>()?
                };
                let contents = if let Some(json_filename) = json_file {
                    let json = Path::new(&json_filename);

                    Some(MsgMigrateContract::replace_parameters(
                        &from_key.public_key(&secp).account()?,
                        &contract,
                        new_code_id,
                        &std::fs::read_to_string(json)?,
                    ))
                } else {
                    None
                };

                let hash = terra
                    .wasm()
                    .migrate(&secp, &from_key, &contract, new_code_id, contents, memo)
                    .await?
                    .txhash;

                let tx = terra
                    .tx()
                    .get_and_wait(&hash, retries, tokio::time::Duration::from_secs(3))
                    .await?;

                let codes =
                    tx.get_attribute_from_result_logs("migrate_contract", "contract_address");
                let contract = if let Some(code) = codes.first() {
                    code.1.clone()
                } else {
                    panic!(
                        "{}/{} not present in TX log",
                        "migrate_contract", "contract_address"
                    );
                };

                let codes = tx.get_attribute_from_result_logs("migrate_contract", "code_id");
                let code_id = if let Some(code) = codes.first() {
                    code.1.clone()
                } else {
                    panic!("{}/{} not present in TX log", "migrate_contract", "code_id");
                };

                println!("Contract: {} Migrated to {}", contract, code_id);
            }
        }
        Ok(())
    }
}

async fn get_attribute_tx(
    terra: &Terra,
    hash: &str,
    retries: usize,
    sleep: tokio::time::Duration,
    event_type: &str,
    attribute_key: &str,
) -> Result<String> {
    let tx = terra.tx().get_and_wait(hash, retries, sleep).await?;
    let codes = tx.get_attribute_from_result_logs(event_type, attribute_key);
    if let Some(code) = codes.first() {
        Ok(code.1.clone())
    } else {
        panic!("{}/{} not present in TX log", event_type, attribute_key)
    }
}
