use anyhow::Result;
//use crate::errors::Result;
use structopt::StructOpt;
use terra_rust_api::Terra;
use terra_rust_wallet::Wallet;
#[derive(StructOpt)]
pub enum WalletCommand {
    #[structopt(name = "create", help = "create a wallet")]
    Create {
        #[allow(dead_code)]
        #[structopt(name = "name", help = "name of the wallet")]
        name: String,
    },
    #[structopt(name = "list", help = "List available wallets")]
    List,
    #[structopt(name = "delete", help = "delete a wallet")]
    Delete {
        #[structopt(name = "name", help = "name of the wallet")]
        #[allow(dead_code)]
        name: String,
    },
}

pub fn wallet_cmd_parse(
    _terra: &Terra,
    wallet: &Wallet,
    _seed: Option<&str>,
    cmd: WalletCommand,
) -> Result<()> {
    match cmd {
        WalletCommand::Create { name } => {
            Wallet::new(&name)?;
            println!("Wallet {} created", name);
            Ok(())
        }
        WalletCommand::List => {
            let wallets = Wallet::get_wallets()?;
            println!("{:#?}", wallets);
            Ok(())
        }
        WalletCommand::Delete { name } => {
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
