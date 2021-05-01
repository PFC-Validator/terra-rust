use crate::client::client_types::{terra_decimal_format, terra_f64_format, terra_u64_format};

use rust_decimal::Decimal;
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

#[derive(Deserialize, Serialize, Debug)]
pub struct OracleVotes {
    #[serde(with = "terra_decimal_format")]
    pub exchange_rate: Decimal,
    pub denom: String,
    pub voter: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OracleVotesResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: Vec<OracleVotes>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OraclePreVotes {
    pub hash: String,
    pub denom: String,
    pub voter: String,
    #[serde(with = "terra_u64_format")]
    pub submit_block: u64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct OraclePreVotesResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: Vec<OraclePreVotes>,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OracleVoteFeederResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    /// the feeder account
    pub result: String,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct OracleVoteMissResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,

    #[serde(with = "terra_u64_format")]
    /// the number of missess
    pub result: u64,
}
