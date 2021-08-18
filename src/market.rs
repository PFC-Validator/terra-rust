use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::{PublicKey, Terra};
//use crate::errors::Result;
//use crate::keys::get_private_key;

use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::messages::Message;

use crate::{NAME, VERSION};
use rust_decimal::Decimal;
use terra_rust_api::core_types::Coin;
use terra_rust_api::messages::market::MsgSwap;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum MarketCommand {
    #[structopt(name = "swap-rate", about = "see the exchange rate")]
    SwapRate {
        #[structopt(name = "denom", help = "token symbol. remember we are uXXX not XXX")]
        denom: String,
        #[structopt(name = "amount", help = "the amount. remember we are uXXX not XXX")]
        amount: Decimal,
        #[structopt(name = "ask", help = "what to swap the amount into")]
        ask: String,
    },
    #[structopt(name = "swap", about = "swap/exchange token")]
    Swap {
        /// from. The nickname in the wallet used to sign the transaction, do the token exchange
        from: String,
        #[structopt(
            name = "offer",
            help = "amount & type of token to switch uXXX not XXX. eg 1000ukrw"
        )]
        offer_coin: String,
        #[structopt(
            name = "ask",
            help = "what to exchange it into. remember we are uXXX not XXX. eg uluna"
        )]
        ask_denom: String,
        #[structopt(name = "to", help = "who to send it too. defaults to yourself")]
        to: Option<String>,
    },
    #[structopt(
        name = "sweep",
        about = "swap all tokens above a certain threshold in your wallet into a single token"
    )]
    Sweep {
        /// from. The nickname in the wallet used to sign the transaction, do the token exchange
        from: String,
        #[structopt(name = "to-coin", help = "the token to exchange into")]
        to_coin: String,
        #[structopt(
            name = "threshold",
            help = "the minimum amount to consider in 'to-coin' denomination. (ie. don't swap if there is only 10c worth)"
        )]
        threshold: Decimal,
    },
}

pub async fn market_cmd_parse<'a>(
    terra: &Terra<'a>,
    wallet: &Wallet<'a>,
    seed: Option<&str>,
    market_cmd: MarketCommand,
) -> Result<()> {
    match market_cmd {
        MarketCommand::SwapRate { denom, ask, amount } => {
            let coin = Coin::create(&denom, amount);
            let sw = terra.market().swap(&coin, &ask).await?;

            println!("{}", serde_json::to_string_pretty(&sw)?);
        }

        MarketCommand::Swap {
            from,
            offer_coin,
            to,
            ask_denom,
        } => {
            let secp = Secp256k1::new();
            let from_key = wallet.get_private_key(&secp, &from, seed)?;
            let from_public_key = from_key.public_key(&secp);
            let coin: Coin = Coin::parse(&offer_coin)
                .expect("invalid offer coin. hint: 1000ukrw")
                .unwrap();
            let from_account = from_public_key.account()?;
            let to_account = match to {
                Some(to_k) => {
                    let valid_tok = PublicKey::from_account(&to_k)?;
                    valid_tok.account()?
                }
                None => from_account.clone(),
            };
            let swap = MsgSwap::create(coin, ask_denom, to_account);

            let messages: Vec<Message> = vec![swap];
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
        MarketCommand::Sweep {
            from,
            to_coin,
            threshold,
        } => {
            let secp = Secp256k1::new();
            let from_key = wallet.get_private_key(&secp, &from, seed)?;
            let from_public_key = from_key.public_key(&secp);
            let from_account = from_public_key.account()?;
            let messages = terra
                .market()
                .generate_sweep_messages(from_account, to_coin, threshold)
                .await?;

            if messages.is_empty() {
                println!("No coins match your threshold")
            } else {
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
        }
    };
    Ok(())
}
