use crate::core_types::MsgInternal;
use crate::messages::Message;
use serde::Serialize;

/// withdraw commission from a validator
#[derive(Serialize, Debug)]
pub struct MsgWithdrawValidatorCommission {
    pub validator_address: String,
}
impl MsgInternal for MsgWithdrawValidatorCommission {}
impl MsgWithdrawValidatorCommission {
    pub fn create(validator_address: String) -> Message {
        let internal = MsgWithdrawValidatorCommission { validator_address };
        Message {
            s_type: "distribution/MsgWithdrawValidatorCommission".into(),
            value: Box::new(internal),
        }
    }
}
/// withdraw reward from a validator
#[derive(Serialize, Debug)]
pub struct MsgWithdrawDelegationReward {
    pub delegator_address: String,
    pub validator_address: String,
}
impl MsgInternal for MsgWithdrawDelegationReward {}
impl MsgWithdrawDelegationReward {
    pub fn create(delegator_address: String, validator_address: String) -> Message {
        let internal = MsgWithdrawDelegationReward {
            delegator_address,
            validator_address,
        };
        Message {
            s_type: "distribution/MsgWithdrawDelegationReward".into(),
            value: Box::new(internal),
        }
    }
}
