use structopt::StructOpt;
use terra_rust_api::Terra;

use crate::errors::Result;
use crate::keys::get_private_key;
use crate::{NAME, VERSION};
use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::core_types::Msg;
use terra_rust_api::messages::oracle::MsgDelegateFeedConsent;

#[derive(StructOpt)]
pub enum OracleCommand {
    #[structopt(name = "parameters", about = "Get Oracle Parameters")]
    Parameters,
    #[structopt(
        name = "set-feeder",
        about = "set account that can submit exchange rate updates on behalf of your validator"
    )]
    SetFeeder {
        /// validator account (specify the key name in the wallet)
        validator: String,
        /// the delegate account
        #[structopt(name = "delegate", help = "The account name of the feeder account")]
        delegate: String,
    },
}

pub async fn oracle_cmd_parse(
    terra: &Terra<'_>,
    wallet: &str,
    seed: Option<&str>,
    oracle_cmd: OracleCommand,
) -> Result<()> {
    match oracle_cmd {
        OracleCommand::Parameters => {
            let resp = terra.oracle().parameters().await?;

            println!("{}", serde_json::to_string(&resp)?)
        }
        OracleCommand::SetFeeder {
            validator,
            delegate,
        } => {
            println!("Set Feeder {}", delegate);
            let secp = Secp256k1::new();
            let from_key = get_private_key(&secp, wallet, &validator, seed)?;
            let from_public_key = from_key.public_key(&secp);
            let from_operator = from_public_key.operator_address()?;
            let delegate_msg: MsgDelegateFeedConsent =
                MsgDelegateFeedConsent::create(from_operator, delegate);

            let messages: Vec<Box<dyn Msg>> = vec![Box::new(delegate_msg)];
            let (std_sign_msg, sigs) = terra
                .generate_transaction_to_broadcast(
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

            let resp = terra.tx().broadcast_sync(&std_sign_msg, &sigs).await?;
            match resp.code {
                Some(code) => {
                    log::error!("{}", serde_json::to_string(&resp)?);
                    eprintln!("Transaction returned a {} {}", code, resp.txhash)
                }
                None => {
                    println!("{}", resp.txhash)
                }
            }
        }
    }
    Ok(())
}
