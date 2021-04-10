use crate::client::client_types::terra_u64_format;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct Coin {
    pub denom: String,
    #[serde(with = "terra_u64_format")]
    pub amount: u64,
}

impl Coin {
    pub fn create(denom: &str, amount: u64) -> Coin {
        Coin {
            denom: denom.to_string(),
            amount,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}{}", self.amount, self.denom)
    }
}
