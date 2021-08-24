use crate::client::staking_types::{Validator, ValidatorDelegation, ValidatorUnbondingDelegation};
use crate::{LCDResult, Terra};

pub struct Staking<'a> {
    terra: &'a Terra<'a>,
}
impl Staking<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Staking<'a> {
        Staking { terra }
    }
    pub async fn validator(&self, key: &str) -> anyhow::Result<LCDResult<Validator>> {
        //   let url = self.terra.url.to_owned() + "/staking/validators/" + key;
        self.terra
            .send_cmd::<LCDResult<Validator>>("/staking/validators/", Some(key))
            .await
    }
    /// Get list of validators
    pub async fn validators(&self) -> anyhow::Result<LCDResult<Vec<Validator>>> {
        self.terra
            .send_cmd::<LCDResult<Vec<Validator>>>("/staking/validators", None)
            .await
    }
    pub async fn validator_by_moniker(&self, moniker: &str) -> anyhow::Result<Option<Validator>> {
        let lst = self
            .terra
            .send_cmd::<LCDResult<Vec<Validator>>>("/staking/validators", None)
            .await?
            .result;
        match lst.iter().find(|&p| p.description.moniker == moniker) {
            None => Ok(None),
            Some(v) => Ok(Some(v.to_owned())),
        }
    }
    /// all delegations for a given validator
    pub async fn validator_delegations(
        &self,
        key: &str,
    ) -> anyhow::Result<LCDResult<Vec<ValidatorDelegation>>> {
        self.terra
            .send_cmd::<LCDResult<Vec<ValidatorDelegation>>>(
                &format!("/staking/validators/{}/delegations", key),
                None,
            )
            .await
    }

    /// all unbondings for a given validator
    pub async fn validator_unbonding_delegations(
        &self,
        key: &str,
    ) -> anyhow::Result<LCDResult<Vec<ValidatorUnbondingDelegation>>> {
        self.terra
            .send_cmd::<LCDResult<Vec<ValidatorUnbondingDelegation>>>(
                &format!("/staking/validators/{}/unbonding_delegations", key),
                None,
            )
            .await
    }
}
