use crate::client::tendermint_types::{BlockResult, ValidatorSetResponse};
use crate::Terra;

pub struct Tendermint<'a> {
    terra: &'a Terra<'a>,
}
impl Tendermint<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Tendermint<'a> {
        Tendermint { terra }
    }
    /// get the latest block
    pub async fn blocks(&self) -> anyhow::Result<BlockResult> {
        let response = self
            .terra
            .send_cmd::<BlockResult>("/blocks/latest", None)
            .await?;
        Ok(response)
    }
    /// get a block at a specific height
    pub async fn blocks_at_height(&self, height: u64) -> anyhow::Result<BlockResult> {
        let response = self
            .terra
            .send_cmd::<BlockResult>(&format!("/blocks/{}", height), None)
            .await?;
        Ok(response)
    }
    /// get the latest validatorset
    pub async fn validatorsets(
        &self,
        page: usize,
        limit: usize,
    ) -> anyhow::Result<ValidatorSetResponse> {
        let args = if page == 0 {
            format!("?limit={}", limit)
        } else {
            format!("?page={}&limit={}", page, limit)
        };
        let response = self
            .terra
            .send_cmd::<ValidatorSetResponse>("/validatorsets/latest", Some(&args))
            .await?;
        Ok(response)
    }
    /// get a validatorset at a specific height
    pub async fn validatorsets_at_height(
        &self,
        height: u64,
        page: usize,
        limit: usize,
    ) -> anyhow::Result<ValidatorSetResponse> {
        let args = if page == 0 {
            format!("?limit={}", limit)
        } else {
            format!("?page={}&limit={}", page, limit)
        };
        let response = self
            .terra
            .send_cmd::<ValidatorSetResponse>(&format!("/validatorsets/{}", height), Some(&args))
            .await?;
        Ok(response)
    }
}
