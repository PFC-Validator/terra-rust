//use crate::client::client_types::terra_u64_format;
use crate::core_types::{Coin, Msg};

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
#[allow(missing_docs)]
pub struct MsgSend2 {
    pub(crate) amount: Vec<Coin>,
    pub(crate) from_address: String,
    pub(crate) to_address: String,
}
#[derive(Deserialize, Serialize, Debug)]
/// Message: Send N coins from an address to another
pub struct MsgSend {
    #[allow(missing_docs)]
    #[serde(rename = "type")]
    stype: String,
    #[allow(missing_docs)]
    value: MsgSend2,
}
impl Msg for MsgSend {}
impl MsgSend {
    /// Send amount coins from from_address to to_address
    pub fn create_single(from_address: String, to_address: String, amount: Coin) -> MsgSend {
        let msg = MsgSend2 {
            from_address,
            to_address,
            amount: vec![amount],
        };
        MsgSend {
            stype: String::from("bank/MsgSend"),
            value: msg,
        }
    }
    /// send multiple coins from from_address to to_address
    pub fn create(from_address: String, to_address: String, amount: Vec<Coin>) -> MsgSend {
        let msg = MsgSend2 {
            amount,
            from_address,
            to_address,
        };
        MsgSend {
            stype: String::from("bank/MsgSend"),
            value: msg,
        }
    }
}
