/// Bank API Transactions
pub mod bank;
pub mod distribution;
/// market messages
pub mod market;
/// Oracle API Transactions
pub mod oracle;
/// slashing messages
pub mod slashing;
/// messages around staking
pub mod staking;
/// regular contract interactions
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
