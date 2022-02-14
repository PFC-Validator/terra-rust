use anyhow::Result;
use clap::{Parser, Subcommand};

use terra_rust_api::Terra;

use secp256k1::Secp256k1;
use terra_rust_wallet::Wallet;
#[derive(Parser)]
/// Auth operations
pub struct AuthCommand {
    #[clap(subcommand)]
    command: AuthEnum,
}
#[derive(Subcommand)]
pub enum AuthEnum {
    #[clap(name = "account")]
    Account {
        #[clap(name = "address", help = "the address to query")]
        address: String,
    },
    /// list delegations of account
    #[clap(name = "delegations")]
    Delegations {
        #[clap(name = "address", help = "the terra address")]
        // the address to get more info on.
        address: String,
    },
    /// list unbonding delegations of account
    #[clap(name = "unbonding")]
    Unbonding {
        #[clap(name = "address", help = "the terra address")]
        // the address to get more info on.
        address: String,
    },
    /// list validators of account
    #[clap(name = "validators")]
    Validators {
        #[clap(name = "address", help = "the terra address")]
        // the address to get more info on.
        address: String,
    },
}
impl AuthCommand {
    pub async fn parse(self, terra: &Terra, wallet: &Wallet<'_>, seed: Option<&str>) -> Result<()> {
        match self.command {
            AuthEnum::Account { address } => {
                let account_id = if !address.starts_with("terra1") {
                    let secp = Secp256k1::new();
                    wallet.get_account(&secp, &address, seed)?
                } else {
                    address
                };
                let sw = terra.auth().account(&account_id).await?;

                println!("{}", serde_json::to_string_pretty(&sw)?);
            }
            AuthEnum::Delegations { address } => {
                let account_id = if !address.starts_with("terra1") {
                    let secp = Secp256k1::new();
                    wallet.get_account(&secp, &address, seed)?
                } else {
                    address
                };
                let v = terra.auth().validator_delegations(&account_id).await?;
                println!("{:#?}", v.result);
            }
            AuthEnum::Unbonding { address } => {
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
            AuthEnum::Validators { address } => {
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
}
