use reqwest::StatusCode;
//use crate::client::core_types::Msg;
use crate::client::tx_types::{
    TXResultAsync, TXResultBlock, TXResultSync, TxEstimate, TxFeeResult,
};
use crate::core_types::{Coin, StdSignMsg, StdSignature, StdTx};
use crate::errors::TerraRustAPIError;
use crate::errors::TerraRustAPIError::TXNotFound;
use crate::messages::Message;
use crate::{LCDResult, Terra};

#[allow(clippy::upper_case_acronyms)]
pub struct TX<'a> {
    terra: &'a Terra<'a>,
}
impl<'a> TX<'a> {
    pub fn create(terra: &'a Terra) -> TX<'a> {
        TX { terra }
    }
    /// perform an Async submission to the blockchain. This returns the TXHash
    /// This is not guaranteed to successfully create a transaction record, due to numerous factors
    pub async fn broadcast_async(
        &self,
        std_sign_msg: &'a StdSignMsg<'a>,
        sigs: &[StdSignature],
    ) -> anyhow::Result<TXResultAsync> {
        let std_tx: StdTx = StdTx::from_StdSignMsg(std_sign_msg, sigs, "async");

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
        std_sign_msg: &'a StdSignMsg<'a>,
        sigs: &[StdSignature],
    ) -> anyhow::Result<TXResultSync> {
        let std_tx: StdTx = StdTx::from_StdSignMsg(std_sign_msg, sigs, "sync");
        //    let js_sig = serde_json::to_string(&std_tx)?;
        log::info!("TX broadcast #messages ={}", &std_tx.tx.msg.len());
        if self.terra.debug {
            log::debug!("{}", serde_json::to_string(&std_tx)?);
        }
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
        std_sign_msg: &'a StdSignMsg<'a>,
        sigs: &[StdSignature],
    ) -> anyhow::Result<TXResultBlock> {
        log::warn!("Broadcast_block is not recommended to be used in production situations");
        let std_tx: StdTx = StdTx::from_StdSignMsg(std_sign_msg, sigs, "block");
        //    let js_sig = serde_json::to_string(&std_tx)?;
        let response = self
            .terra
            .post_cmd::<StdTx, TXResultBlock>("/txs", &std_tx)
            .await?;
        Ok(response)
    }
    /// get TX result
    pub async fn get(&self, hash: &str) -> Result<TXResultBlock, TerraRustAPIError> {
        let resp = self
            .terra
            .send_cmd::<TXResultBlock>(&format!("/txs/{}", hash), None)
            .await?;
        Ok(resp)
    }
    /// get TX result
    pub async fn get_and_wait(
        &self,
        hash: &str,
        max_times: usize,
        sleep_amount: tokio::time::Duration,
    ) -> Result<TXResultBlock, TerraRustAPIError> {
        let mut times = 0;
        while times < max_times {
            let tx = self.get(hash).await;
            match tx {
                Ok(tx_response) => return Ok(tx_response),
                Err(e) => {
                    times += 1;
                    match &e {
                        TerraRustAPIError::TerraLCDResponse(statuscode, out) => {
                            if statuscode == &StatusCode::NOT_FOUND {
                                log::info!(
                                    "Transaction not applied .. retry #{} sleeping 1s",
                                    times
                                );
                                tokio::time::sleep(sleep_amount).await;
                            } else {
                                log::error!("Invalid Response TX: {} {}", statuscode, out);
                                break;
                            }
                        }
                        _ => {
                            log::error!("Invalid Response TX: {}", e);
                            break;
                        }
                    }
                }
            }
        }
        Err(TXNotFound(max_times))
    }
    /// Estimate the StdFee structure based on the gas used
    pub async fn estimate_fee(
        &self,
        sender: &str,
        msgs: &[Message],
        gas_adjustment: f64,
        gas_prices: &[&Coin],
    ) -> anyhow::Result<LCDResult<TxFeeResult>> {
        let tx_est = TxEstimate::create(
            self.terra.chain_id,
            sender,
            msgs,
            gas_adjustment,
            gas_prices,
        );

        if self.terra.debug {
            log::info!("Estimate Transaction = {}", serde_json::to_string(&tx_est)?);
        } else {
            log::debug!(
                "Estimate Transaction = {:#?} #messages={}",
                tx_est.base_req,
                tx_est.msgs.len()
            )
        }
        let resp = self
            .terra
            .post_cmd::<TxEstimate, LCDResult<TxFeeResult>>("/txs/estimate_fee", &tx_est)
            .await?;
        Ok(resp)
    }
}
