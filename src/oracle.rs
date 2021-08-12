use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
//use crate::errors::Result;
//use crate::keys::get_private_key;
use crate::{NAME, VERSION};
use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::client::oracle::Voters;
use terra_rust_api::messages::oracle::MsgDelegateFeedConsent;
use terra_rust_api::messages::Message;
use terra_rust_wallet::Wallet;

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
    #[structopt(name = "voters", about = "commands related to exchange rate voting")]
    Voters {
        /// validator account (the terravaloper one)
        validator: String,
        #[structopt(subcommand)]
        cmd: VotersCommand,
    },
}
#[derive(StructOpt)]
pub enum VotersCommand {
    Votes,
    PreVotes,
    Feeder,
    Miss,
    AggregatePreVote,
    AggregateVote,
}
pub async fn oracle_cmd_parse<'a>(
    terra: &Terra<'a>,
    wallet: &Wallet<'a>,
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
            let from_key = wallet.get_private_key(&secp, &validator, seed)?;
            let from_public_key = from_key.public_key(&secp);
            let from_operator = from_public_key.operator_address()?;
            let delegate_msg = MsgDelegateFeedConsent::create(from_operator, delegate);

            let messages: Vec<Message> = vec![delegate_msg];
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
        OracleCommand::Voters { validator, cmd } => {
            let voter = terra.oracle().voters(&validator);
            voter_cmd_parse(&voter, wallet, seed, cmd).await?;
        }
    }
    Ok(())
}

pub async fn voter_cmd_parse<'a>(
    voters: &Voters<'a>,
    _wallet: &Wallet<'a>,
    _seed: Option<&str>,
    cmd: VotersCommand,
) -> Result<()> {
    match cmd {
        VotersCommand::Votes => {
            let x = voters.votes().await?;
            println!("{:#?}", x)
        }
        VotersCommand::PreVotes => {
            let x = voters.prevotes().await?;
            println!("{:#?}", x)
        }
        VotersCommand::Feeder => {
            let x = voters.feeder().await?;
            println!("{:#?}", x)
        }
        VotersCommand::Miss => {
            let x = voters.miss().await?;
            println!("{:#?}", x)
        }
        VotersCommand::AggregatePreVote => {
            todo!()
        }
        VotersCommand::AggregateVote => {
            todo!()
        }
    }
    Ok(())
}
