use anyhow::Result;
use clap::{Parser, Subcommand};
use terra_rust_wallet::Wallet;
#[derive(Parser)]
/// wallet ops
pub struct WalletCommand {
    #[clap(subcommand)]
    command: WalletEnum,
}
#[derive(Subcommand)]
enum WalletEnum {
    /// create a wallet
    #[clap(name = "create")]
    Create {
        /// name of the wallet
        #[clap(name = "name")]
        name: String,
    },
    /// list available wallets
    #[clap(name = "list")]
    List,
    /// delete a wallet
    #[clap(name = "delete")]
    Delete {
        /// name of wallet
        #[clap(name = "name")]
        name: String,
    },
}
impl WalletCommand {
    pub fn parse(self, wallet: &Wallet) -> Result<()> {
        match self.command {
            WalletEnum::Create { name } => {
                Wallet::new(&name)?;
                println!("Wallet {} created", name);
                Ok(())
            }
            WalletEnum::List => {
                let wallets = Wallet::get_wallets()?;
                println!("{:#?}", wallets);
                Ok(())
            }
            WalletEnum::Delete { name } => {
                if name.eq(&wallet.name) {
                    wallet.delete()?;
                    Ok(())
                } else {
                    eprintln!("you will need to specify the wallet as a --wallet parameter as well as the wallet name");
                    eprintln!("Wallet name is currently set to {}", &wallet.name);
                    Ok(())
                }
            }
        }
    }
}
