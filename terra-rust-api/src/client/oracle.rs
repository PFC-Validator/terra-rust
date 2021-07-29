use crate::client::oracle_types::{
    OracleParametersResult, OraclePreVotesResult, OracleVoteFeederResult, OracleVoteMissResult,
    OracleVotesResult,
};
use crate::Terra;

pub struct Oracle<'a> {
    terra: &'a Terra<'a>,
}
impl<'a> Oracle<'a> {
    pub fn create(terra: &'a Terra) -> Oracle<'a> {
        Oracle { terra }
    }
    pub async fn parameters(&self) -> anyhow::Result<OracleParametersResult> {
        let response = self
            .terra
            .send_cmd::<OracleParametersResult>("/oracle/parameters", None)
            .await?;
        Ok(response)
    }
    pub fn voters(&self, validator: &'a str) -> Voters<'a> {
        Voters::create(self.terra, validator)
    }
}
pub struct Voters<'a> {
    terra: &'a Terra<'a>,
    pub validator: &'a str,
}
impl<'a> Voters<'a> {
    pub fn create(terra: &'a Terra, validator: &'a str) -> Voters<'a> {
        Voters { terra, validator }
    }
    pub async fn votes(&self) -> anyhow::Result<OracleVotesResult> {
        let response = self
            .terra
            .send_cmd::<OracleVotesResult>(
                &format!("/oracle/voters/{}/votes", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }
    pub async fn prevotes(&self) -> anyhow::Result<OraclePreVotesResult> {
        let response = self
            .terra
            .send_cmd::<OraclePreVotesResult>(
                &format!("/oracle/voters/{}/prevotes", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }

    pub async fn feeder(&self) -> anyhow::Result<OracleVoteFeederResult> {
        let response = self
            .terra
            .send_cmd::<OracleVoteFeederResult>(
                &format!("/oracle/voters/{}/feeder", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }
    pub async fn miss(&self) -> anyhow::Result<OracleVoteMissResult> {
        let response = self
            .terra
            .send_cmd::<OracleVoteMissResult>(
                &format!("/oracle/voters/{}/miss", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }
}
