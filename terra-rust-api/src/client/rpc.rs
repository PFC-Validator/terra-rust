use crate::client::rpc_types::{RPCNetInfo, RPCResult, RPCStatus, RPCUnconfirmedTXS};
use crate::Terra;
pub struct RPC<'a> {
    terra: &'a Terra<'a>,
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
}
