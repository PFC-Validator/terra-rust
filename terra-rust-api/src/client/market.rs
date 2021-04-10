use crate::client::core_types::Coin;
use crate::client::market_types::SwapResult;

use crate::errors::Result;
use crate::Terra;

pub struct Market<'a> {
    terra: &'a Terra<'a>,
}
impl Market<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Market<'a> {
        Market { terra }
    }
    pub async fn swap(&self, offer: &Coin, ask_denom: &str) -> Result<SwapResult> {
        let url = format!(
            "{}/market/swap?offer_coin={}&ask_denom={}",
            self.terra.url.to_owned(),
            offer.to_string(),
            ask_denom
        );

        let req = self.terra.client.get(url);
        let response = req.send().await?;
        let json = response.json::<SwapResult>().await?;

        Ok(json)
    }
}
