use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_api::Terra;
use terra_rust_wallet::Wallet;

use secp256k1::Secp256k1;

#[derive(Subcommand)]
pub enum VoterCommand {
    Feeder,
    Miss,
    PreVote,
    Vote,
    AggregatorPreVote,
    AggregatorVote,
}

/// validator operations
#[derive(Parser)]
pub struct ValidatorCommand {
    #[clap(subcommand)]
    command: ValidatorEnum,
}
#[derive(Subcommand)]
pub enum ValidatorEnum {
    #[clap(name = "list")]
    // list all validators. Including Jailed ones
    List,
    #[clap(name = "describe")]
    Describe {
        #[clap(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },
    #[clap(name = "moniker")]
    Moniker {
        #[clap(name = "moniker", help = "the validator's moniker")]
        /// the validator to get more info on. try PFC
        moniker: String,
    },
    #[clap(name = "delegations")]
    Delegations {
        #[clap(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },
    #[clap(name = "unbonding")]
    Unbonding {
        #[clap(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },

    #[clap(name = "voter")]
    Voters {
        /// the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
        #[clap(subcommand)]
        cmd: VoterCommand,
    },
}
impl ValidatorCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        match self.command {
            ValidatorEnum::List => {
                let list = terra.staking().validators().await?;
                println!("{:#?}", list.result);
            }
            ValidatorEnum::Describe { validator } => {
                let account_id = if !validator.starts_with("terravaloper1") {
                    let secp = Secp256k1::new();
                    wallet
                        .get_public_key(&secp, &validator, seed)?
                        .operator_address()?
                } else {
                    validator
                };
                let v = terra.staking().validator(&account_id).await?;
                println!("{:#?}", v.result);
            }
            ValidatorEnum::Moniker { moniker } => {
                let v = terra.staking().validator_by_moniker(&moniker).await?;
                println!("{:#?}", v);
            }
            ValidatorEnum::Delegations { validator } => {
                let account_id = if !validator.starts_with("terravaloper1") {
                    let secp = Secp256k1::new();
                    wallet
                        .get_public_key(&secp, &validator, seed)?
                        .operator_address()?
                } else {
                    validator
                };
                let v = terra.staking().validator_delegations(&account_id).await?;
                println!("{:#?}", v.result);
            }
            ValidatorEnum::Unbonding { validator } => {
                let account_id = if !validator.starts_with("terravaloper1") {
                    let secp = Secp256k1::new();
                    wallet
                        .get_public_key(&secp, &validator, seed)?
                        .operator_address()?
                } else {
                    validator
                };
                let v = terra
                    .staking()
                    .validator_unbonding_delegations(&account_id)
                    .await?;
                println!("{:#?}", v.result);
            }

            ValidatorEnum::Voters { .. } => {
                todo!();
            }
        }
        Ok(())
    }
}
