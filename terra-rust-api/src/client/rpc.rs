use crate::client::rpc_types::{RPCNetInfo, RPCResult, RPCStatus, RPCUnconfirmedTXS};
use crate::tendermint_types::{BlockResult, BlockResultsResult};
use crate::Terra;
pub struct RPC<'a> {
    terra: &'a Terra,
    rpc_url: &'a str,
}
impl RPC<'_> {
    pub fn create<'a>(terra: &'a Terra, tendermint_url: &'a str) -> RPC<'a> {
        RPC {
            terra,
            rpc_url: tendermint_url,
        }
    }
    pub async fn status(&self) -> anyhow::Result<RPCStatus> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<RPCStatus>>(self.rpc_url, "/status", None)
            .await?
            .result)
    }
    pub async fn net_info(&self) -> anyhow::Result<RPCNetInfo> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<RPCNetInfo>>(self.rpc_url, "/net_info", None)
            .await?
            .result)
    }
    pub async fn unconfirmed_txs(&self) -> anyhow::Result<RPCUnconfirmedTXS> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<RPCUnconfirmedTXS>>(self.rpc_url, "/unconfirmed_txs", None)
            .await?
            .result)
    }
    pub async fn block(&self) -> anyhow::Result<BlockResult> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<BlockResult>>(self.rpc_url, "/block", None)
            .await?
            .result)
    }
    pub async fn block_at_height(&self, height: u64) -> anyhow::Result<BlockResult> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<BlockResult>>(
                self.rpc_url,
                &format!("/block?height={}", height),
                None,
            )
            .await?
            .result)
    }
    pub async fn block_results(&self) -> anyhow::Result<BlockResultsResult> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<BlockResultsResult>>(self.rpc_url, "/block_results", None)
            .await?
            .result)
    }
    pub async fn block_results_at_height(&self, height: u64) -> anyhow::Result<BlockResultsResult> {
        Ok(self
            .terra
            .send_cmd_url::<RPCResult<BlockResultsResult>>(
                self.rpc_url,
                &format!("/block_results?height={}", height),
                None,
            )
            .await?
            .result)
    }
}
