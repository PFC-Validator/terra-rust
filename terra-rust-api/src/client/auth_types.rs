use crate::client::client_types::{terra_opt_u64_format, terra_u64_format};

use crate::core_types::PubKeySig;
use serde::{Deserialize, Serialize};

/// This structure serves a few purposes
/// one.. to get the public key (which can be used to validate our private key calcs
/// two.. the account number and sequence fields that are used to generate a signed message
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AuthAccount {
    /// The account address
    pub address: String,
    /// The balance in the account. Does not include delegated coins
    pub public_key: Option<PubKeySig>,
    #[serde(with = "terra_u64_format")]
    /// The account number
    pub account_number: u64,
    /// The sequence. This is used to avoid 'double transmitting' a transaction
    #[serde(default, with = "terra_opt_u64_format")]
    pub sequence: Option<u64>,
}
