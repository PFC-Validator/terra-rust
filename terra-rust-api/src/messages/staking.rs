use crate::client::client_types::{terra_decimal_format, terra_opt_decimal_format};
use crate::core_types::{Coin, MsgInternal};

use crate::messages::Message;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
/// Description of the validator which appears in public
/// on MsgEditValidator messages for fields that don't change you should put "[do-not-modify]"
///
pub struct ValidatorDescription {
    pub moniker: String,
    pub identity: String,
    pub website: String,
    pub security_contact: String,
    pub details: String,
}
impl ValidatorDescription {
    pub fn create_create(
        moniker: String,
        identity: Option<String>,
        website: Option<String>,
        security_contact: Option<String>,
        details: Option<String>,
    ) -> ValidatorDescription {
        ValidatorDescription {
            moniker,
            identity: identity.unwrap_or_else(|| "".into()),
            website: website.unwrap_or_else(|| "".into()),
            security_contact: security_contact.unwrap_or_else(|| "".into()),
            details: details.unwrap_or_else(|| "".into()),
        }
    }
    pub fn create_edit(
        moniker: String,
        identity: Option<String>,
        website: Option<String>,
        security_contact: Option<String>,
        details: Option<String>,
    ) -> ValidatorDescription {
        ValidatorDescription {
            moniker,
            identity: identity.unwrap_or_else(|| "[do-not-modify]".into()),
            website: website.unwrap_or_else(|| "[do-not-modify]".into()),
            security_contact: security_contact.unwrap_or_else(|| "[do-not-modify]".into()),
            details: details.unwrap_or_else(|| "[do-not-modify]".into()),
        }
    }
}
/// Description of the validator commission structure
#[derive(Deserialize, Serialize, Debug)]
pub struct ValidatorCommission {
    #[serde(with = "terra_decimal_format")]
    pub rate: Decimal,
    #[serde(with = "terra_decimal_format")]
    pub max_rate: Decimal,
    #[serde(with = "terra_decimal_format")]
    pub max_change_rate: Decimal,
}
/// create validator message
#[derive(Serialize, Debug)]
pub struct MsgCreateValidator {
    pub description: ValidatorDescription,
    pub commission: ValidatorCommission,
    #[serde(with = "terra_decimal_format")]
    pub min_self_delegation: Decimal,
    pub delegator_address: String,
    pub validator_address: String,
    pub pubkey: String,
    pub value: Coin,
}
impl MsgInternal for MsgCreateValidator {}
impl MsgCreateValidator {
    pub fn create(
        description: ValidatorDescription,
        commission: ValidatorCommission,
        min_self_delegation: Decimal,
        delegator_address: String,
        validator_address: String,
        pubkey: String,
        value: Coin,
    ) -> Message {
        let internal = MsgCreateValidator {
            description,
            commission,
            min_self_delegation,
            delegator_address,
            validator_address,
            pubkey,
            value,
        };
        Message {
            s_type: "staking/MsgCreateValidator".into(),
            value: Box::new(internal),
        }
    }
}
/// edit validator message
#[derive(Serialize, Debug)]
pub struct MsgEditValidator {
    pub description: ValidatorDescription,
    pub address: String,
    #[serde(with = "terra_decimal_format")]
    pub commission_rate: Decimal,
    #[serde(with = "terra_opt_decimal_format")]
    pub min_self_delegation: Option<Decimal>,
}
impl MsgInternal for MsgEditValidator {}
impl MsgEditValidator {
    pub fn create(
        description: ValidatorDescription,
        address: String,
        commission_rate: Decimal,
        min_self_delegation: Option<Decimal>,
    ) -> Message {
        let internal = MsgEditValidator {
            description,
            address,
            commission_rate,
            min_self_delegation,
        };
        Message {
            s_type: "staking/MsgEditValidator".into(),
            value: Box::new(internal),
        }
    }
}
