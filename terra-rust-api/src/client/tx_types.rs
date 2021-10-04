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
    //  #[serde(with = "terra_u64_format")]
    //  pub height: u64,
    /// Transaction hash of the transaction
    pub txhash: String,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TXResultSync {
    //  #[serde(with = "terra_u64_format")]
    //  pub height: u64,
    pub txhash: String,
    pub code: Option<usize>,
    pub raw_log: String,
    pub logs: Option<Vec<TxResultBlockEvent>>,
}
impl TXResultSync {}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TxResultBlockAttribute {
    pub key: String,
    pub value: Option<String>,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct TxResultBlockEvent {
    #[serde(rename = "type")]
    pub s_type: String,
    pub attributes: Vec<TxResultBlockAttribute>,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct TxResultBlockMsg {
    pub msg_index: Option<usize>,
    // pub log: Option<String>,
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
    pub logs: Vec<TxResultBlockMsg>,
    // #[serde(with = "terra_u64_format")]
    // pub gas_wanted: u64,
    // #[serde(with = "terra_u64_format")]
    // pub gas_used: u64,
}
impl TXResultBlock {
    /// find a attribute's value from TX logs.
    /// returns: msg_index and value
    pub fn get_attribute_from_result_logs(
        &self,

        event_type: &str,
        attribute_key: &str,
    ) -> Vec<(usize, String)> {
        let mut response: Vec<(usize, String)> = Default::default();
        for log_part in &self.logs {
            let msg_index = log_part.msg_index.unwrap_or_default();
            let events = &log_part.events;
            //      log::info!("logs{:?}", events);
            let events_filtered = events
                .iter()
                .filter(|event| event.s_type == event_type)
                .collect::<Vec<_>>();
            //      log::info!("Filtered Events {:?}", events_filtered);
            if let Some(event) = events_filtered.first() {
                let attributes_filtered = event
                    .attributes
                    .iter()
                    .filter(|attr| attr.key == attribute_key)
                    .map(|f| f.value.clone())
                    .flatten()
                    .collect::<Vec<_>>();

                if let Some(attr_key) = attributes_filtered.first() {
                    response.push((msg_index, attr_key.clone()));
                }
            }
        }
        response
    }
}
#[derive(Serialize)]
pub struct TxEstimate2<'a> {
    pub msg: &'a [Message],
}
#[derive(Serialize, Debug)]
pub struct TxBaseReq<'a> {
    pub chain_id: String,
    pub from: String,
    #[serde(with = "terra_f64_format")]
    pub gas_adjustment: f64,
    pub gas: String,
    pub gas_prices: &'a [&'a Coin],
}
#[derive(Serialize)]
pub struct TxEstimate<'a> {
    pub base_req: TxBaseReq<'a>,
    pub msgs: &'a [Message],
}
impl<'a> TxEstimate<'a> {
    pub fn create(
        chain_id: &str,
        sender: &str,
        msg: &'a [Message],
        gas_adjustment: f64,
        gas_prices: &'a [&'a Coin],
    ) -> TxEstimate<'a> {
        TxEstimate {
            base_req: TxBaseReq {
                from: sender.into(),
                gas: "auto".into(),
                chain_id: chain_id.into(),
                gas_adjustment,
                gas_prices,
            },
            msgs: msg,
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TxFeeResult {
    pub fee: TxFee,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct TxFee {
    pub amount: Vec<Coin>,
    #[serde(with = "terra_u64_format")]
    pub gas: u64,
}
/*
#[derive(Deserialize, Serialize, Debug)]
pub struct TxFeeResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: TxFeeBlock,
}
*/
