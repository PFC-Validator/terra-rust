use crate::client::core_types::Msg;
use crate::client::tx_types::{
    TXEstimate, TXFeeResult, TXResultAsync, TXResultBlock, TXResultSync,
};
use crate::core_types::{Coin, StdSignMsg, StdSignature, StdTx};
use crate::errors::Result;
use crate::Terra;

pub struct TX<'a> {
    terra: &'a Terra<'a>,
}
impl TX<'_> {
    pub fn create<'a>(terra: &'a Terra) -> TX<'a> {
        TX { terra }
    }
    /// perform an Async submission to the blockchain. This returns the TXHash
    /// This is not guaranteed to successfully create a transaction record, due to numerous factors
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
    /// perform a sync submission to the blockchain. This will return more validation logic than async
    /// but you wait. It is still not guaranteed to create a blockchain transaction
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
    /// perform a 'blocking' submission to the blockchain. This will only return once the transaction
    /// is executed on the blockchain. This is great for debugging, but not recommended to be used otherwise
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
    /// get TX result
    pub async fn get(&self, hash: &str) -> Result<TXResultBlock> {
        let resp = self
            .terra
            .send_cmd::<TXResultBlock>(&format!("/txs/{}", hash), None)
            .await?;
        Ok(resp)
    }
    /// Estimate the StdFee structure based on the gas used
    pub async fn estimate_fee(
        &self,
        msgs: &Vec<Box<dyn Msg>>,
        gas_adjustment: f64,
        gas_prices: &Vec<&Coin>,
    ) -> Result<TXFeeResult> {
        let tx_est = TXEstimate::create(msgs, gas_adjustment, gas_prices);

        log::info!("#Messages = {}", serde_json::to_string(&tx_est)?);
        let resp = self
            .terra
            .post_cmd::<TXEstimate, TXFeeResult>("/txs/estimate_fee", &tx_est)
            .await?;
        Ok(resp)
    }
}
