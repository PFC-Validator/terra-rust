use reqwest::StatusCode;
//use crate::client::core_types::Msg;
#[allow(deprecated)]
use crate::client::tx_types::TXResultBlock;
use crate::client::tx_types::{
    TXResultAsync, TXResultSync, TxEstimate, TxFeeResult, V1TXResult, V1TXSResult,
};

use crate::core_types::{Coin, StdSignMsg, StdSignature, StdTx};
use crate::errors::TerraRustAPIError;
use crate::errors::TerraRustAPIError::TXNotFound;
use crate::messages::Message;
use crate::{LCDResult, Terra};

#[allow(clippy::upper_case_acronyms)]
pub struct TX<'a> {
    terra: &'a Terra,
}
impl<'a> TX<'a> {
    pub fn create(terra: &'a Terra) -> TX<'a> {
        TX { terra }
    }
    /// perform an Async submission to the blockchain. This returns the TXHash
    /// This is not guaranteed to successfully create a transaction record, due to numerous factors
    pub async fn broadcast_async(
        &self,
        std_sign_msg: &StdSignMsg,
        sigs: &[StdSignature],
    ) -> Result<TXResultAsync, TerraRustAPIError> {
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
    #[allow(deprecated)]
    pub async fn broadcast_sync(
        &self,
        std_sign_msg: &StdSignMsg,
        sigs: &[StdSignature],
    ) -> Result<TXResultSync, TerraRustAPIError> {
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
    #[allow(deprecated)]
    pub async fn broadcast_block(
        &self,
        std_sign_msg: &StdSignMsg,
        sigs: &[StdSignature],
    ) -> Result<TXResultBlock, TerraRustAPIError> {
        log::warn!("Broadcast_block is not recommended to be used in production situations");
        let std_tx: StdTx = StdTx::from_StdSignMsg(std_sign_msg, sigs, "block");
        //    let js_sig = serde_json::to_string(&std_tx)?;
        let response = self
            .terra
            .post_cmd::<StdTx, TXResultBlock>("/txs", &std_tx)
            .await?;
        Ok(response)
    }
    #[deprecated(
        since = "1.2.12",
        note = "terra has deprecated this API endpoint. use get_v1"
    )]
    /// get TX result
    #[allow(deprecated)]
    pub async fn get(&self, hash: &str) -> Result<TXResultBlock, TerraRustAPIError> {
        let resp = self
            .terra
            .send_cmd::<TXResultBlock>(&format!("/txs/{}", hash), None)
            .await?;
        Ok(resp)
    }
    /// use v1 API
    pub async fn get_v1(&self, hash: &str) -> Result<V1TXResult, TerraRustAPIError> {
        let resp = self
            .terra
            .send_cmd::<V1TXResult>(&format!("/cosmos/tx/v1beta1/txs/{}", hash), None)
            .await?;
        Ok(resp)
    }
    #[deprecated(
        since = "1.2.12",
        note = "terra has deprecated this API endpoint. Use get_and_wait_v1"
    )]
    /// get TX result, retrying a few times.
    #[allow(deprecated)]
    pub async fn get_and_wait(
        &self,
        hash: &str,
        max_times: usize,
        sleep_amount: tokio::time::Duration,
    ) -> Result<TXResultBlock, TerraRustAPIError> {
        let mut times = 0;
        while times < max_times {
            #[allow(deprecated)]
            let tx = self.get(hash).await;

            match tx {
                Ok(tx_response) => return Ok(tx_response),
                Err(e) => {
                    times += 1;
                    match &e {
                        TerraRustAPIError::TerraLCDResponse(statuscode, out) => {
                            if statuscode == &StatusCode::NOT_FOUND {
                                log::debug!(
                                    "Transaction not applied .. retry #{} sleeping {} seconds",
                                    times,
                                    sleep_amount.as_secs()
                                );
                                tokio::time::sleep(sleep_amount).await;
                            } else {
                                log::error!("Invalid Response TX: {} {}", statuscode, out);
                                break;
                            }
                        }
                        _ => {
                            log::error!("Invalid Response TX: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
        Err(TXNotFound(hash.to_string(), max_times))
    }
    /// get TX result, retrying a few times.
    pub async fn get_and_wait_v1(
        &self,
        hash: &str,
        max_times: usize,
        sleep_amount: tokio::time::Duration,
    ) -> Result<V1TXResult, TerraRustAPIError> {
        let mut times = 0;
        while times < max_times {
            let tx = self.get_v1(hash).await;

            match tx {
                Ok(tx_response) => return Ok(tx_response),
                Err(e) => {
                    times += 1;
                    match &e {
                        TerraRustAPIError::TerraLCDResponse(statuscode, out) => {
                            if statuscode == &StatusCode::BAD_REQUEST {
                                log::debug!(
                                    "Transaction not applied .. retry #{} sleeping {} seconds",
                                    times,
                                    sleep_amount.as_secs()
                                );
                                tokio::time::sleep(sleep_amount).await;
                            } else {
                                log::error!("Invalid Response TX: {} {}", statuscode, out);
                                break;
                            }
                        }
                        _ => {
                            log::error!("Invalid Response TX: {:?}", e);
                            break;
                        }
                    }
                }
            }
        }
        Err(TXNotFound(hash.to_string(), max_times))
    }

    /// Estimate the StdFee structure based on the gas used
    pub async fn estimate_fee(
        &self,
        sender: &str,
        msgs: &[Message],
        gas_adjustment: f64,
        gas_prices: &[&Coin],
    ) -> Result<LCDResult<TxFeeResult>, TerraRustAPIError> {
        let tx_est = TxEstimate::create(
            &self.terra.chain_id,
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
    /// simulate transaction for estimating gas usage
    pub async fn simulate_v1(
        &self,
        sender: &str,
        msgs: &[Message],
        gas_adjustment: f64,
        gas_prices: &[&Coin],
    ) -> Result<LCDResult<TxFeeResult>, TerraRustAPIError> {
        let tx_est = TxEstimate::create(
            &self.terra.chain_id,
            sender,
            msgs,
            gas_adjustment,
            gas_prices,
        );

        log::debug!(
            "simulate Transaction = {:#?} #messages={}",
            tx_est.base_req,
            tx_est.msgs.len()
        );

        let resp = self
            .terra
            .post_cmd::<TxEstimate, LCDResult<TxFeeResult>>("/cosmos/tx/v1beta1/simulate", &tx_est)
            .await?;
        Ok(resp)
    }
    /// Get a list of transactions in a given block
    pub async fn get_txs_in_block(
        &self,
        height: u64,
        offset: Option<u64>,
        limit: Option<u64>,
    ) -> Result<V1TXSResult, TerraRustAPIError> {
        let resp = self
            .terra
            .send_cmd::<V1TXSResult>(
                &format!(
                    "/cosmos/tx/v1beta1/txs?events=tx.height={}&order_by=ORDER_BY_ASC&pagination.limit={}&pagination.offset={}",
                    height,
                    limit.unwrap_or(100),
                    offset.unwrap_or_default()
                ),
                None,
            )
            .await?;
        Ok(resp)
    }
}
