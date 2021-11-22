use anyhow::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;

use secp256k1::Secp256k1;
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

pub async fn auth_cmd_parse(
    terra: &Terra,
    wallet: &Wallet<'_>,
    seed: Option<&str>,
    auth_cmd: AuthCommand,
) -> Result<()> {
    match auth_cmd {
        AuthCommand::Account { address } => {
            let account_id = if !address.starts_with("terra1") {
                let secp = Secp256k1::new();
                wallet.get_account(&secp, &address, seed)?
            } else {
                address
            };
            let sw = terra.auth().account(&account_id).await?;

            println!("{}", serde_json::to_string_pretty(&sw)?);
        }
        AuthCommand::Delegations { address } => {
            let account_id = if !address.starts_with("terra1") {
                let secp = Secp256k1::new();
                wallet.get_account(&secp, &address, seed)?
            } else {
                address
            };
            let v = terra.auth().validator_delegations(&account_id).await?;
            println!("{:#?}", v.result);
        }
        AuthCommand::Unbonding { address } => {
            let account_id = if !address.starts_with("terra1") {
                let secp = Secp256k1::new();
                wallet.get_account(&secp, &address, seed)?
            } else {
                address
            };
            let v = terra
                .auth()
                .validator_unbonding_delegations(&account_id)
                .await?;
            println!("{:#?}", v.result);
        }
        AuthCommand::Validators { address } => {
            let account_id = if !address.starts_with("terra1") {
                let secp = Secp256k1::new();
                wallet.get_account(&secp, &address, seed)?
            } else {
                address
            };
            let v = terra.auth().delegated_validators(&account_id).await?;
            println!("{:#?}", v.result);
        }
    };
    Ok(())
}
