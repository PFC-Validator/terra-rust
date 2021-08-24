use crate::auth_types::AuthAccount;
use crate::staking_types::{Validator, ValidatorDelegation, ValidatorUnbondingDelegation};
use crate::{LCDResult, LCDTypeValue, Terra};

pub struct Auth<'a> {
    terra: &'a Terra<'a>,
}
impl Auth<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Auth<'a> {
        Auth { terra }
    }
    pub async fn account(
        &self,
        account_address: &str,
    ) -> anyhow::Result<LCDResult<LCDTypeValue<AuthAccount>>> {
        let response = self
            .terra
            .send_cmd::<LCDResult<LCDTypeValue<AuthAccount>>>(
                &format!("/auth/accounts/{}", account_address),
                None,
            )
            .await?;
        Ok(response)
    }
    /// all delegations for a given account
    pub async fn validator_delegations(
        &self,
        account_address: &str,
    ) -> anyhow::Result<LCDResult<ValidatorDelegation>> {
        self.terra
            .send_cmd::<LCDResult<ValidatorDelegation>>(
                &format!("/staking/delegators/{}/delegations", account_address),
                None,
            )
            .await
    }
    /// all unbonding delegations for a given account
    pub async fn validator_unbonding_delegations(
        &self,
        account_address: &str,
    ) -> anyhow::Result<LCDResult<ValidatorUnbondingDelegation>> {
        self.terra
            .send_cmd::<LCDResult<ValidatorUnbondingDelegation>>(
                &format!(
                    "/staking/delegators/{}/unbonding_delegations",
                    account_address
                ),
                None,
            )
            .await
    }
    /// all validators for a given account
    pub async fn delegated_validators(
        &self,
        account_address: &str,
    ) -> anyhow::Result<LCDResult<Vec<Validator>>> {
        self.terra
            .send_cmd::<LCDResult<Vec<Validator>>>(
                &format!("/staking/delegators/{}/validators", account_address),
                None,
            )
            .await
    }
}
