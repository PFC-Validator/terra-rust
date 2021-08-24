use crate::client::client_types::{
    terra_datetime_format, terra_decimal_format, terra_f64_format, terra_u64_format,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::Deserialize;

/// Information provided by the validator for their validation node.
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorDescription {
    /// Displayed in public
    pub moniker: String,
    /// link to keybase.io ID
    pub identity: String,
    /// web URL
    pub website: String,
    /// a way to contact the human behind the validator
    pub security_contact: String,
    /// a blurb describing how fantastic the validator is, and why you should validate with them
    pub details: String,
}
/// Commission Rates
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorCommissionRates {
    /// The current commission rate
    #[serde(with = "terra_f64_format")]
    pub rate: f64,
    /// The maximum rate the validator can charge. This can not be altered once set
    #[serde(with = "terra_f64_format")]
    pub max_rate: f64,
    /// How much the rate can change in 24 hours
    #[serde(with = "terra_f64_format")]
    pub max_change_rate: f64,
}
#[allow(missing_docs)]
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorCommission {
    pub commission_rates: ValidatorCommissionRates,
    #[serde(with = "terra_datetime_format")]
    pub update_time: DateTime<Utc>,
}
/// Top level Validator response
#[derive(Deserialize, Clone, Debug)]
pub struct Validator {
    /// The reference address for the validator
    pub operator_address: String,
    /// used in block generation
    pub consensus_pubkey: String,
    /// Is this validator in the active validator set
    pub jailed: bool,
    /// represents the process of being jailed.
    pub status: u16,

    /// Total amount of tokens delegated to the validator
    #[serde(with = "terra_u64_format")]
    pub tokens: u64,
    /// Total amount the validator has delegated themselves.
    #[serde(with = "terra_f64_format")]
    pub delegator_shares: f64,
    /// The validator description structure
    pub description: ValidatorDescription,
    #[serde(with = "terra_u64_format")]
    /// For Jailed / soon to be jailed validators. The height of the chain that it occurred
    pub unbonding_height: u64,
    /// For Jailed / soon to be jailed validators. When that occurred
    #[serde(with = "terra_datetime_format")]
    pub unbonding_time: DateTime<Utc>,
    /// Commission structure
    pub commission: ValidatorCommission,
    /// Minimum amount the validator requires to delegate. Going under this removes the validator from the set
    #[serde(with = "terra_u64_format")]
    pub min_self_delegation: u64,
}

/// Validator Delegation amount
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorDelegationBalance {
    /// token name
    pub denom: String,
    /// amount delegated
    #[serde(with = "terra_decimal_format")]
    pub amount: Decimal,
}
/// Validator Delegation amount
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorDelegation {
    /// account delegating the funds
    pub delegator_address: String,
    /// validator oper address
    pub validator_address: String,
    /// amount delegated
    #[serde(with = "terra_decimal_format")]
    pub shares: Decimal,
    // balance amount
    pub balance: ValidatorDelegationBalance,
}

/// Validator Delegation amount
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorUnbondingDelegationEntry {
    /// block # when unbonding happened
    #[serde(with = "terra_u64_format")]
    pub creation_height: u64,
    /// time when it will complete
    #[serde(with = "terra_datetime_format")]
    pub completion_time: DateTime<Utc>,
    /// initial balance
    #[serde(with = "terra_u64_format")]
    pub initial_balance: u64,
    /// balance
    #[serde(with = "terra_u64_format")]
    pub balance: u64,
}
/// Validator Delegation amount
#[derive(Deserialize, Clone, Debug)]
pub struct ValidatorUnbondingDelegation {
    /// account delegating the funds
    pub delegator_address: String,
    /// validator 'oper' address
    pub validator_address: String,
    /// delegation entries
    pub entries: Vec<ValidatorUnbondingDelegationEntry>,
}
