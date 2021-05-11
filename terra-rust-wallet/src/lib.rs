// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]
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
* This uses crytpographic routines that have not gone through any security audit.
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

//#[macro_use]
extern crate error_chain;

use bitcoin::secp256k1::All;
use bitcoin::secp256k1::Secp256k1;
use errors::Result;
use serde::{Deserialize, Serialize};
use terra_rust_api::{PrivateKey, PublicKey};

#[derive(Deserialize, Serialize, Debug)]
pub struct WalletInternal {
    pub keys: Vec<String>,
}
///
/// Wallet operations based on Keyring API
pub struct Wallet<'a> {
    name: &'a str,
}
impl<'a> Wallet<'a> {
    /// create a new wallet to store keys into
    pub fn new(wallet_name: &'a str) -> Result<Wallet<'a>> {
        log::debug!("Creating new wallet {}", wallet_name);
        let wallet = Wallet::create(wallet_name);
        let wallet_list_name = &wallet.full_list_name();
        let keyring = keyring::Keyring::new(wallet_name, wallet_list_name);
        let wallet_internal = WalletInternal { keys: vec![] };
        keyring.set_password(&serde_json::to_string(&wallet_internal)?)?;

        Ok(wallet)
    }
    /// setups the wallet structure
    pub fn create(wallet: &'a str) -> Wallet<'a> {
        Wallet { name: wallet }
    }
    /// retrieves the private key from the keyring
    pub fn get_private_key(
        &self,
        secp: &Secp256k1<All>,
        key_name: &str,
        seed: Option<&str>,
    ) -> Result<PrivateKey> {
        let full_key_name = self.full_key_name(key_name);
        //  log::debug!("Wallet/Key - {}/{}", &self.name, &full_key_name);
        let keyring = keyring::Keyring::new(&self.name, &full_key_name);
        let phrase = &keyring.get_password()?;

        match seed {
            None => Ok(PrivateKey::from_words(secp, phrase)?),
            Some(seed_str) => Ok(PrivateKey::from_words_seed(secp, phrase, seed_str)?),
        }
    }
    /// retrieves the public key associated with the stored private key
    pub fn get_public_key(
        &self,
        secp: &Secp256k1<All>,
        key_name: &str,
        seed: Option<&str>,
    ) -> Result<PublicKey> {
        let private_key: PrivateKey = self.get_private_key(secp, key_name, seed)?;

        let pub_key = private_key.public_key(&secp);
        Ok(pub_key)
    }
    /// stores the private key into the keyring
    pub fn store_key(&self, key_name: &str, pk: &PrivateKey) -> Result<bool> {
        let full_key_name = self.full_key_name(key_name);

        let keyring = keyring::Keyring::new(&self.name, &full_key_name);
        keyring.set_password(pk.words().unwrap())?;
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
    pub fn delete_key(&self, key_name: &str) -> Result<bool> {
        let full_key_name = self.full_key_name(key_name);
        let keyring = keyring::Keyring::new(&self.name, &full_key_name);
        keyring.delete_password()?;
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
    pub fn list(&self) -> Result<Vec<String>> {
        self.get_keys()
    }
    // deletes the wallet and ALL the keys in the wallet
    pub fn delete(&self) -> Result<()> {
        let keys = self.get_keys()?;
        for key in keys {
            log::debug!("Deleting Key {} in wallet {}", key, &self.name);
            self.delete_key(&key)?;
        }
        let wallet_list_name = self.full_list_name();
        let keyring = keyring::Keyring::new(&self.name, &wallet_list_name);
        keyring.delete_password()?;

        Ok(())
    }
    /// key name format
    fn full_key_name(&self, key_name: &'a str) -> String {
        format!("TERRA-RUST-{}-{}", self.name, key_name)
    }
    fn full_list_name(&self) -> String {
        format!("TERRA-RUST-{}_KEYS", self.name)
    }

    fn get_keys(&self) -> Result<Vec<String>> {
        let wallet_list_name = self.full_list_name();
        let keyring = keyring::Keyring::new(&self.name, &wallet_list_name);

        let wallet_internal: WalletInternal = serde_json::from_str(&keyring.get_password()?)?;
        Ok(wallet_internal.keys)
    }
    fn set_keys(&self, int: &WalletInternal) -> Result<()> {
        let wallet_list_name = self.full_list_name();
        let keyring = keyring::Keyring::new(&self.name, &wallet_list_name);

        keyring.set_password(&serde_json::to_string(int)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tst {
    use super::*;

    #[test]
    pub fn test_wallet_create_delete() -> Result<()> {
        let wallet = Wallet::new("PFC-Test Wallet")?;
        let key_list = wallet.get_keys()?;
        assert!(key_list.is_empty());
        wallet.delete()?;
        Ok(())
    }
    #[test]
    pub fn test_wallet_add_del() -> Result<()> {
        let str_1 = "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius";
        let str_2 = "wonder caution square unveil april art add hover spend smile proud admit modify old copper throw crew happy nature luggage reopen exhibit ordinary napkin";

        let s = Secp256k1::new();
        let pk = PrivateKey::from_words(&s, str_1)?;
        let pk2 = PrivateKey::from_words(&s, str_2)?;

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
