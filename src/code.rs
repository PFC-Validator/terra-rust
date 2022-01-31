use anyhow::Result;
use secp256k1::Secp256k1;
use std::path::Path;
use terra_rust_api::{Message, Terra};

use structopt::StructOpt;
//use terra_rust_api::client::tx_types::TXResultSync;
use crate::{NAME, VERSION};
use terra_rust_api::messages::wasm::MsgStoreCode;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum CodeCommand {
    Set {
        #[structopt(name = "sender", help = "the sender account")]
        sender: String,
        #[structopt(name = "contract", help = "WASM file to set")]
        wasm: String,
        #[structopt(name = "retries", help = "number of retries", default_value = "10")]
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
        CodeCommand::Set {
            sender,
            wasm,
            retries,
        } => {
            let from_key = wallet.get_private_key(&secp, &sender, seed)?;

            let from_public_key = from_key.public_key(&secp);

            let wasm_path = Path::new(&wasm);

            let store_message =
                MsgStoreCode::create_from_file(&from_public_key.account()?, wasm_path)?;
            let messages: Vec<Message> = vec![store_message];

            let resp = terra
                .submit_transaction_sync(
                    &secp,
                    &from_key,
                    messages,
                    Some(format!(
                        "PFC-{}/{}",
                        NAME.unwrap_or("TERRARUST"),
                        VERSION.unwrap_or("DEV")
                    )),
                )
                .await?;
            log::info!("{:?}", &resp);

            let hash = resp.txhash;
            let tx = terra
                .tx()
                .get_and_wait(&hash, retries, tokio::time::Duration::from_secs(3))
                .await?;
            let codes = tx.get_attribute_from_result_logs("store_code", "code_id");
            if let Some(code) = codes.first() {
                let code_id: u64 = code.1.parse()?;

                println!("Code has been stored as {}", code_id);
            }
        }
    }
    Ok(())
}
