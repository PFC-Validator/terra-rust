/// Bank API Transactions
pub mod bank;
/// Oracle API Transactions
pub mod oracle;
pub mod staking;

use crate::core_types::MsgInternal;
pub use bank::MsgSend;
use serde::Serialize;

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
