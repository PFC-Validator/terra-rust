use crate::client::staking_types::{ValidatorListResult, ValidatorResult};
use crate::errors::Result;
use crate::staking_types::Validator;
use crate::Terra;

pub struct Staking<'a> {
    terra: &'a Terra<'a>,
}
impl Staking<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Staking<'a> {
        Staking { terra }
    }
    pub async fn validator(&self, key: &str) -> Result<ValidatorResult> {
        //   let url = self.terra.url.to_owned() + "/staking/validators/" + key;
        self.terra
            .send_cmd::<ValidatorResult>("/staking/validators/", Some(key))
            .await
        //  let req = self.terra.client.get(url);
        //let response = req.send().await?;
        //Ok(response.json::<ValidatorResult>().await?)
    }
    pub async fn validators(&self) -> Result<ValidatorListResult> {
        self.terra
            .send_cmd::<ValidatorListResult>("/staking/validators", None)
            .await
    }
    pub async fn validator_by_moniker(&self, moniker: &str) -> Result<Option<Validator>> {
        let lst = self
            .terra
            .send_cmd::<ValidatorListResult>("/staking/validators", None)
            .await?
            .result;
        match lst.iter().find(|&p| p.description.moniker == moniker) {
            None => Ok(None),
            Some(v) => Ok(Some(v.to_owned())),
        }
    }
}
