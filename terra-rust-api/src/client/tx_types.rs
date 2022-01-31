use crate::client::client_types::{terra_datetime_format, terra_f64_format, terra_u64_format};
use chrono::{DateTime, Utc};

use crate::core_types::Coin;
use crate::messages::Message;
use serde::{Deserialize, Serialize};
use serde_json::Value;

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

impl TXResultSync {
    pub fn is_success(&self) -> bool {
        self.code.is_none()
    }
}

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
impl TxResultBlockEvent {
    /// get all key/values from the event that have the key 'key'
    pub fn get_attribute(&self, key: &str) -> Vec<TxResultBlockAttribute> {
        self.attributes
            .iter()
            .filter(|attr| attr.key == key)
            .cloned()
            .collect()
    }
    /// return the first value of the first attribute that has the key 'key'
    pub fn get_first_value(&self, key: &str) -> Option<String> {
        self.get_attribute(key)
            .first()
            .map(|attr| attr.value.clone().unwrap_or_default())
    }
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct TxResultBlockMsg {
    pub msg_index: Option<usize>,
    // pub log: Option<String>,
    pub events: Vec<TxResultBlockEvent>,
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TxBlockMsgInner {
    pub sender: String,
    pub contract: Option<String>,
    pub execute_msg: Option<Value>,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TxBlockMsg {
    pub value: TxBlockMsgInner,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TXBlockValue {
    pub msg: Vec<TxBlockMsg>,
}
#[allow(clippy::upper_case_acronyms)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TXBlock {
    pub value: TXBlockValue,
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
    #[serde(with = "terra_datetime_format")]
    pub timestamp: DateTime<Utc>,
    // #[serde(with = "terra_u64_format")]
    // pub gas_wanted: u64,
    // #[serde(with = "terra_u64_format")]
    // pub gas_used: u64,
    pub tx: Option<TXBlock>,
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

        if let Some(logs) = &self.logs {
            for log_part in logs {
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
        }
        response
    }
    /// get the list of event types from a TX record
    pub fn get_events(&self, event_type: &str) -> Vec<TxResultBlockEvent> {
        let mut response: Vec<TxResultBlockEvent> = Default::default();

        if let Some(logs) = &self.logs {
            for log_part in logs {
                let events = &log_part.events;
                //      log::info!("logs{:?}", events);
                let events_filtered = events
                    .iter()
                    .filter(|event| event.s_type == event_type)
                    .collect::<Vec<_>>();
                for event in events_filtered {
                    response.push(event.clone());
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

#[derive(Deserialize, Serialize, Debug)]
pub struct V1TXBody {
    pub messages: Vec<serde_json::Value>,
    pub memo: String,
    #[serde(with = "terra_u64_format")]
    pub timeout_height: u64,
    pub extension_options: Vec<serde_json::Value>,
    pub non_critical_extension_options: Vec<serde_json::Value>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct V1TX {
    pub body: V1TXBody,
    pub auth_info: serde_json::Value,
    pub signatures: Vec<serde_json::Value>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct V1TXResponse {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub txhash: String,
    pub codespace: String,
    pub code: usize,
    pub data: String,
    pub raw_log: String,
    pub logs: Vec<serde_json::Value>,
    pub info: String,
    #[serde(with = "terra_u64_format")]
    pub gas_wanted: u64,
    #[serde(with = "terra_u64_format")]
    pub gas_used: u64,
    pub tx: serde_json::Value,
    #[serde(with = "terra_datetime_format")]
    pub timestamp: DateTime<Utc>,
    pub events: Option<Vec<serde_json::Value>>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct V1Pagination {
    pub next_key: Option<String>,
    #[serde(with = "terra_u64_format")]
    pub total: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct V1TXSResult {
    pub txs: Vec<V1TX>,
    pub tx_responses: Vec<V1TXResponse>,
    pub pagination: V1Pagination,
}
