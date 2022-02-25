use crate::client::staking_types::{Validator, ValidatorDelegation, ValidatorUnbondingDelegation};
use crate::errors::TerraRustAPIError;
use crate::staking_types::ValidatorDelegationsV1Response;
use crate::{LCDResult, Terra};

pub struct Staking<'a> {
    terra: &'a Terra,
}
impl Staking<'_> {
    pub fn create(terra: &'_ Terra) -> Staking<'_> {
        Staking { terra }
    }
    pub async fn validator(&self, key: &str) -> Result<LCDResult<Validator>, TerraRustAPIError> {
        //   let url = self.terra.url.to_owned() + "/staking/validators/" + key;
        Ok(self
            .terra
            .send_cmd::<LCDResult<Validator>>("/staking/validators/", Some(key))
            .await?)
    }
    /// Get list of validators
    pub async fn validators(&self) -> Result<LCDResult<Vec<Validator>>, TerraRustAPIError> {
        Ok(self
            .terra
            .send_cmd::<LCDResult<Vec<Validator>>>("/staking/validators", None)
            .await?)
    }
    /// Get list of validators at a given height
    pub async fn validators_at_height(
        &self,
        height: u64,
        limit: Option<u64>,
    ) -> Result<LCDResult<Vec<Validator>>, TerraRustAPIError> {
        Ok(self
            .terra
            .send_cmd::<LCDResult<Vec<Validator>>>(
                &format!(
                    "/staking/validators?height={}&limit={}",
                    height,
                    limit.unwrap_or(200u64)
                ),
                None,
            )
            .await?)
    }
    pub async fn validator_by_moniker(
        &self,
        moniker: &str,
    ) -> Result<Option<Validator>, TerraRustAPIError> {
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
    ) -> Result<LCDResult<Vec<ValidatorDelegation>>, TerraRustAPIError> {
        self.terra
            .send_cmd::<LCDResult<Vec<ValidatorDelegation>>>(
                &format!("/staking/validators/{}/delegations", key),
                None,
            )
            .await
    }
    /// all delegations for a given validator (limit) (new format)
    pub async fn validator_delegations_limit(
        &self,
        key: &str,
        limit: u64,
    ) -> Result<ValidatorDelegationsV1Response, TerraRustAPIError> {
        self.terra
            .send_cmd::<ValidatorDelegationsV1Response>(
                &format!(
                    "/cosmos/staking/v1beta1/validators/{}/delegations?pagination.limit={}",
                    key, limit
                ),
                None,
            )
            .await
    }

    /// all unbondings for a given validator
    pub async fn validator_unbonding_delegations(
        &self,
        key: &str,
    ) -> Result<LCDResult<Vec<ValidatorUnbondingDelegation>>, TerraRustAPIError> {
        self.terra
            .send_cmd::<LCDResult<Vec<ValidatorUnbondingDelegation>>>(
                &format!("/staking/validators/{}/unbonding_delegations", key),
                None,
            )
            .await
    }
}
