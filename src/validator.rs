use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
use terra_rust_wallet::Wallet;

use secp256k1::Secp256k1;
//use crate::errors::Result;
#[derive(StructOpt)]
pub enum VoterCommand {
    Feeder,
    Miss,
    PreVote,
    Vote,
    AggregatorPreVote,
    AggregatorVote,
}

#[derive(StructOpt)]
pub enum ValidatorCommand {
    #[structopt(name = "list")]
    // list all validators. Including Jailed ones
    List,
    #[structopt(name = "describe")]
    Describe {
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },
    #[structopt(name = "moniker")]
    Moniker {
        #[structopt(name = "moniker", help = "the validator's moniker")]
        /// the validator to get more info on. try PFC
        moniker: String,
    },
    #[structopt(name = "delegations")]
    Delegations {
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },
    #[structopt(name = "unbonding")]
    Unbonding {
        #[structopt(name = "validator", help = "the validator's terravaloper address")]
        // the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
    },

    #[structopt(name = "voter")]
    Voters {
        /// the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
        #[structopt(subcommand)]
        cmd: VoterCommand,
    },
}

pub async fn validator_cmd_parse<'a>(
    terra: &Terra,
    wallet: &Wallet<'a>,
    seed: Option<&str>,
    cmd: ValidatorCommand,
) -> Result<()> {
    match cmd {
        ValidatorCommand::List => {
            let list = terra.staking().validators().await?;
            println!("{:#?}", list.result);
        }
        ValidatorCommand::Describe { validator } => {
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
        ValidatorCommand::Moniker { moniker } => {
            let v = terra.staking().validator_by_moniker(&moniker).await?;
            println!("{:#?}", v);
        }
        ValidatorCommand::Delegations { validator } => {
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
        ValidatorCommand::Unbonding { validator } => {
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

        ValidatorCommand::Voters { .. } => {
            todo!();
        }
    }
    Ok(())
}
