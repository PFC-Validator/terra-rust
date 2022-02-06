use crate::client::client_types::{
    base64_encoded_format, base64_opt_encoded_format, terra_datetime_format, terra_i64_format,
    terra_u64_format,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockIdParts {
    // #[serde(with = "terra_u64_format")]
    pub total: usize,
    pub hash: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockId {
    pub hash: String,
    pub parts: BlockIdParts,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockHeaderVersion {
    #[serde(with = "terra_u64_format")]
    pub block: u64,
    //  #[serde(with = "terra_u64_format")]
    //  pub app: u64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockHeader {
    pub version: BlockHeaderVersion,
    pub chain_id: String,
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    #[serde(with = "terra_datetime_format")]
    pub time: DateTime<Utc>,
    pub last_block_id: BlockId,
    pub last_commit_hash: String,
    pub data_hash: String,
    pub validators_hash: String,
    pub next_validators_hash: String,
    pub consensus_hash: String,
    pub app_hash: String,
    pub last_results_hash: String,
    pub evidence_hash: String,
    pub proposer_address: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockEvidence {
    //pub evidence: Option<...>
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockSignature {
    /// 1 -no signature .. 2 -- signature
    pub block_id_flag: usize,
    /// HEX/Bytes version of string
    pub validator_address: String,
    #[serde(with = "terra_datetime_format")]
    pub timestamp: DateTime<Utc>,
    pub signature: Option<String>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockCommit {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    // #[serde(with = "terra_u64_format")]
    pub round: usize,
    pub block_id: BlockId,
    pub signatures: Vec<BlockSignature>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockData {
    pub txs: Option<Vec<String>>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub data: BlockData,
    pub evidence: BlockEvidence,
    pub last_commit: BlockCommit,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockResult {
    pub block_id: BlockId,
    pub block: Block,
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EventAttribute {
    #[serde(with = "base64_encoded_format")]
    pub key: String,
    #[serde(with = "base64_opt_encoded_format")]
    pub value: Option<String>,
    pub index: bool,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct EventType {
    #[serde(rename = "type")]
    pub s_type: String,
    pub attributes: Vec<EventAttribute>,
}
impl EventType {
    pub fn attribute_map(&self) -> HashMap<String, Option<String>> {
        self.attributes
            .iter()
            .map(|attr| (attr.key.clone(), attr.value.clone()))
            .collect::<HashMap<String, Option<String>>>()
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RPCTXResult {
    pub code: usize,
    pub data: Option<String>,
    pub log: String,
    pub info: String,
    pub gas_wanted: String,
    pub gas_used: String,
    pub events: Vec<EventType>,
    pub codespace: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RPCPubKeyOuter {
    #[serde(rename = "Sum")]
    pub sum: RPCPubKey,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RPCPubKeyValue {
    pub ed25519: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct RPCPubKey {
    #[serde(rename = "type")]
    pub s_type: String,
    pub value: RPCPubKeyValue,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RPCValidatorUpdate {
    pub pub_key: RPCPubKeyOuter,
    #[serde(with = "terra_u64_format")]
    pub power: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockResultsResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub txs_results: Option<Vec<RPCTXResult>>,
    pub begin_block_events: Option<Vec<EventType>>,
    pub end_block_events: Option<Vec<EventType>>,
    pub validator_updates: Option<Vec<RPCValidatorUpdate>>,
    pub consensus_param_updates: Option<serde_json::Value>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidatorSetResult {
    #[serde(with = "terra_u64_format")]
    pub block_height: u64,
    pub validators: Vec<Validator>,
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TendermintPublicKey {
    #[serde(rename = "type")]
    pub s_type: String,
    pub value: String,
}
#[derive(Deserialize, Clone, Serialize, Debug)]
pub struct Validator {
    pub address: String,
    pub pub_key: TendermintPublicKey,
    #[serde(with = "terra_i64_format")]
    pub proposer_priority: i64,
    #[serde(with = "terra_u64_format")]
    pub voting_power: u64,
}
