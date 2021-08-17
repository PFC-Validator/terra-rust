use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
//use crate::errors::Result;
//use crate::keys::get_private_key;

use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::messages::Message;

use crate::{NAME, VERSION};
use terra_rust_api::messages::slashing::MsgUnjail;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum SlashingCommand {
    #[structopt(name = "unjail", about = "unjail a validator")]
    UnJail {
        /// validator account (specify the key name in the wallet)
        validator: String,
    },
}

pub async fn slashing_cmd_parse<'a>(
    terra: &Terra<'a>,
    wallet: &Wallet<'a>,
    seed: Option<&str>,
    slashing_cmd: SlashingCommand,
) -> Result<()> {
    match slashing_cmd {
        SlashingCommand::UnJail { validator } => {
            let secp = Secp256k1::new();
            let from_key = wallet.get_private_key(&secp, &validator, seed)?;
            let from_public_key = from_key.public_key(&secp);

            let from_account = from_public_key.operator_address()?;
            let un_jail = MsgUnjail::create(from_account);

            let messages: Vec<Message> = vec![un_jail];
            let resp = terra
                .submit_transaction_sync(
                    &secp,
                    &from_key,
                    &messages,
                    Some(format!(
                        "PFC-{}/{}",
                        NAME.unwrap_or("TERRARUST"),
                        VERSION.unwrap_or("DEV")
                    )),
                )
                .await?;

            println!("{}", resp.txhash);
            log::info!("{}", resp.raw_log);
        }
    };
    Ok(())
}
