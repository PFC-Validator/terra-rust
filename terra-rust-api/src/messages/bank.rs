//use crate::client::client_types::terra_u64_format;
use crate::core_types::{Coin, MsgInternal};

use crate::messages::Message;
use serde::Serialize;

#[derive(Serialize, Debug)]
/// Message: Send N coins from an address to another

pub struct MsgSend {
    pub amount: Vec<Coin>,
    pub from_address: String,
    pub to_address: String,
}

impl MsgInternal for MsgSend {}
impl MsgSend {
    /// Send amount coins from from_address to to_address
    pub fn create_single(from_address: String, to_address: String, amount: Coin) -> Message {
        MsgSend::create(from_address, to_address, vec![amount])
    }
    /// send multiple coins from from_address to to_address
    pub fn create(from_address: String, to_address: String, amount: Vec<Coin>) -> Message {
        let internal = MsgSend {
            amount,
            from_address,
            to_address,
        };
        Message {
            s_type: "bank/MsgSend".into(),
            value: Box::new(internal),
        }
    }
}
