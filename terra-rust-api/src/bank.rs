//use crate::client::client_types::terra_u64_format;
use crate::core_types::{Coin, Msg};

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
pub struct MsgSend2 {
    pub(crate) from_address: String,
    pub(crate) to_address: String,
    pub(crate) amount: Vec<Coin>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct MsgSend {
    #[serde(rename = "type")]
    stype: String,
    value: MsgSend2,
}
impl Msg for MsgSend {}
impl MsgSend {
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
    pub fn create(from_address: String, to_address: String, amount: Vec<Coin>) -> MsgSend {
        let msg = MsgSend2 {
            from_address,
            to_address,
            amount,
        };
        MsgSend {
            stype: String::from("bank/MsgSend"),
            value: msg,
        }
    }
}
