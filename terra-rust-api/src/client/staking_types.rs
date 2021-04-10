use crate::client::client_types::{terra_datetime_format, terra_f64_format, terra_u64_format};
use chrono::{DateTime, Utc};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ValidatorDescription {
    moniker: String,
    identity: String,
    website: String,
    security_contact: String,
    details: String,
}
#[derive(Deserialize, Debug)]
pub struct ValidatorCommissionRates {
    #[serde(with = "terra_f64_format")]
    rate: f64,
    #[serde(with = "terra_f64_format")]
    max_rate: f64,
    #[serde(with = "terra_f64_format")]
    max_change_rate: f64,
}
#[derive(Deserialize, Debug)]
pub struct ValidatorCommission {
    commission_rates: ValidatorCommissionRates,
    #[serde(with = "terra_datetime_format")]
    update_time: DateTime<Utc>,
}
#[derive(Deserialize, Debug)]
pub struct Validator {
    operator_address: String,
    consensus_pubkey: String,
    jailed: bool,
    status: u16,

    #[serde(with = "terra_u64_format")]
    tokens: u64,
    #[serde(with = "terra_f64_format")]
    delegator_shares: f64,
    description: ValidatorDescription,
    #[serde(with = "terra_u64_format")]
    unbonding_height: u64,
    #[serde(with = "terra_datetime_format")]
    unbonding_time: DateTime<Utc>,
    commission: ValidatorCommission,
    #[serde(with = "terra_u64_format")]
    min_self_delegation: u64,
}
#[derive(Deserialize, Debug)]
pub struct ValidatorResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: Validator,
}

#[derive(Deserialize, Debug)]
pub struct ValidatorListResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: Vec<Validator>,
}
