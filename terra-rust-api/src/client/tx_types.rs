use crate::client::client_types::{terra_f64_format, terra_u64_format};

use crate::core_types::Coin;
use crate::messages::Message;
use serde::{Deserialize, Serialize};

/**
sync: Wait for the tx to pass/fail CheckTx
async: Don't wait for pass/fail CheckTx; send and return tx immediately
block: Wait for the tx to pass/fail CheckTx, DeliverTx, and be committed in a block (not-recommended)

It's best to always use sync.
*/
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultAsync {
    /// height of the chain when submitted
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    /// Transaction hash of the transaction
    pub txhash: String,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultSync {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub txhash: String,
    pub code: Option<usize>,
    pub raw_log: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TxResultBlockAttribute {
    pub key: String,
    pub value: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TxResultBlockEvent {
    #[serde(rename = "type")]
    pub sytpe: String,
    pub attributes: Vec<TxResultBlockAttribute>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TxResultBlockMsg {
    pub msg_index: usize,
    pub log: String,
    pub events: Vec<TxResultBlockEvent>,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultBlock {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub txhash: String,
    pub codespace: Option<String>,
    pub code: Option<usize>,
    pub raw_log: String,
    pub logs: Option<Vec<TxResultBlockMsg>>,
    // #[serde(with = "terra_u64_format")]
    // pub gas_wanted: u64,
    // #[serde(with = "terra_u64_format")]
    // pub gas_used: u64,
}

#[derive(Serialize)]
pub struct TxEstimate2<'a> {
    pub msg: &'a [Message],
}
#[derive(Serialize)]
pub struct TxEstimate<'a> {
    pub tx: TxEstimate2<'a>,
    #[serde(with = "terra_f64_format")]
    pub gas_adjustment: f64,
    pub gas_prices: &'a [&'a Coin],
}
impl<'a> TxEstimate<'a> {
    pub fn create(
        msg: &'a [Message],
        gas_adjustment: f64,
        gas_prices: &'a [&'a Coin],
    ) -> TxEstimate<'a> {
        TxEstimate {
            tx: TxEstimate2 { msg },
            gas_adjustment,
            gas_prices,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TxFeeBlock {
    pub fees: Vec<Coin>,
    #[serde(with = "terra_u64_format")]
    pub gas: u64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TxFeeResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: TxFeeBlock,
}
