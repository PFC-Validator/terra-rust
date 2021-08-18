use crate::client::core_types::Coin;
use crate::client::market_types::SwapResult;

use crate::{Message, Terra};
use rust_decimal::Decimal;

use crate::messages::market::MsgSwap;
use futures::future::join_all;

pub struct Market<'a> {
    terra: &'a Terra<'a>,
}
impl Market<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Market<'a> {
        Market { terra }
    }
    /// obtain how much a coin is worth in a secondary coin
    pub async fn swap(&self, offer: &Coin, ask_denom: &str) -> anyhow::Result<SwapResult> {
        let response = self
            .terra
            .send_cmd::<SwapResult>(
                "/market/swap",
                Some(&format!(
                    "?offer_coin={}&ask_denom={}",
                    offer.to_string(),
                    ask_denom
                )),
            )
            .await?;
        Ok(response)
    }
    /// generate a set of transactions to swap a account's tokens into another, as long as they are above a certain threshold
    pub async fn generate_sweep_messages(
        &self,
        from: String,
        to_coin: String,
        threshold: Decimal,
    ) -> anyhow::Result<Vec<Message>> {
        let account_balances = self.terra.auth().account(&from).await?;
        let potential_coins = account_balances
            .result
            .value
            .coins
            .into_iter()
            .filter(|c| c.denom != to_coin)
            .collect::<Vec<Coin>>();
        let into_currency_futures = potential_coins
            .into_iter()
            .map(|c| async {
                let resp = self
                    .terra
                    .market()
                    .swap(&c.clone(), &to_coin)
                    .await
                    .and_then(|f| Ok((c, f.result)));
                resp
            })
            .collect::<Vec<_>>();

        let into_currency = join_all(into_currency_futures).await;
        /*
        let has_err = into_currency.into_iter().find(|f| f.is_err());
        match has_err {
            Some(err) => {
                err?;
            }
            None => {}
        };

         */

        let mut err = None;
        let to_convert = &into_currency
            .into_iter()
            .flat_map(|f| match f {
                Ok(coins) => {
                    if coins.1.amount > threshold {
                        Some(coins)
                    } else {
                        None
                    }
                }
                Err(e) => {
                    eprintln!("Error  {}", e.to_string());
                    err = Some(e);
                    None
                }
            })
            .collect::<Vec<_>>();
        match err {
            Some(e) => Err(e),
            None => Ok(to_convert
                .into_iter()
                .map(|f| MsgSwap::create(f.0.clone(), to_coin.clone(), from.clone()))
                .collect::<Vec<_>>()),
        }
    }
}
