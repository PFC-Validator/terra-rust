#![allow(missing_docs)]
#![allow(missing_docs)]
use thiserror::Error;
/*
use error_chain::error_chain;

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for Error {
    fn from(e: Box<dyn std::error::Error + Sync + Send>) -> Self {
        Self::from(format!("{:?}", e))
    }
}

#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
error_chain! {

    foreign_links {
       //TerraError ( terra_rust_api::errors::Error);
       KeyringError(keyring::KeyringError);
       SerdeJsonError(serde_json::Error);
    }
    errors {
       TerraWallet(err:String) {
            description("Terra Wallet Error")
            display("Terra Wallet: {}" ,err)
       }
       KeyNotFound(key:String) {
            description("Terra Wallet Key Not Found")
            display("Terra Wallet Key {} not found", key)
       }
    }
}
*/
#[derive(Error, Debug)]
pub enum TerraRustWalletError {
    #[error(transparent)]
    KeyringError(#[from] ::keyring::KeyringError),
    #[error(transparent)]
    SerdeJsonError(#[from] ::serde_json::Error),

    #[error("Terra Wallet `{key}` Key not found Error")]
    KeyNotFound {
        key: String,
        source: keyring::KeyringError,
    },

    #[error("unknown Terra-Rust Wallet error")]
    Unknown,
}
