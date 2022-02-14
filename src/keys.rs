use anyhow::Result;
use clap::{Parser, Subcommand};
use std::io::{self, BufRead};
use terra_rust_api::PrivateKey;

use secp256k1::Secp256k1;
use terra_rust_wallet::Wallet;
/// Key Operations
#[derive(Parser)]

pub struct KeysCommand {
    #[clap(subcommand)]
    command: KeysEnum,
}

#[derive(Subcommand)]
pub enum KeysEnum {
    #[clap(name = "parse", about = "parse a hex key into a terra account")]
    Parse {
        #[clap(name = "hex")]
        /// hex public key to convert
        hex: String,
    },
    /// Create a new key to the wallet
    New {
        #[clap(name = "name", help = "a memorable name to use in this client")]
        name: String,
    },
    /// Recover an existing key to the wallet using the recovery words
    Recover {
        #[clap(name = "name", help = "a memorable name to use in this client")]
        name: String,
    },
    /// Delete a key from the wallet
    Delete {
        #[clap(name = "name", help = "delete the private key with this name.")]
        name: String,
    },
    /// Get Public Key representation of the private key
    ///
    Get {
        #[clap(name = "name", help = "the key with this name.")]
        name: String,
    },
    /// List keys in the wallet
    List,
}

impl KeysCommand {
    pub fn parse(self, wallet: &Wallet, seed: Option<&str>) -> Result<()> {
        match self.command {
            KeysEnum::Parse { .. } => {
                todo!()
            }
            KeysEnum::Recover { name } => {
                let secp = Secp256k1::new();

                println!("Please input the set of the recovery words, followed by the passphrase (which is passed via --seed)");
                if seed.is_some() {
                    println!("Your Passphrase is {}", seed.unwrap());
                } else {
                    println!("No Passphrase is being used");
                }
                println!("These will be stored in your computer's vault (win10)/secret service (linux)/keyring (os/x)");
                println!();
                println!("We take NO responsibility for the safety/security of this.");
                println!("This software is ALPHA and has not undergone a security audit");
                println!(
                    "For high value keys, we suggest you always use a hardware wallet, like ledger"
                );
                println!();
                println!("Recovery words:");
                let stdin = io::stdin();
                let mut iterator = stdin.lock().lines();

                let words = iterator.next().unwrap().unwrap();

                let pk = match seed {
                    Some(seed_str) => PrivateKey::from_words_seed(&secp, &words, seed_str)?,
                    None => PrivateKey::from_words(&secp, &words, 0, 0)?,
                };
                wallet.store_key(&name, &pk)?;
            }

            KeysEnum::New { name } => {
                let secp = Secp256k1::new();

                println!("This key will be stored in your computer's vault (win10)/secret service (linux)/keyring (os/x)");

                let pk = match seed {
                    None => PrivateKey::new(&secp)?,
                    Some(seed_str) => PrivateKey::new_seed(&secp, seed_str)?,
                };
                println!("Please write these down and save these in a secure location.");
                println!("These words can be used to transfer all your coins out of your account");
                println!("NO ONE has a need for these keys except you. If they are asking for them it is a scam.");
                println!();
                println!("Your recovery words are:");
                println!("{}", pk.words().unwrap());
                if seed.is_some() {
                    println!("Please also take note of your seed phrase");
                }

                wallet.store_key(&name, &pk)?;

                let pub_key = wallet.get_public_key(&secp, &name, seed)?;

                println!("{}", pub_key.account()?)
            }
            KeysEnum::Delete { name } => {
                wallet.delete_key(&name)?;
            }
            KeysEnum::Get { name } => {
                let secp = Secp256k1::new();
                let pub_key = wallet.get_public_key(&secp, &name, seed)?;

                println!("{}", pub_key.account()?);
            }
            KeysEnum::List => {
                let keys = wallet.list()?;
                println!("{:#?}", keys);
            }
        }
        Ok(())
    }
}
