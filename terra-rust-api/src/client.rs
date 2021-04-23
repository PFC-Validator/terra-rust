use crate::errors::{ErrorKind, Result};

use reqwest::header::{HeaderMap, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};

pub mod auth;
pub mod auth_types;
pub mod client_types;
pub mod core_types;
mod market;
pub mod market_types;
mod staking;
pub mod staking_types;
pub mod tx;
pub mod tx_types;

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
    pub fn auth(&self) -> auth::Auth {
        auth::Auth::create(&self)
    }
    pub fn staking(&self) -> staking::Staking {
        staking::Staking::create(&self)
    }
    pub fn market(&self) -> market::Market {
        market::Market::create(&self)
    }
    pub fn tx(&self) -> tx::TX {
        tx::TX::create(&self)
    }
    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(USER_AGENT, "PFC-TerraRust".parse().unwrap());
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
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

        log::debug!("URL={}", &request_url);

        let req = self
            .client
            .get(&request_url)
            .headers(Terra::construct_headers());

        Terra::resp::<T>(&request_url, req).await
    }
    pub async fn post_cmd<R: for<'de> Serialize, T: for<'de> Deserialize<'de>>(
        &self,
        path: &str,
        args: &R,
    ) -> Result<T> {
        let request_url = format!("{}{}", self.url.to_owned(), path);

        log::debug!("URL={}", &request_url);
        //log::debug!("JSON={}", &args);

        let req = self
            .client
            .post(&request_url)
            .headers(Terra::construct_headers())
            .json::<R>(args);
        //      .body(args.to_string());
        //  let response = ;
        //  let response = req.send().await?;
        Terra::resp::<T>(&request_url, req).await
    }
    async fn resp<T: for<'de> Deserialize<'de>>(
        request_url: &str,
        req: RequestBuilder,
    ) -> Result<T> {
        let response = req.send().await?;
        if !response.status().is_success() {
            let status_text = response.text().await?;
            //  eprintln!("{}", &request_url);
            log::error!("URL={} - {}", &request_url, &status_text);
            Err(ErrorKind::Terra(status_text).into())
        } else {
            let struct_response: T = response.json::<T>().await?;
            Ok(struct_response)
        }
    }
}
