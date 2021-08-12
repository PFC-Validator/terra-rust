use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
//use crate::errors::Result;
//use crate::keys::get_private_key;

use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::messages::{Message, MsgSend};

use crate::{NAME, VERSION};
use rust_decimal::Decimal;
use terra_rust_api::core_types::Coin;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum BankCommand {
    #[structopt(name = "send", about = "send some coin[s] to an account")]
    Send {
        /// from account (specify the key name in the wallet
        from: String,
        /// the to account in 'terra' format
        to: String,
        /// the amount
        amount: Decimal,
        /// denom
        denom: String,
    },
}

pub async fn bank_cmd_parse<'a>(
    terra: &Terra<'a>,
    wallet: &Wallet<'a>,
    seed: Option<&str>,
    bank_cmd: BankCommand,
) -> Result<()> {
    match bank_cmd {
        BankCommand::Send {
            from,
            to,
            amount,
            denom,
        } => {
            let secp = Secp256k1::new();
            let from_key = wallet.get_private_key(&secp, &from, seed)?;
            let from_public_key = from_key.public_key(&secp);
            let coin: Coin = Coin::create(&denom, amount);
            let from_account = from_public_key.account()?;
            let send = MsgSend::create(from_account, to, vec![coin]);

            let messages: Vec<Message> = vec![send];
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
