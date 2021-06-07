use crate::client::core_types::Coin;
use crate::client::market_types::SwapResult;

use crate::Terra;

pub struct Market<'a> {
    terra: &'a Terra<'a>,
}
impl Market<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Market<'a> {
        Market { terra }
    }
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
}
