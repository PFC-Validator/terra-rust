use crate::errors::{ErrorKind, Result};

use crate::core_types::{Coin, Msg, StdFee};
use num_traits::cast::ToPrimitive;
use reqwest::header::{HeaderMap, CONTENT_TYPE, USER_AGENT};
use reqwest::{Client, RequestBuilder};
use serde::{Deserialize, Serialize};
mod auth;
/// Structures used in account authentication
pub mod auth_types;
/// JSON Serializer/Deserializer helpers
pub mod client_types;
/// Common Structures throughout the library
pub mod core_types;
mod market;
/// Structures used for Market APIs
pub mod market_types;
mod oracle;
/// Structures used for Oracle APIs
pub mod oracle_types;
mod staking;
/// Structures used for Staking APIs
pub mod staking_types;
mod tendermint;
/// Structures used for Tendermint / Misc APIs
pub mod tendermint_types;
mod tx;
/// Structures used for sending transactions to LCD
pub mod tx_types;

const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");

/// When Submitting transactions you need to either submit gas or a fee to the validator
/// This structure is used to determine what your preferences are by default
/// Higher fees may be given preference by the validator to include the transaction in their block
pub struct GasOptions {
    /// If specified the TX will use the fee specified
    pub fees: Option<Coin>,
    /// if true, the server will call the 'estimate_transaction' to get an estimate.
    /// This estimate is then multiplied by the gas_adjustment field
    pub estimate_gas: bool,
    /// your estimate of the gas to use.
    pub gas: Option<u64>,
    /// used to calculate the fee .. gas * gas_price
    pub gas_price: Option<Coin>,
    /// used to adjust the estimate
    pub gas_adjustment: Option<f64>,
}

/// The main structure that all API calls are generated from
pub struct Terra<'a> {
    /// reqwest Client
    client: Client,
    /// The URL of the LCD
    url: &'a str,
    /// The Chain of the network
    pub chain_id: &'a str,
    /// Gas Options used to help with gas/fee generation of transactions
    pub gas_options: Option<&'a GasOptions>,
}
impl<'a> Terra<'_> {
    /// Create a FULL client interface
    pub async fn lcd_client(
        url: &'a str,
        chain_id: &'a str,
        gas_options: &'a GasOptions,
    ) -> Result<Terra<'a>> {
        let client = reqwest::Client::new();
        Ok(Terra {
            client,
            url,
            chain_id,
            gas_options: Some(gas_options),
        })
    }
    /// Create a read-only / query client interface
    pub async fn lcd_client_no_tx(url: &'a str, chain_id: &'a str) -> Result<Terra<'a>> {
        let client = reqwest::Client::new();
        Ok(Terra {
            client,
            url,
            chain_id,
            gas_options: None,
        })
    }
    /// Auth API functions
    pub fn auth(&self) -> auth::Auth {
        auth::Auth::create(&self)
    }
    /// Staking API functions
    pub fn staking(&self) -> staking::Staking {
        staking::Staking::create(&self)
    }
    /// Market API functions
    pub fn market(&self) -> market::Market {
        market::Market::create(&self)
    }
    /// Oracle API functions
    pub fn oracle(&self) -> oracle::Oracle {
        oracle::Oracle::create(&self)
    }
    /// Tendermint (MISC) API Functions
    pub fn tendermint(&self) -> tendermint::Tendermint {
        tendermint::Tendermint::create(&self)
    }
    /// TXS API Functions
    pub fn tx(&self) -> tx::TX {
        tx::TX::create(&self)
    }

    fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            format!("PFC-TerraRust/{}", VERSION.unwrap_or("-?-"))
                .parse()
                .unwrap(),
        );
        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap());
        headers
    }

    /// used to send a GET command to the LCD
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
    /// used to send a POST with a JSON body to the LCD
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
    /*
    pub async fn calc_fees(&self, messages: Vec<Box<dyn Msg>>) -> Result<StdFee> {
        match self.gas_options {
            Some(gas_opts) => {
                if gas_opts.estimate_gas {
                    let est = &self.tx().estimate_fee(messages).await?;
                    let mut fees: Vec<Coin> = vec![];
                    for fee in est.result.fees {
                        fees.push(Coin::create(&fee.denom, fee.amount))
                    }

                    let gas = *&est.result.gas as f64
                        * self.gas_options.unwrap().gas_adjustment.unwrap_or(1.0);

                    Ok(StdFee::create(fees, gas.ceil().to_u64().unwrap()))
                } else {
                    match &gas_opts.fees {
                        Some(fee) => Ok(StdFee::create_single(
                            gas_opts.fees.unwrap(),
                            gas_opts.gas.unwrap_or(0),
                        )),
                        None => Ok(StdFee::create(vec![], gas_opts.gas.unwrap_or(0))),
                    }
                }
            }
            None => Err(ErrorKind::NoGasOpts.into()),
        }
    }

     */
}
