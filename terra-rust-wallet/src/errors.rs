#![allow(missing_docs)]
#![allow(missing_docs)]
use terra_rust_api::errors::TerraRustAPIError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraRustWalletError {
    #[error(transparent)]
    KeyringError(#[from] KeyringErrorAdapter),
    #[error(transparent)]
    SerdeJsonError(#[from] ::serde_json::Error),
    #[error(transparent)]
    KeyringErrorAdapter(#[from] ::keyring::Error),
    #[error(transparent)]
    TerraRustAPIError(#[from] TerraRustAPIError),
    #[error("Terra Wallet `{key}` Key not found Error")]
    KeyNotFound {
        key: String,
        source: KeyringErrorAdapter,
    },

    #[error("unknown Terra-Rust Wallet error")]
    Unknown,
}

/// Workaround type to provide [Sync] on Linux.
///
/// On Linux, [keyring::KeyringError] does not implement `Sync` due to depending
/// on an older version of the `dbus` crate. This prevents usage of `anyhow`. This
/// wrapper is used to bypass that issue on Linux.
#[derive(Error, Debug)]
#[error(transparent)]
pub struct KeyringErrorAdapter(anyhow::Error);

impl From<keyring::Error> for KeyringErrorAdapter {
    fn from(e: keyring::Error) -> Self {
        KeyringErrorAdapter(anyhow::anyhow!("Keyring error: {:?}", e))
    }
}
