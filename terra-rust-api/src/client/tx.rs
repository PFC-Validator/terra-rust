use crate::client::core_types::Msg;
use crate::client::tx_types::{
    TXEstimate, TXEstimate2, TXFeeResult, TXResultAsync, TXResultBlock, TXResultSync,
};
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
    pub async fn broadcast_async(
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
        log::info!("{}", serde_json::to_string(&std_tx)?);
        let response = self
            .terra
            .post_cmd::<StdTx, TXResultSync>("/txs", &std_tx)
            .await?;
        Ok(response)
    }
    pub async fn broadcast_block(
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
    pub async fn get(&self, hash: &str) -> Result<TXResultBlock> {
        let resp = self
            .terra
            .send_cmd::<TXResultBlock>(&format!("/txs/{}", hash), None)
            .await?;
        Ok(resp)
    }
    pub async fn estimate_fee(&self, msgs: Vec<Box<dyn Msg>>) -> Result<TXFeeResult> {
        let tx_est: TXEstimate = TXEstimate {
            tx: TXEstimate2 { msgs },
        };
        let resp = self
            .terra
            .post_cmd::<TXEstimate, TXFeeResult>("/txs/estimate", &tx_est)
            .await?;
        Ok(resp)
    }
}
