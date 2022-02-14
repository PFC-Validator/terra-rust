use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;

use crate::{NAME, VERSION};
use rust_decimal::Decimal;
use secp256k1::Secp256k1;
use terra_rust_api::core_types::Coin;
use terra_rust_api::messages::staking::{
    MsgBeginRedelegate, MsgCreateValidator, MsgDelegate, MsgEditValidator, MsgUndelegate,
    ValidatorCommission, ValidatorDescription,
};
use terra_rust_api::messages::Message;
use terra_rust_wallet::Wallet;

#[derive(Subcommand)]
enum StakingEnum {
    #[clap(name = "create-validator")]
    /// Create a validator
    CreateValidator {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,

        #[clap(name = "validator", help = "the validator's terravaloper address")]
        /// the validator's publickey  terravalconspub1XXXXXXXXX. try  terrad tendermint show-validator
        validator: String,
        #[clap(
            name = "pubkey",
            help = "the validator's tendermint consensus public key"
        )]
        /// the validator's tendermint consensus public key
        pubkey: String,
        #[clap(name = "moniker", help = "the validator's moniker")]
        moniker: String,
        /// amount of coins to delegate
        amount: String,
        /// commission rate. 1.00 = 100%
        rate: Decimal,
        /// maximum commission rate. 1.00 = 100%
        max_rate: Decimal,
        /// maximum amount that commission can change in a 24hour period. 1.00 = 100%
        max_change_rate: Decimal,
        /// the minimum amount required for the validator to be active. going under this amount will force the validator to be jailed
        min_self_delegation: Decimal,

        /// \[Optional\] the keybase.io PGP identity string.
        identity: Option<String>,
        /// \[Optional\] public URL
        website: Option<String>,
        /// \[Optional\] public contact point. (usually email)
        security_contact: Option<String>,
        /// \[Optional\] general text describing the validator
        details: Option<String>,
    },
    #[clap(name = "edit-validator")]
    /// edit a validator's details
    EditValidator {
        #[clap(name = "validator", help = "the validator key in the wallet")]
        validator: String,
        #[clap(long = "moniker", help = "the validator's moniker")]
        moniker: Option<String>,
        #[clap(long = "identity", help = "the keybase.io PGP identity string")]
        identity: Option<String>,

        #[clap(long = "website", help = "public URL")]
        website: Option<String>,

        #[clap(
            long = "security_contact",
            help = "public contact point. (usually email)"
        )]
        security_contact: Option<String>,

        #[clap(long = "details", help = "general text describing the validator")]
        details: Option<String>,

        #[clap(long = "rate", help = "commission rate. 1.00 = 100%")]
        rate: Option<Decimal>,
        #[clap(
            long = "min_self_delegation",
            help = "the minimum amount required for the validator to be active. going under this amount will force the validator to be jailed"
        )]
        min_self_delegation: Option<Decimal>,
    },
    #[clap(name = "delegate")]
    /// Delegate uLuna to a validator
    Delegate {
        /// delegator. The nickname in the wallet used to sign the transaction,use
        delegator: String,
        #[clap(
            name = "validator",
            help = "the validator's terravaloper address to delegate too"
        )]
        /// the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
        amount: Decimal,
    },
    #[clap(name = "redelegate")]
    /// move your delegated uLuna from one validator to another
    ReDelegate {
        /// delegator. The nickname in the wallet used to sign the transaction,use
        delegator: String,
        #[clap(name = "source", help = "the source validator's terravaloper address")]
        /// the validator to transfer funds from. hint: use the terravaloper address.
        source: String,
        #[clap(
            name = "destination",
            help = "the destination validator's terravaloper address"
        )]
        /// the validator to transfer funds from. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        destination: String,
        /// the amount of uLuna to transfer
        amount: Decimal,
    },
    #[clap(name = "unbond")]
    /// start the unbonding process that removes your uLuna from being staked on a validator
    UnBond {
        /// delegator. The nickname in the wallet used to sign the transaction,use
        delegator: String,

        #[clap(name = "validator", help = "the validator's terravaloper address")]
        /// the validator to unbond funds from. hint: use the terravaloper address.
        validator: String,
        /// the amount of uLuna to transfer
        amount: Decimal,
    },
}
/// Staking Commands
#[derive(Parser)]
pub struct StakingCommand {
    #[clap(subcommand)]
    command: StakingEnum,
}
impl StakingCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        let secp = Secp256k1::new();
        match self.command {
            StakingEnum::CreateValidator {
                delegator,
                validator,
                pubkey,
                moniker,
                amount,
                rate,
                max_rate,
                max_change_rate,
                min_self_delegation,
                identity,
                website,
                security_contact,
                details,
            } => {
                let desc = ValidatorDescription::create_create(
                    details,
                    identity,
                    moniker,
                    security_contact,
                    website,
                );
                let commission = ValidatorCommission {
                    max_change_rate,
                    max_rate,
                    rate,
                };

                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
                let delegator_account = delegator_key.public_key(&secp).account()?;
                log::info!("Validator {}", &validator);
                log::info!("Pubkey {}", &pubkey);
                // let validator_key = PublicKey::from_tendermint_key(&validator)?.operator_address()?;
                let coin = Coin::parse(&amount)?.unwrap();
                let msg = MsgCreateValidator::create(
                    desc,
                    commission,
                    min_self_delegation,
                    delegator_account,
                    validator,
                    pubkey,
                    coin,
                );
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
            StakingEnum::EditValidator {
                validator,
                moniker,
                identity,
                website,
                security_contact,
                details,
                rate,
                min_self_delegation,
            } => {
                let desc = ValidatorDescription::create_edit(
                    details,
                    identity,
                    moniker,
                    security_contact,
                    website,
                );
                log::info!("Validator {}", &validator);
                let validator_key = wallet.get_private_key(&secp, &validator, seed)?;
                let validator_operator = validator_key.public_key(&secp).operator_address()?;
                let msg =
                    MsgEditValidator::create(desc, validator_operator, rate, min_self_delegation)?;
                let messages: Vec<Message> = vec![msg];
                let resp = terra
                    .submit_transaction_sync(
                        &secp,
                        &validator_key,
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
            StakingEnum::Delegate {
                delegator,
                validator,
                amount,
            } => {
                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
                let delegator_account = delegator_key.public_key(&secp).account()?;
                let msg = MsgDelegate::create(
                    delegator_account,
                    validator,
                    Coin::create("uluna", amount),
                )?;
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
            StakingEnum::ReDelegate {
                delegator,
                source,
                destination,
                amount,
            } => {
                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
                let delegator_account = delegator_key.public_key(&secp).account()?;
                let msg = MsgBeginRedelegate::create(
                    delegator_account,
                    destination,
                    source,
                    Coin::create("uluna", amount),
                )?;
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
            StakingEnum::UnBond {
                delegator,
                validator,
                amount,
            } => {
                log::info!("Delegator {}", &delegator);
                let delegator_key = wallet.get_private_key(&secp, &delegator, seed)?;
                let delegator_account = delegator_key.public_key(&secp).account()?;
                let msg = MsgUndelegate::create(
                    delegator_account,
                    validator,
                    Coin::create("uluna", amount),
                )?;
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
        }
    }
}
