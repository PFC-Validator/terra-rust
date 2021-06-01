use structopt::StructOpt;
use terra_rust_api::Terra;

use crate::errors::Result;
use crate::{NAME, VERSION};
use bitcoin::secp256k1::Secp256k1;
use rust_decimal::Decimal;
use terra_rust_api::core_types::Coin;
use terra_rust_api::messages::staking::{
    MsgCreateValidator, ValidatorCommission, ValidatorDescription,
};
use terra_rust_api::messages::Message;
use terra_rust_wallet::Wallet;

#[derive(StructOpt)]
pub enum StakingCommand {
    #[structopt(name = "create-validator")]
    /// Create a validator
    CreateValidator {
        /// delegator. The nickname in the wallet used to sign the transaction, and transfer the initial amount
        delegator: String,

        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        /// the validator's publickey  terravalconspub1XXXXXXXXX. try  terrad tendermint show-validator
        validator: String,
        #[structopt(
            name = "pubkey",
            help = "the validator's tendermint consensus public key"
        )]
        /// the validator's tendermint consensus public key
        pubkey: String,
        #[structopt(name = "moniker", help = "the validator's moniker")]
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

        /// [Optional] the keybase.io PGP identity string.
        identity: Option<String>,
        /// [Optional] public URL
        website: Option<String>,
        /// [Optional] public contact point. (usually email)
        security_contact: Option<String>,
        /// [Optional] general text describing the validator
        details: Option<String>,
    },
    #[structopt(name = "edit-validator")]
    /// edit a validator's details
    EditValidator {
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        /// the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,

        #[structopt(name = "moniker", help = "the validator's moniker")]
        moniker: String,
        /// [Optional] the keybase.io PGP identity string.
        identity: Option<String>,
        /// [Optional] public URL
        website: Option<String>,
        /// [Optional] public contact point. (usually email)
        security_contact: Option<String>,
        /// [Optional] general text describing the validator
        details: Option<String>,
    },
    #[structopt(name = "delegate")]
    /// Delegate uLuna to a validator
    Delegate {
        #[structopt(
            name = "validator",
            help = "the validator's terravaloper address to delegate too"
        )]
        /// the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
        amount: Decimal,
    },
    #[structopt(name = "redelegate")]
    /// move your delegated uLuna from one validator to another
    ReDelegate {
        #[structopt(name = "source", help = "the source validator's terravaloper address")]
        /// the validator to transfer funds from. hint: use the terravaloper address.
        source: String,
        #[structopt(
            name = "destination",
            help = "the destination validator's terravaloper address"
        )]
        /// the validator to transfer funds from. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        destination: String,
        /// the amount of uLuna to transfer
        amount: Decimal,
    },
    #[structopt(name = "unbond")]
    /// start the unbonding process that removes your uLuna from being staked on a validator
    UnBond {
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        /// the validator to transfer funds from. hint: use the terravaloper address.
        validator: String,
        /// the amount of uLuna to transfer
        amount: Decimal,
    },
}

pub async fn staking_cmd_parse<'a>(
    terra: &Terra<'a>,
    wallet: &Wallet<'a>,
    seed: Option<&str>,
    cmd: StakingCommand,
) -> Result<()> {
    match cmd {
        StakingCommand::CreateValidator {
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
                moniker,
                identity,
                website,
                security_contact,
                details,
            );
            let commission = ValidatorCommission {
                max_change_rate,
                max_rate,
                rate,
            };
            let secp = Secp256k1::new();
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
            let (std_sign_msg, sigs) = terra
                .generate_transaction_to_broadcast(
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

            let resp = terra.tx().broadcast_sync(&std_sign_msg, &sigs).await?;
            match resp.code {
                Some(code) => {
                    log::error!("{}", serde_json::to_string(&resp)?);
                    eprintln!("Transaction returned a {} {}", code, resp.txhash)
                }
                None => {
                    println!("{}", resp.txhash)
                }
            };

            Ok(())
        }
        StakingCommand::EditValidator {
            validator,

            moniker,
            identity,
            website,
            security_contact,
            details,
        } => {
            let _v = validator;
            let _desc = ValidatorDescription::create_create(
                moniker,
                identity,
                website,
                security_contact,
                details,
            );
            todo!()
        }
        StakingCommand::Delegate { .. } => {
            todo!()
        }
        StakingCommand::ReDelegate { .. } => {
            todo!()
        }
        StakingCommand::UnBond { .. } => {
            todo!()
        }
    }
}
