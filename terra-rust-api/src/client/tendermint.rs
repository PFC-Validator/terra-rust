use crate::client::tendermint_types::{BlockResult, ValidatorSetResult};
use crate::errors::TerraRustAPIError::TendermintValidatorSet;
use crate::{LCDResult, Terra};

pub struct Tendermint<'a> {
    terra: &'a Terra,
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
    /// @warn the maximum limit (at time of development is 100)
    pub async fn validatorsets(
        &self,
        page: usize,
        limit: usize,
    ) -> anyhow::Result<LCDResult<ValidatorSetResult>> {
        let args = if page == 0 {
            format!("?limit={}", limit)
        } else {
            format!("?page={}&limit={}", page, limit)
        };
        let response = self
            .terra
            .send_cmd::<LCDResult<ValidatorSetResult>>("/validatorsets/latest", Some(&args))
            .await?;
        Ok(response)
    }
    /// get the latest full validatorset
    ///
    pub async fn validatorsets_full(&self) -> anyhow::Result<LCDResult<ValidatorSetResult>> {
        // the interesting thing here is that the height returned is not available for the 2nd one.. so need to fire them off at the same time.
        let part_1_f = self.validatorsets(1, 100);
        let part_2 = self.validatorsets(2, 100).await?;
        let part_1 = part_1_f.await?;
        if part_1.result.block_height != part_2.result.block_height {
            return Err(TendermintValidatorSet(
                part_1.result.block_height,
                part_2.result.block_height,
            )
            .into());
        }
        let mut combined = part_1.result.validators;
        combined.extend(part_2.result.validators);
        let vs_combined = ValidatorSetResult {
            block_height: part_1.result.block_height,
            validators: combined,
        };
        Ok(LCDResult {
            height: part_1.height,
            result: vs_combined,
        })
    }
    /// get the full validatorset at a certain height
    ///
    pub async fn validatorsets_full_at_height(
        &self,
        height: u64,
    ) -> anyhow::Result<LCDResult<ValidatorSetResult>> {
        let part_1 = self.validatorsets_at_height(height, 1, 100).await?;
        let part_2 = self.validatorsets_at_height(height, 2, 100).await?;
        if part_1.result.block_height != part_2.result.block_height {
            return Err(TendermintValidatorSet(
                part_1.result.block_height,
                part_2.result.block_height,
            )
            .into());
        }
        let mut combined = part_1.result.validators;
        combined.extend(part_2.result.validators);
        let vs_combined = ValidatorSetResult {
            block_height: part_1.result.block_height,
            validators: combined,
        };
        Ok(LCDResult {
            height: part_1.height,
            result: vs_combined,
        })
    }

    /// get a validatorset at a specific height
    /// @warn the maximum limit (at time of development is 100)
    pub async fn validatorsets_at_height(
        &self,
        height: u64,
        page: usize,
        limit: usize,
    ) -> anyhow::Result<LCDResult<ValidatorSetResult>> {
        let args = if page == 0 {
            format!("?limit={}", limit)
        } else {
            format!("?page={}&limit={}", page, limit)
        };
        let response = self
            .terra
            .send_cmd::<LCDResult<ValidatorSetResult>>(
                &format!("/validatorsets/{}", height),
                Some(&args),
            )
            .await?;
        Ok(response)
    }
}
