use crate::errors::{ErrorKind, Result};
use log::debug;
use reqwest::Client;
use serde::Deserialize;

pub mod client_types;
pub mod core_types;
mod market;
pub mod market_types;
mod staking;
pub mod staking_types;

pub struct Terra<'a> {
    client: Client,
    url: &'a str,
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
    pub async fn send_cmd<T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        args: Option<&str>,
    ) -> Result<T> {
        let request_url = match args {
            Some(a) => format!("{}{}{}", self.url.to_owned(), path, a),
            None => format!("{}{}", self.url.to_owned(), path),
        };

        debug!("URL={}", request_url);
        let req = self.client.get(request_url);
        let response = req.send().await?;

        if !response.status().is_success() {
            let status_text = response.text().await?;
            Err(ErrorKind::Terra(status_text).into())
        } else {
            let struct_response: T = response.json::<T>().await?;
            Ok(struct_response)
        }
    }
}
