use anyhow::Result;
use clap::{Parser, Subcommand};
use secp256k1::Secp256k1;
use terra_rust_api::messages::{Message, MsgSend};
use terra_rust_api::Terra;

use crate::{NAME, VERSION};
use rust_decimal::Decimal;
use terra_rust_api::core_types::Coin;
use terra_rust_wallet::Wallet;
/// Bank Transactions
#[derive(Parser)]
pub struct BankCommand {
    #[clap(subcommand)]
    command: BankEnum,
}
#[derive(Subcommand)]
pub enum BankEnum {
    #[clap(name = "send", about = "send some coin[s] to an account")]
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
    Balance {
        account: String,
    },
}
impl BankCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        match self.command {
            BankEnum::Send {
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
                let send = MsgSend::create(from_account, to.clone(), vec![coin])?;

                let messages: Vec<Message> = vec![send];
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
            BankEnum::Balance { account } => {
                let account_id = if !account.starts_with("terra1") {
                    let secp = Secp256k1::new();
                    wallet.get_account(&secp, &account, seed)?
                } else {
                    account.clone()
                };
                let sw = terra.bank().balances(&account_id).await?;
                println!("{}", serde_json::to_string_pretty(&sw)?);
            }
        };
        Ok(())
    }
}
