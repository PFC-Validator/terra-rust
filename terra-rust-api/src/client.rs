use crate::errors::Result;
use reqwest::Client;

pub mod client_types;
pub mod core_types;
mod market;
pub mod market_types;
mod staking;
pub mod staking_types;

pub struct Terra<'a> {
    pub client: Client,
    pub url: &'a str,
    pub chain_id: &'a str,
}
impl<'a> Terra<'_> {
    pub async fn lcd_client(url: &'a str, chain_id: &'a str) -> Result<Terra<'a>> {
        let client = reqwest::Client::new();
        Ok(Terra {
            client,
            url,
            chain_id,
        })
    }
    pub fn staking(&self) -> staking::Staking {
        staking::Staking::create(&self)
    }
    pub fn market(&self) -> market::Market {
        market::Market::create(&self)
    }
}
