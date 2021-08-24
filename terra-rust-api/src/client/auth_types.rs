use crate::client::client_types::terra_u64_format;

use crate::core_types::{Coin, PubKeySig};
use serde::{Deserialize, Serialize};

/// This structure serves a few purposes
/// one.. to get the public key (which can be used to validate our private key calcs
/// two.. the account number and sequence fields that are used to generate a signed message
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthAccount {
    /// The account address
    pub address: String,
    /// The balance in the account. Does not include delegated coins
    pub coins: Vec<Coin>,
    /// The public key of the account
    pub public_key: Option<PubKeySig>,
    #[serde(with = "terra_u64_format")]
    /// The account number
    pub account_number: u64,
    /// The sequence. This is used to avoid 'double transmitting' a transaction
    #[serde(with = "terra_u64_format")]
    pub sequence: u64,
}
