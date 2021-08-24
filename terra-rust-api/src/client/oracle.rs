use crate::client::oracle_types::{OracleParameters, OraclePreVotes, OracleVotes};
use crate::{LCDResult, Terra};

pub struct Oracle<'a> {
    terra: &'a Terra<'a>,
}
impl<'a> Oracle<'a> {
    pub fn create(terra: &'a Terra) -> Oracle<'a> {
        Oracle { terra }
    }
    pub async fn parameters(&self) -> anyhow::Result<LCDResult<OracleParameters>> {
        let response = self
            .terra
            .send_cmd::<LCDResult<OracleParameters>>("/oracle/parameters", None)
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
    pub async fn votes(&self) -> anyhow::Result<LCDResult<Vec<OracleVotes>>> {
        let response = self
            .terra
            .send_cmd::<LCDResult<Vec<OracleVotes>>>(
                &format!("/oracle/voters/{}/votes", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }
    pub async fn prevotes(&self) -> anyhow::Result<LCDResult<Vec<OraclePreVotes>>> {
        let response = self
            .terra
            .send_cmd::<LCDResult<Vec<OraclePreVotes>>>(
                &format!("/oracle/voters/{}/prevotes", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }

    pub async fn feeder(&self) -> anyhow::Result<LCDResult<String>> {
        let response = self
            .terra
            .send_cmd::<LCDResult<String>>(
                &format!("/oracle/voters/{}/feeder", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }
    pub async fn miss(&self) -> anyhow::Result<LCDResult<String>> {
        let response = self
            .terra
            .send_cmd::<LCDResult<String>>(
                &format!("/oracle/voters/{}/miss", &self.validator),
                None,
            )
            .await?;
        Ok(response)
    }
}
