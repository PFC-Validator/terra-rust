/// Bank API Transactions
pub mod bank;
/// Oracle API Transactions
pub mod oracle;
/// messages around staking
pub mod staking;
/// regulat contract interactions
pub mod wasm;

use crate::core_types::MsgInternal;
pub use bank::MsgSend;
use serde::Serialize;
pub use wasm::MsgExecuteContract;

#[derive(Serialize)]
/// Message: Send N coins from an address to another
pub struct Message {
    #[allow(missing_docs)]
    #[serde(rename = "type")]
    s_type: String,
    #[allow(missing_docs)]
    value: Box<dyn MsgInternal>,
}
// impl<'a> Msg for Message<'a> {}
