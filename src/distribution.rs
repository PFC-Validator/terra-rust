use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;

use crate::{NAME, VERSION};
use secp256k1::Secp256k1;
use terra_rust_api::messages::distribution::{
    MsgWithdrawDelegationReward, MsgWithdrawValidatorCommission,
};

use terra_rust_api::messages::Message;
use terra_rust_wallet::Wallet;

#[derive(Subcommand)]
enum DistributionEnum {
    #[clap(name = "withdraw-reward")]
    /// Withdraw delegation reward
    Reward {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,
        #[clap(
            name = "validator",
            help = "the validator's terravaloper address. If blank withdraws from ALL"
        )]
        /// the validator's oper terravaloper1XXXXXXXXX. if blank, withdraws from ALL validators (todo)
        validator: Option<String>,
    },
    #[clap(name = "withdraw-commission")]
    /// Withdraw commission
    Commission {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,
        #[clap(name = "validator", help = "the validator's terravaloper address.")]
        ///  the validator's oper terravaloper1XXXXXXXXX.
        validator: String,
    },
    #[clap(name = "withdraw")]
    /// Withdraw commission & delegation reward
    Withdraw {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,
    },
}
#[derive(Parser)]
/// Validator Distribution (Reward) Commands
pub struct DistributionCommand {
    #[clap(subcommand)]
    command: DistributionEnum,
}
impl DistributionCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        let secp = Secp256k1::new();
        match self.command {
            DistributionEnum::Reward {
                delegator,
                validator,
            } => {
                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
                let delegator_account = delegator_key.public_key(&secp).account()?;

                match validator {
                    Some(v) => {
                        log::info!("Validator {}", &v);
                        let msg = MsgWithdrawDelegationReward::create(delegator_account, v)?;
                        let messages: Vec<Message> = vec![msg];
                        let resp = terra
                            .submit_transaction_sync(
                                &secp,
                                &delegator_key,
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
                    None => todo!("withdrawing from ALL validators not implemented yet"),
                }
                Ok(())
            }
            DistributionEnum::Commission {
                delegator,
                validator,
            } => {
                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;

                log::info!("Validator {}", &validator);
                let msg = MsgWithdrawValidatorCommission::create(validator)?;
                let messages: Vec<Message> = vec![msg];
                let resp = terra
                    .submit_transaction_sync(
                        &secp,
                        &delegator_key,
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
                Ok(())
            }
            DistributionEnum::Withdraw {
                delegator,
                //validator,
            } => {
                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
                let delegator_account = delegator_key.public_key(&secp).account()?;
                let validator = delegator_key.public_key(&secp).operator_address()?;

                log::info!("Validator {}", &validator);
                let msg_commission = MsgWithdrawValidatorCommission::create(validator.clone())?;
                let msg_rewards =
                    MsgWithdrawDelegationReward::create(delegator_account, validator)?;
                let messages: Vec<Message> = vec![msg_commission, msg_rewards];
                let resp = terra
                    .submit_transaction_sync(
                        &secp,
                        &delegator_key,
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
                Ok(())
            }
        }
    }
}
