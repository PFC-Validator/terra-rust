use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;

use crate::{NAME, VERSION};
use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::messages::distribution::{
    MsgWithdrawDelegationReward, MsgWithdrawValidatorCommission,
};

use terra_rust_api::messages::Message;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum DistributionCommand {
    #[structopt(name = "withdraw-reward")]
    /// Withdraw delegation reward
    WithdrawReward {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,
        #[structopt(
            name = "validator",
            help = "the validator's terravaloper address. If blank withdrawls from ALL"
        )]
        /// the validator's oper terravaloper1XXXXXXXXX. if blank, withdraws from ALL validators (todo)
        validator: Option<String>,
    },
    #[structopt(name = "withdraw-commission")]
    /// Withdraw commission
    WithdrawCommission {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,
        #[structopt(name = "validator", help = "the validator's terravaloper address.")]
        ///  the validator's oper terravaloper1XXXXXXXXX.
        validator: String,
    },
    #[structopt(name = "withdraw")]
    /// Withdraw commission & delegation reward
    Withdraw {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        ///  the validator's oper terravaloper1XXXXXXXXX.
        validator: String,
    },
}

pub async fn distribution_cmd_parse<'a>(
    terra: &Terra<'a>,
    wallet: &Wallet<'a>,
    seed: Option<&str>,
    cmd: DistributionCommand,
) -> Result<()> {
    let secp = Secp256k1::new();
    match cmd {
        DistributionCommand::WithdrawReward {
            delegator,
            validator,
        } => {
            log::info!("Delegator {}", &delegator);
            let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
            let delegator_account = delegator_key.public_key(&secp).account()?;

            match validator {
                Some(v) => {
                    log::info!("Validator {}", &v);
                    let msg = MsgWithdrawDelegationReward::create(delegator_account, v);
                    let messages: Vec<Message> = vec![msg];
                    let resp = terra
                        .submit_transaction_sync(
                            &secp,
                            &delegator_key,
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
                None => todo!("withdrawing from ALL validators not implemented yet"),
            }
            Ok(())
        }
        DistributionCommand::WithdrawCommission {
            delegator,
            validator,
        } => {
            log::info!("Delegator {}", &delegator);
            let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;

            log::info!("Validator {}", &validator);
            let msg = MsgWithdrawValidatorCommission::create(validator);
            let messages: Vec<Message> = vec![msg];
            let resp = terra
                .submit_transaction_sync(
                    &secp,
                    &delegator_key,
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
            Ok(())
        }
        DistributionCommand::Withdraw {
            delegator,
            validator,
        } => {
            log::info!("Delegator {}", &delegator);
            let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
            let delegator_account = delegator_key.public_key(&secp).account()?;

            log::info!("Validator {}", &validator);
            let msg_commission = MsgWithdrawValidatorCommission::create(validator.clone());
            let msg_rewards = MsgWithdrawDelegationReward::create(delegator_account, validator);
            let messages: Vec<Message> = vec![msg_commission, msg_rewards];
            let resp = terra
                .submit_transaction_sync(
                    &secp,
                    &delegator_key,
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
            Ok(())
        }
    }
}
