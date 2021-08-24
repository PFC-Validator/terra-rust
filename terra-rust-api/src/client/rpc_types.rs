use crate::client::client_types::terra_u64_format;

use serde::Deserialize;
use serde::Serialize;

/// Information provided by the validator for their node info
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCProtocolVersion {
    pub p2p: String,
    pub block: String,
    pub app: String,
}
/// Information provided by the validator for their node info
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCProtocolOther {
    pub tx_index: String,
    pub rpc_address: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCNodeInfo {
    pub protocol_version: RPCProtocolVersion,
    pub id: String,
    pub listen_addr: String,
    pub network: String,
    pub version: String,
    pub channels: String,
    pub moniker: String,
    pub other: RPCProtocolOther,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCSyncInfo {
    pub catching_up: bool,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCValidatorInfo {
    pub address: String,
    #[serde(with = "terra_u64_format")]
    pub voting_power: u64,
}
/// Information provided by the validator for their status
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCStatus {
    pub node_info: RPCNodeInfo,
    pub sync_info: RPCSyncInfo,
    pub validator_info: RPCValidatorInfo,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCConnectionStatus {
    #[serde(with = "terra_u64_format", rename = "Duration")]
    pub duration: u64,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCNetPeer {
    pub node_info: RPCNodeInfo,
    pub is_outbound: bool,
    pub connection_status: RPCConnectionStatus,
    pub remote_ip: String,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCNetInfo {
    pub listening: bool,
    #[serde(with = "terra_u64_format")]
    pub n_peers: u64,
    pub peers: Vec<RPCNetPeer>,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct RPCUnconfirmedTXS {
    #[serde(with = "terra_u64_format")]
    pub n_txs: u64,
    #[serde(with = "terra_u64_format")]
    pub total: u64,
    #[serde(with = "terra_u64_format")]
    pub total_bytes: u64,
    pub txs: Vec<String>,
}

#[allow(missing_docs)]
#[derive(Deserialize, Debug)]
pub struct RPCResult<T> {
    pub jsonrpc: String,
    pub id: i64,
    pub result: T,
}
