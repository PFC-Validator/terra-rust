use crate::client::client_types::terra_u64_format;

use serde::{Deserialize, Serialize};

#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmCode {
    #[serde(with = "terra_u64_format")]
    pub code_id: u64,
    pub code_hash: String,
    pub creator: String,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmCodeResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: WasmCode,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmContractInfo {
    pub address: String,
    pub owner: String,
    #[serde(with = "terra_u64_format")]
    pub code_id: u64,
    pub init_msg: String,
    pub migratable: bool,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmContractInfoResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: WasmContractInfo,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmParameter {
    #[serde(with = "terra_u64_format")]
    pub max_contract_size: u64,
    #[serde(with = "terra_u64_format")]
    pub max_contract_gas: u64,
    #[serde(with = "terra_u64_format")]
    pub max_contract_msg_size: u64,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmParameterResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: WasmParameter,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmQueryRaw {
    pub key: String,
    pub value: String,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct WasmQueryRawResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: WasmQueryRaw,
}
