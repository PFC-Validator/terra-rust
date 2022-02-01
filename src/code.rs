use anyhow::Result;
use secp256k1::{All, Secp256k1};
use std::path::Path;
use terra_rust_api::{Message, PrivateKey, Terra};

use structopt::StructOpt;
use terra_rust_api::core_types::Coin;
//use terra_rust_api::client::tx_types::TXResultSync;
use crate::{NAME, VERSION};
use terra_rust_api::messages::wasm::{MsgInstantiateContract, MsgMigrateContract, MsgStoreCode};
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum CodeCommand {
    Store {
        #[structopt(name = "sender", help = "the sender account")]
        sender: String,
        #[structopt(name = "wasm", help = "WASM file to set")]
        wasm: String,
        #[structopt(
            name = "retries",
            long = "retries",
            help = "number of retries",
            default_value = "10"
        )]
        retries: usize,
    },
    Instantiate {
        #[structopt(name = "sender", help = "the sender account")]
        sender: String,
        #[structopt(
            name = "wasm_file_or_code_id",
            help = "WASM file to set or code_id of existing one"
        )]
        wasm: String,
        #[structopt(name = "json_file", help = "JSON file to instantiate")]
        json_file: String,
        #[structopt(
            name = "coins",
            long = "coins",
            help = "initial coins",
            default_value = ""
        )]
        coins: String,
        #[structopt(
            name = "admin",
            help = "the admin account (defaults to same as sender. For no admin specify none)",
            long = "admin",
            default_value = "same"
        )]
        admin: String,
        #[structopt(
            name = "retries",
            long = "retries",
            help = "number of retries",
            default_value = "10"
        )]
        retries: usize,
    },
    Migrate {
        #[structopt(name = "sender", help = "the sender account")]
        sender: String,
        #[structopt(
            name = "contract",
            help = "the contract",
            long = "contract",
            env = "TERRARUST_CONTRACT"
        )]
        contract: String,
        #[structopt(
            name = "wasm_file_or_code_id",
            help = "WASM file to set or code_id of existing one"
        )]
        wasm: String,
        #[structopt(name = "json_file", long = "json", help = "JSON file to migrate")]
        json_file: Option<String>,

        #[structopt(
            name = "retries",
            long = "retries",
            help = "number of retries",
            default_value = "10"
        )]
        retries: usize,
    },
}

pub async fn code_cmd_parse(
    terra: &Terra,
    wallet: &Wallet<'_>,
    seed: Option<&str>,
    code_cmd: CodeCommand,
) -> Result<()> {
    let secp = Secp256k1::new();
    match code_cmd {
        CodeCommand::Store {
            sender,
            wasm,
            retries,
        } => {
            let from_key = wallet.get_private_key(&secp, &sender, seed)?;
            let hash = do_store(terra, &secp, &from_key, wasm).await?;
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
        CodeCommand::Instantiate {
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
                let hash = do_store(terra, &secp, &from_key, wasm).await?;
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
            let hash =
                do_instantiate(terra, &secp, &from_key, code_id, json, coin_vec, admin_key).await?;
            let contract = get_attribute_tx(
                terra,
                &hash,
                retries,
                tokio::time::Duration::from_secs(3),
                "wasm",
                "contract_address",
            )
            .await?;
            println!("Contract: {}", contract);
        }
        CodeCommand::Migrate {
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
                let hash = do_store(terra, &secp, &from_key, wasm).await?;
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
            let hash =
                do_migrate(terra, &secp, &from_key, &contract, new_code_id, json_file).await?;

            let tx = terra
                .tx()
                .get_and_wait(&hash, retries, tokio::time::Duration::from_secs(3))
                .await?;

            let codes = tx.get_attribute_from_result_logs("migrate_contract", "contract_address");
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
async fn do_store(
    terra: &Terra,
    secp: &Secp256k1<All>,
    from: &PrivateKey,
    wasm: String,
) -> Result<String> {
    let from_public_key = from.public_key(secp);

    let wasm_path = Path::new(&wasm);

    let store_message = MsgStoreCode::create_from_file(&from_public_key.account()?, wasm_path)?;
    let messages: Vec<Message> = vec![store_message];

    let resp = terra
        .submit_transaction_sync(
            secp,
            from,
            messages,
            Some(format!(
                "PFC-{}/{}",
                NAME.unwrap_or("TERRARUST"),
                VERSION.unwrap_or("DEV")
            )),
        )
        .await?;
    log::info!("{:?}", &resp);
    Ok(resp.txhash)
}

async fn do_migrate(
    terra: &Terra,
    secp: &Secp256k1<All>,
    from: &PrivateKey,
    contract: &str,
    new_code_id: u64,
    migrate_file: Option<String>,
) -> Result<String> {
    let from_public_key = from.public_key(secp);

    let migrate_message = if let Some(migrate_path) = migrate_file {
        MsgMigrateContract::create_from_file(
            &from_public_key.account()?,
            contract,
            new_code_id,
            Path::new(&migrate_path),
        )?
    } else {
        MsgMigrateContract::create_from_json(
            &from_public_key.account()?,
            contract,
            new_code_id,
            "{}",
        )?
    };

    let messages: Vec<Message> = vec![migrate_message];

    let resp = terra
        .submit_transaction_sync(
            secp,
            from,
            messages,
            Some(format!(
                "PFC-{}/{}",
                NAME.unwrap_or("TERRARUST"),
                VERSION.unwrap_or("DEV")
            )),
        )
        .await?;
    log::info!("{:?}", &resp);
    Ok(resp.txhash)
}

async fn do_instantiate(
    terra: &Terra,
    secp: &Secp256k1<All>,
    from: &PrivateKey,
    code_id: u64,
    json: &Path,
    coins: Vec<Coin>,
    admin: Option<String>,
) -> Result<String> {
    let from_public_key = from.public_key(secp);
    let init_message = MsgInstantiateContract::create_from_file(
        &from_public_key.account()?,
        admin,
        code_id,
        json,
        coins,
    )?;
    let messages: Vec<Message> = vec![init_message];

    let resp = terra
        .submit_transaction_sync(
            secp,
            from,
            messages,
            Some(format!(
                "PFC-{}/{}",
                NAME.unwrap_or("TERRARUST"),
                VERSION.unwrap_or("DEV")
            )),
        )
        .await?;
    log::info!("{:?}", &resp);

    Ok(resp.txhash)
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
