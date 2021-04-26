use crate::client::client_types::{terra_f64_format, terra_u64_format};

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct OracleParameterWhiteList {
    pub name: String,
    #[serde(with = "terra_f64_format")]
    pub tobin_tax: f64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OracleParameters {
    #[serde(with = "terra_u64_format")]
    pub vote_period: u64,
    #[serde(with = "terra_f64_format")]
    pub vote_threshold: f64,
    #[serde(with = "terra_f64_format")]
    pub reward_band: f64,
    #[serde(with = "terra_u64_format")]
    pub reward_distribution_window: u64,
    pub whitelist: Vec<OracleParameterWhiteList>,
    #[serde(with = "terra_f64_format")]
    pub slash_fraction: f64,
    #[serde(with = "terra_u64_format")]
    pub slash_window: u64,
    #[serde(with = "terra_f64_format")]
    pub min_valid_per_window: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OracleParametersResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: OracleParameters,
}
