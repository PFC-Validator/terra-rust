use crate::client::auth_types::AuthAccountResult;
//use crate::errors::Result;
use crate::auth_types::AccountDelegatedValidatorResult;
use crate::client::staking_types::{ValidatorDelegationResult, ValidatorUnbondingDelegationResult};
use crate::Terra;

pub struct Auth<'a> {
    terra: &'a Terra<'a>,
}
impl Auth<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Auth<'a> {
        Auth { terra }
    }
    pub async fn account(&self, account_address: &str) -> anyhow::Result<AuthAccountResult> {
        let response = self
            .terra
            .send_cmd::<AuthAccountResult>(&format!("/auth/accounts/{}", account_address), None)
            .await?;
        Ok(response)
    }
    /// all delegations for a given account
    pub async fn validator_delegations(
        &self,
        account_address: &str,
    ) -> anyhow::Result<ValidatorDelegationResult> {
        self.terra
            .send_cmd::<ValidatorDelegationResult>(
                &format!("/staking/delegators/{}/delegations", account_address),
                None,
            )
            .await
    }
    /// all unbonding delegations for a given account
    pub async fn validator_unbonding_delegations(
        &self,
        account_address: &str,
    ) -> anyhow::Result<ValidatorUnbondingDelegationResult> {
        self.terra
            .send_cmd::<ValidatorUnbondingDelegationResult>(
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
    ) -> anyhow::Result<AccountDelegatedValidatorResult> {
        self.terra
            .send_cmd::<AccountDelegatedValidatorResult>(
                &format!("/staking/delegators/{}/validators", account_address),
                None,
            )
            .await
    }
}
