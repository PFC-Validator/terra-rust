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
    pub details: String,
    pub identity: String,
    pub moniker: String,
    pub security_contact: String,
    pub website: String,
}
impl ValidatorDescription {
    pub fn create_create(
        details: Option<String>,
        identity: Option<String>,
        moniker: String,
        security_contact: Option<String>,
        website: Option<String>,
    ) -> ValidatorDescription {
        ValidatorDescription {
            details: details.unwrap_or_else(|| "".into()),
            identity: identity.unwrap_or_else(|| "".into()),
            moniker,
            security_contact: security_contact.unwrap_or_else(|| "".into()),
            website: website.unwrap_or_else(|| "".into()),
        }
    }
    pub fn create_edit(
        details: Option<String>,
        identity: Option<String>,
        moniker: Option<String>,
        security_contact: Option<String>,
        website: Option<String>,
    ) -> ValidatorDescription {
        ValidatorDescription {
            details: details.unwrap_or_else(|| "[do-not-modify]".into()),
            identity: identity.unwrap_or_else(|| "[do-not-modify]".into()),
            moniker: moniker.unwrap_or_else(|| "[do-not-modify]".into()),
            security_contact: security_contact.unwrap_or_else(|| "[do-not-modify]".into()),
            website: website.unwrap_or_else(|| "[do-not-modify]".into()),
        }
    }
}
/// Description of the validator commission structure
#[derive(Deserialize, Serialize, Debug)]
pub struct ValidatorCommission {
    #[serde(with = "terra_decimal_format")]
    pub max_change_rate: Decimal,
    #[serde(with = "terra_decimal_format")]
    pub max_rate: Decimal,
    #[serde(with = "terra_decimal_format")]
    pub rate: Decimal,
}
/// create validator message
#[derive(Serialize, Debug)]
pub struct MsgCreateValidator {
    pub commission: ValidatorCommission,
    pub delegator_address: String,
    pub description: ValidatorDescription,
    #[serde(with = "terra_decimal_format")]
    pub min_self_delegation: Decimal,
    pub pubkey: String,
    pub value: Coin,
    pub validator_address: String,
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
            commission,
            delegator_address,
            description,
            min_self_delegation,
            pubkey,
            value,
            validator_address,
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
    pub address: String,
    #[serde(with = "terra_opt_decimal_format")]
    pub commission_rate: Option<Decimal>,
    pub description: ValidatorDescription,
    #[serde(with = "terra_opt_decimal_format")]
    pub min_self_delegation: Option<Decimal>,
}
impl MsgInternal for MsgEditValidator {}
impl MsgEditValidator {
    pub fn create(
        description: ValidatorDescription,
        address: String,
        commission_rate: Option<Decimal>,
        min_self_delegation: Option<Decimal>,
    ) -> Message {
        let internal = MsgEditValidator {
            address,
            commission_rate,
            description,
            min_self_delegation,
        };
        Message {
            s_type: "staking/MsgEditValidator".into(),
            value: Box::new(internal),
        }
    }
}

/// edit undelegate message
#[derive(Serialize, Debug)]
pub struct MsgUndelegate {
    pub amount: Coin,
    pub delegator_address: String,
    pub validator_address: String,
}
impl MsgInternal for MsgUndelegate {}
impl MsgUndelegate {
    pub fn create(delegator_address: String, validator_address: String, amount: Coin) -> Message {
        let internal = MsgUndelegate {
            amount,
            delegator_address,
            validator_address,
        };
        Message {
            s_type: "staking/MsgUndelegate".into(),
            value: Box::new(internal),
        }
    }
}
/// edit undelegate message
#[derive(Serialize, Debug)]
pub struct MsgDelegate {
    pub amount: Coin,
    pub delegator_address: String,
    pub validator_address: String,
}
impl MsgInternal for MsgDelegate {}
impl MsgDelegate {
    pub fn create(delegator_address: String, validator_address: String, amount: Coin) -> Message {
        let internal = MsgDelegate {
            amount,
            delegator_address,
            validator_address,
        };
        Message {
            s_type: "staking/MsgDelegate".into(),
            value: Box::new(internal),
        }
    }
}
/// edit undelegate message
#[derive(Serialize, Debug)]
pub struct MsgBeginRedelegate {
    pub amount: Coin,
    pub delegator_address: String,
    pub validator_dst_address: String,
    pub validator_src_address: String,
}
impl MsgInternal for MsgBeginRedelegate {}
impl MsgBeginRedelegate {
    pub fn create(
        delegator_address: String,
        validator_dst_address: String,
        validator_src_address: String,
        amount: Coin,
    ) -> Message {
        let internal = MsgBeginRedelegate {
            amount,
            delegator_address,
            validator_src_address,
            validator_dst_address,
        };
        Message {
            s_type: "staking/MsgBeginRedelegate".into(),
            value: Box::new(internal),
        }
    }
}
