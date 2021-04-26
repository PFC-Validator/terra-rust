use crate::client::client_types::{terra_f64_format, terra_u64_format};

use crate::core_types::{Coin, Msg};
use serde::{Deserialize, Serialize};

/**
sync: Wait for the tx to pass/fail CheckTx
async: Don't wait for pass/fail CheckTx; send and return tx immediately
block: Wait for the tx to pass/fail CheckTx, DeliverTx, and be committed in a block (not-recommended)

It's best to always use sync.
*/
#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultAsync {
    /// height of the chain when submitted
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    /// Transaction hash of the transaction
    pub txhash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultSync {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub txhash: String,
    pub code: Option<usize>,
    pub raw_log: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultBlockAttribute {
    pub key: String,
    pub value: String,
}
#[derive(Deserialize, Debug)]
pub struct TXResultBlockEvent {
    #[serde(rename = "type")]
    pub sytpe: String,
    pub attributes: Vec<TXResultBlockAttribute>,
}
#[derive(Deserialize, Debug)]
pub struct TXResultBlockMsg {
    pub msg_index: usize,
    pub log: String,
    pub events: Vec<TXResultBlockEvent>,
}
#[derive(Deserialize, Debug)]
pub struct TXResultBlock {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub txhash: String,
    pub codespace: Option<String>,
    pub code: Option<usize>,
    pub raw_log: String,
    pub logs: Vec<TXResultBlockMsg>,
    #[serde(with = "terra_u64_format")]
    pub gas_wanted: u64,
    #[serde(with = "terra_u64_format")]
    pub gas_used: u64,
}

#[derive(Serialize)]
pub struct TXEstimate2<'a> {
    pub msg: &'a Vec<Box<dyn Msg>>,
}
#[derive(Serialize)]
pub struct TXEstimate<'a> {
    pub tx: TXEstimate2<'a>,
    #[serde(with = "terra_f64_format")]
    pub gas_adjustment: f64,
    pub gas_prices: &'a Vec<&'a Coin>,
}
impl<'a> TXEstimate<'a> {
    pub fn create(
        msg: &'a Vec<Box<dyn Msg>>,
        gas_adjustment: f64,
        gas_prices: &'a Vec<&'a Coin>,
    ) -> TXEstimate<'a> {
        TXEstimate {
            tx: TXEstimate2 { msg },
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
pub struct TXFeeResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: TxFeeBlock,
}
