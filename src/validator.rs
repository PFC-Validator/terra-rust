use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
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
    #[structopt(name = "voter")]
    Voters {
        /// the validator to get more info on. hint: use the terravaloper address. try terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q
        validator: String,
        #[structopt(subcommand)]
        cmd: VoterCommand,
    },
}

pub async fn validator_cmd_parse(terra: &Terra<'_>, cmd: ValidatorCommand) -> Result<()> {
    match cmd {
        ValidatorCommand::List => {
            let list = terra.staking().validators().await?;
            /*
            if !list.result.is_empty() {
                let v1 = list.result.get(0).unwrap();
                println!("{:#?}", v1);
            }

             */
            println!("{:#?}", list.result);
        }
        ValidatorCommand::Describe { validator } => {
            let v = terra.staking().validator(&validator).await?;
            println!("{:#?}", v.result);
        }
        ValidatorCommand::Moniker { moniker } => {
            let v = terra.staking().validator_by_moniker(&moniker).await?;
            println!("{:#?}", v);
        }

        ValidatorCommand::Voters { .. } => {
            todo!();
        }
    }
    Ok(())
}
