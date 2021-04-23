use crate::client::tx_types::{TXResultAsync, TXResultBlock, TXResultSync};
use crate::core_types::{StdSignMsg, StdSignature, StdTx};
use crate::errors::Result;
use crate::Terra;

pub struct TX<'a> {
    terra: &'a Terra<'a>,
}
impl TX<'_> {
    pub fn create<'a>(terra: &'a Terra) -> TX<'a> {
        TX { terra }
    }
    pub async fn broadcast_async<'a>(
        &self,
        std_sign_msg: &StdSignMsg,
        sigs: &Vec<StdSignature>,
    ) -> Result<TXResultAsync> {
        let std_tx: StdTx = StdTx::from_StdSignMsg(&std_sign_msg, &sigs, "async");

        //  let js_sig = serde_json::to_string(&std_tx)?;
        let response = self
            .terra
            .post_cmd::<StdTx, TXResultAsync>("/txs", &std_tx)
            .await?;
        Ok(response)
    }
    pub async fn broadcast_sync(
        &self,
        std_sign_msg: &StdSignMsg,
        sigs: &Vec<StdSignature>,
    ) -> Result<TXResultSync> {
        let std_tx: StdTx = StdTx::from_StdSignMsg(&std_sign_msg, &sigs, "sync");
        //    let js_sig = serde_json::to_string(&std_tx)?;
        let response = self
            .terra
            .post_cmd::<StdTx, TXResultSync>("/txs", &std_tx)
            .await?;
        Ok(response)
    }
    pub async fn broadcast_block<'a>(
        &self,
        std_sign_msg: &StdSignMsg,
        sigs: &Vec<StdSignature>,
    ) -> Result<TXResultBlock> {
        log::warn!("Broadcast_block is not recommended to be used in production situations");
        let std_tx: StdTx = StdTx::from_StdSignMsg(&std_sign_msg, &sigs, "block");
        //    let js_sig = serde_json::to_string(&std_tx)?;
        let response = self
            .terra
            .post_cmd::<StdTx, TXResultBlock>("/txs", &std_tx)
            .await?;
        Ok(response)
    }
}
