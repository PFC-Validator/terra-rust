//use crate::client::client_types::terra_u64_format;
use crate::core_types::{Coin, MsgInternal};

use crate::messages::Message;
use serde::Serialize;

#[derive(Serialize, Debug)]
/// swap a coin, and send it to someone

pub struct MsgSwap {
    pub ask_denom: String,
    pub offer_coin: Coin,
    /// to account
    pub trader: String,
}

impl MsgInternal for MsgSwap {}
impl MsgSwap {
    /// swap a coin, and send it to someone
    pub fn create(offer_coin: Coin, ask_denom: String, trader: String) -> Message {
        let internal = MsgSwap {
            ask_denom,
            offer_coin,
            trader,
        };
        Message {
            s_type: "market/MsgSwap".into(),
            value: Box::new(internal),
        }
    }
}
