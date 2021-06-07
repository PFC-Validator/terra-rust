use crate::client::tendermint_types::BlockResult;
use crate::Terra;

pub struct Tendermint<'a> {
    terra: &'a Terra<'a>,
}
impl Tendermint<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Tendermint<'a> {
        Tendermint { terra }
    }
    pub async fn blocks(&self) -> anyhow::Result<BlockResult> {
        let response = self
            .terra
            .send_cmd::<BlockResult>("/blocks/latest", None)
            .await?;
        Ok(response)
    }
    pub async fn blocks_at_height(&self, height: u64) -> anyhow::Result<BlockResult> {
        let response = self
            .terra
            .send_cmd::<BlockResult>(&format!("/blocks/{}", height), None)
            .await?;
        Ok(response)
    }
}
