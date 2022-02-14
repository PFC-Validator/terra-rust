use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;

use secp256k1::Secp256k1;
use terra_rust_api::messages::Message;

use crate::{NAME, VERSION};
use terra_rust_api::messages::slashing::MsgUnjail;
use terra_rust_wallet::Wallet;

#[derive(Subcommand)]
enum SlashingEnum {
    #[clap(name = "unjail", about = "unjail a validator")]
    UnJail {
        /// validator account (specify the key name in the wallet)
        validator: String,
    },
}

/// Slashing Commands
#[derive(Parser)]
pub struct SlashingCommand {
    #[clap(subcommand)]
    command: SlashingEnum,
}
impl SlashingCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        match self.command {
            SlashingEnum::UnJail { validator } => {
                let secp = Secp256k1::new();
                let from_key = wallet.get_private_key(&secp, &validator, seed)?;
                let from_public_key = from_key.public_key(&secp);

                let from_account = from_public_key.operator_address()?;
                let un_jail = MsgUnjail::create(from_account)?;

                let messages: Vec<Message> = vec![un_jail];
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

                println!("{}", resp.txhash);
                log::info!("{}", resp.raw_log);
            }
        };
        Ok(())
    }
}
