// `error_chain!` can recurse deeply
// #![recursion_limit = "1024"]
#![allow(missing_docs)]
/*!
* This crate provides an interface into the Terra wallet service.
* # PFC
*
* This work is sponsored by the PFC (Productive Framework Code) Validator,
* feel free to delegate to the [PFC](https://station.terra.money/validator/terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q) validator.
*
* It will help defray the costs.
*
* # Warning
* This uses cryptographic routines that have not gone through any security audit.
*
* The manner which it stores private keys may be unsecure/subject to hacks, and if you use it, may put the assets behind those keys at risk.
*
* This is ALPHA software.
*
* # Usage
* TBD
*/

/// Error Messages
pub mod errors;

use crate::errors::KeyringErrorAdapter;
//#[macro_use]
//extern crate error_chain;
use crate::errors::TerraRustWalletError;
use secp256k1::Secp256k1;
use serde::{Deserialize, Serialize};
use terra_rust_api::{PrivateKey, PublicKey};

#[derive(Deserialize, Serialize, Debug)]
/// Internal structure used to hold list of keys in keyring
pub struct WalletInternal {
    pub keys: Vec<String>,
}
#[derive(Deserialize, Serialize, Debug)]
/// Internal structure used to hold list of keys in keyring
pub struct WalletListInternal {
    pub wallets: Vec<String>,
}

///
/// Wallet operations based on Keyring API
///
/// stores key names in another 'username/password' to facilitate listing keys, and deletion of ALL keys in a wallet
pub struct Wallet<'a> {
    pub name: &'a str,
}
impl<'a> Wallet<'a> {
    /// create a new wallet to store keys into. This just creates the structure
    /// use #new to create a new wallet
    pub fn new(wallet_name: &'a str) -> Result<Wallet<'a>, TerraRustWalletError> {
        log::debug!("Creating new wallet {}", wallet_name);
        let wallet = Wallet::create(wallet_name);
        let wallet_list_name = &wallet.full_list_name();
        let keyring = keyring::Keyring::new(wallet_name, wallet_list_name);
        let wallet_internal = WalletInternal { keys: vec![] };
        keyring
            .set_password(&serde_json::to_string(&wallet_internal)?)
            .map_err(KeyringErrorAdapter::from)?;
        let string_key_name: String = String::from(wallet_name);

        match Wallet::get_wallets() {
            Ok(old_list) => {
                let mut new_list: Vec<String> = vec![];
                for s in old_list {
                    if s.ne(wallet_name) {
                        new_list.push(s);
                    }
                }
                new_list.push(string_key_name);
                let wallet_list = WalletListInternal { wallets: new_list };
                Wallet::set_wallets(&wallet_list)?;
            }
            Err(_) => {
                // Keyring just returns a 'generic' error, probably need to dig in and check if it is 'NOTFOUND' vs other
                let wallet_list = WalletListInternal {
                    wallets: vec![string_key_name],
                };
                Wallet::set_wallets(&wallet_list)?;
            }
        }

        Ok(wallet)
    }
    /// setups the wallet structure
    pub fn create(wallet: &'a str) -> Wallet<'a> {
        Wallet { name: wallet }
    }
    /// retrieves the private key from the keyring
    pub fn get_private_key<C: secp256k1::Signing + secp256k1::Context>(
        &self,
        secp: &'a Secp256k1<C>,
        key_name: &'a str,
        seed: Option<&'a str>,
    ) -> Result<PrivateKey, TerraRustWalletError> {
        let full_key_name = self.full_key_name(key_name);
        let keyring = keyring::Keyring::new(self.name, &full_key_name);
        let phrase = &keyring.get_password().map_err(KeyringErrorAdapter::from)?;
        // log::info!("{}", phrase);
        match seed {
            None => Ok(PrivateKey::from_words(secp, phrase, 0, 0)?),
            Some(seed_str) => Ok(PrivateKey::from_words_seed(secp, phrase, seed_str)?),
        }
    }
    /// retrieves the public key associated with the stored private key
    pub fn get_public_key<C: secp256k1::Signing + secp256k1::Context>(
        &self,
        secp: &Secp256k1<C>,
        key_name: &str,
        seed: Option<&str>,
    ) -> Result<PublicKey, TerraRustWalletError> {
        let private_key: PrivateKey = self.get_private_key(secp, key_name, seed)?;

        let pub_key = private_key.public_key(secp);
        Ok(pub_key)
    }

    /// get account from key name
    pub fn get_account<C: secp256k1::Signing + secp256k1::Context>(
        &self,
        secp: &Secp256k1<C>,
        key_name: &str,
        seed: Option<&str>,
    ) -> Result<String, TerraRustWalletError> {
        let pub_key = self.get_public_key(secp, key_name, seed)?;
        let account = pub_key.account()?;
        Ok(account)
    }
    /// stores the private key into the keyring
    pub fn store_key(&self, key_name: &str, pk: &PrivateKey) -> Result<bool, TerraRustWalletError> {
        let full_key_name = self.full_key_name(key_name);

        let keyring = keyring::Keyring::new(self.name, &full_key_name);
        keyring
            .set_password(pk.words().unwrap())
            .map_err(KeyringErrorAdapter::from)?;
        let old_list = self.get_keys()?;
        let string_key_name: String = String::from(key_name);
        let mut new_list: Vec<String> = vec![];
        for s in old_list {
            if s.ne(key_name) {
                new_list.push(s);
            }
        }

        new_list.push(string_key_name);
        let wallet_internal = WalletInternal { keys: new_list };
        self.set_keys(&wallet_internal)?;

        Ok(true)
    }
    /// deletes the private key from the keyring
    pub fn delete_key(&self, key_name: &str) -> Result<bool, TerraRustWalletError> {
        let full_key_name = self.full_key_name(key_name);
        let keyring = keyring::Keyring::new(self.name, &full_key_name);
        keyring
            .delete_password()
            .map_err(KeyringErrorAdapter::from)?;
        let old_list = self.get_keys()?;
        let mut new_list = vec![];
        for s in old_list {
            if s.ne(key_name) {
                new_list.push(s);
            }
        }
        let wallet_internal = WalletInternal { keys: new_list };
        self.set_keys(&wallet_internal)?;
        Ok(true)
    }
    /// lists the keys in the wallet
    pub fn list(&self) -> Result<Vec<String>, TerraRustWalletError> {
        self.get_keys()
    }

    /// deletes the wallet and ALL the keys in the wallet
    pub fn delete(&self) -> Result<(), TerraRustWalletError> {
        let keys = self.get_keys()?;
        for key in keys {
            log::debug!("Deleting Key {} in wallet {}", key, &self.name);
            self.delete_key(&key)?;
        }
        let wallet_list_name = self.full_list_name();
        let keyring = keyring::Keyring::new(self.name, &wallet_list_name);
        keyring
            .delete_password()
            .map_err(KeyringErrorAdapter::from)?;
        let old_list = Wallet::get_wallets()?;
        // let string_key_name: String = String::from(self.name);
        let mut new_list: Vec<String> = vec![];
        for s in old_list {
            if s.ne(self.name) {
                new_list.push(s);
            }
        }
        let wallet_list = WalletListInternal { wallets: new_list };
        Wallet::set_wallets(&wallet_list)?;
        Ok(())
    }
    /// key name format
    fn full_key_name(&self, key_name: &'a str) -> String {
        format!("TERRA-RUST-{}-{}", self.name, key_name)
    }
    /// used to store list of keys for a wallet
    fn full_list_name(&self) -> String {
        format!("TERRA-RUST-{}_KEYS", self.name)
    }
    /// used to store list of wallets
    fn wallet_list_name() -> String {
        "TERRA-RUST_WALLETS".to_string()
    }

    /// get list of keys in a wallet
    fn get_keys(&self) -> Result<Vec<String>, TerraRustWalletError> {
        let wallet_list_name = self.full_list_name();
        let keyring = keyring::Keyring::new(self.name, &wallet_list_name);
        let pass = keyring
            .get_password()
            .map_err(|source| TerraRustWalletError::KeyNotFound {
                key: wallet_list_name,
                source: KeyringErrorAdapter::from(source),
            })?;

        let wallet_internal: WalletInternal = serde_json::from_str(&pass)?;
        Ok(wallet_internal.keys)
    }

    /// get list of wallets
    pub fn get_wallets() -> Result<Vec<String>, TerraRustWalletError> {
        let wallet_list_name = Wallet::wallet_list_name();
        let keyring = keyring::Keyring::new(&wallet_list_name, "wallets");

        let wallet_internal: WalletListInternal =
            serde_json::from_str(&keyring.get_password().map_err(KeyringErrorAdapter::from)?)?;
        Ok(wallet_internal.wallets)
    }

    /// update keys in a wallet
    fn set_keys(&self, int: &WalletInternal) -> Result<(), TerraRustWalletError> {
        let wallet_list_name = self.full_list_name();
        let keyring = keyring::Keyring::new(self.name, &wallet_list_name);

        keyring
            .set_password(&serde_json::to_string(int)?)
            .map_err(KeyringErrorAdapter::from)?;
        Ok(())
    }
    /// update list of wallets
    fn set_wallets(int: &WalletListInternal) -> Result<(), TerraRustWalletError> {
        let wallet_list_name = Wallet::wallet_list_name();
        let keyring = keyring::Keyring::new(&wallet_list_name, "wallets");

        keyring
            .set_password(&serde_json::to_string(int)?)
            .map_err(KeyringErrorAdapter::from)?;
        Ok(())
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    pub fn test_wallet_create_delete() -> anyhow::Result<()> {
        let wallet = Wallet::new("PFC-Test Wallet")?;
        let key_list = wallet.get_keys()?;
        assert!(key_list.is_empty());
        wallet.delete()?;
        Ok(())
    }
    #[test]
    pub fn test_wallet_add_del() -> anyhow::Result<()> {
        let str_1 = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
        let str_2 = "wonder caution square unveil april art add hover spend smile proud admit modify old copper throw crew happy nature luggage reopen exhibit ordinary napkin";

        let s = Secp256k1::new();
        let pk = PrivateKey::from_words(&s, str_1, 0, 0)?;
        let pk2 = PrivateKey::from_words(&s, str_2, 0, 0)?;

        let wallet = Wallet::new("PFC-Test Wallet")?;
        wallet.store_key("PFC-Test-Key", &pk)?;
        let key_list = wallet.get_keys()?;
        assert_eq!(key_list.len(), 1);

        wallet.store_key("PFC-Test-Key-2", &pk2)?;
        let mut key_list = wallet.get_keys()?;
        assert_eq!(key_list.len(), 2);

        key_list.sort();
        assert_eq!(key_list.join(","), "PFC-Test-Key,PFC-Test-Key-2");

        let pk_get = wallet.get_private_key(&s, "PFC-Test-Key", None)?;
        assert_eq!(pk_get.words().unwrap(), str_1);

        wallet.delete_key("PFC-Test-Key")?;
        let key_list = wallet.get_keys()?;
        assert_eq!(key_list.len(), 1);
        assert_eq!(key_list.join(","), "PFC-Test-Key-2");

        wallet.delete()?;
        Ok(())
    }
}
