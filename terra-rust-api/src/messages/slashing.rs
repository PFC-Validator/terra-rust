use crate::core_types::MsgInternal;

use crate::messages::Message;
use serde::Serialize;

/// Unjail message
#[derive(Serialize, Debug)]
pub struct MsgUnjail {
    pub address: String,
}
impl MsgInternal for MsgUnjail {}
impl MsgUnjail {
    pub fn create(address: String) -> Message {
        let internal = MsgUnjail { address };
        Message {
            s_type: "slashing/MsgUnjail".into(),
            value: Box::new(internal),
        }
    }
}
