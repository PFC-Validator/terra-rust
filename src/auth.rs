use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;

use terra_rust_wallet::Wallet;
#[derive(StructOpt)]
pub enum AuthCommand {
    #[structopt(name = "account")]
    Account {
        #[structopt(name = "address", help = "the address to query")]
        address: String,
    },
    /// list delegations of account
    #[structopt(name = "delegations")]
    Delegations {
        #[structopt(name = "address", help = "the terra address")]
        // the address to get more info on.
        address: String,
    },
    /// list unbonding delegations of account
    #[structopt(name = "unbonding")]
    Unbonding {
        #[structopt(name = "address", help = "the terra address")]
        // the address to get more info on.
        address: String,
    },
    /// list validators of account
    #[structopt(name = "validators")]
    Validators {
        #[structopt(name = "address", help = "the terra address")]
        // the address to get more info on.
        address: String,
    },
}

pub async fn auth_cmd_parse<'a>(
    terra: &Terra<'a>,
    _wallet: &Wallet<'a>,
    _seed: Option<&str>,
    auth_cmd: AuthCommand,
) -> Result<()> {
    match auth_cmd {
        AuthCommand::Account { address } => {
            let sw = terra.auth().account(&address).await?;

            println!("{}", serde_json::to_string_pretty(&sw)?);
        }
        AuthCommand::Delegations { address } => {
            let v = terra.auth().validator_delegations(&address).await?;
            println!("{:#?}", v.result);
        }
        AuthCommand::Unbonding { address } => {
            let v = terra
                .auth()
                .validator_unbonding_delegations(&address)
                .await?;
            println!("{:#?}", v.result);
        }
        AuthCommand::Validators { address } => {
            let v = terra.auth().delegated_validators(&address).await?;
            println!("{:#?}", v.result);
        }
    };
    Ok(())
}
