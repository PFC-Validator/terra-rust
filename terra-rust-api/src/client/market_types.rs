use crate::client::client_types::terra_u64_format;
use crate::client::core_types::Coin;
use serde::{Deserialize, Serialize};

/// Swap Market API

#[derive(Deserialize, Serialize, Debug)]
pub struct SwapResult {
    #[serde(with = "terra_u64_format")]
    #[allow(missing_docs)]
    pub height: u64,
    #[allow(missing_docs)]
    pub result: Coin,
}
