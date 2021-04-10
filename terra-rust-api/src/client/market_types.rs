use crate::client::client_types::terra_u64_format;
use crate::client::core_types::Coin;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SwapResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: Coin,
}
