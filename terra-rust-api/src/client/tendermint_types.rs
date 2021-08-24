use crate::client::client_types::{terra_datetime_format, terra_i64_format, terra_u64_format};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct BlockIdParts {
    #[serde(with = "terra_u64_format")]
    pub total: u64,
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
    #[serde(with = "terra_u64_format")]
    pub app: u64,
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
    #[serde(with = "terra_u64_format")]
    pub round: u64,
    pub block_id: BlockId,
    pub signatures: Vec<BlockSignature>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct BlockData {
    pub txs: Vec<String>,
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

#[derive(Deserialize, Serialize, Debug)]
pub struct ValidatorSetResult {
    #[serde(with = "terra_u64_format")]
    pub block_height: u64,
    pub validators: Vec<Validator>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct Validator {
    pub address: String,
    pub pub_key: String,
    #[serde(with = "terra_i64_format")]
    pub proposer_priority: i64,
    #[serde(with = "terra_u64_format")]
    pub voting_power: u64,
}
