// use crate::errors::{ErrorKind, Result};
use crate::client::tx_types::TxFeeResult;
use crate::core_types::{Coin, StdFee, StdSignMsg, StdSignature};
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
/// APIs to perform oracle related things
pub mod oracle;
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
/// wasm module/contract related apis
mod wasm;
mod wasm_types;

use crate::errors::TerraRustAPIError;
use crate::messages::Message;
use crate::AddressBook;
use crate::PrivateKey;
use bitcoin::secp256k1::{All, Secp256k1};
use futures::TryFutureExt;
use rust_decimal_macros::dec;
use std::fs::File;

/// Version # of package sent out on requests to help with debugging
const VERSION: Option<&'static str> = option_env!("CARGO_PKG_VERSION");
/// name of package
const NAME: Option<&'static str> = option_env!("CARGO_PKG_NAME");

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
impl GasOptions {
    /// for hard-coding of fees
    pub fn create_with_fees(fees: &str, gas: u64) -> anyhow::Result<GasOptions> {
        Ok(GasOptions {
            fees: Coin::parse(fees)?,
            estimate_gas: false,
            gas: Some(gas),
            gas_price: None,
            gas_adjustment: None,
        })
    }
    /// for when you want the validator to give you an estimate on the amounts
    pub fn create_with_gas_estimate(
        gas_price: &str,
        gas_adjustment: f64,
    ) -> anyhow::Result<GasOptions> {
        Ok(GasOptions {
            fees: None,
            estimate_gas: true,
            gas: None,
            gas_price: Coin::parse(gas_price)?,
            gas_adjustment: Some(gas_adjustment),
        })
    }
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
    pub debug: bool,
}
impl<'a> Terra<'a> {
    const NETWORK_PROD_ADDRESS_BOOK: &'a str = "https://network.terra.dev/addrbook.json";
    const NETWORK_TEST_ADDRESS_BOOK: &'a str = "https://network.terra.dev/testnet/addrbook.json";

    /// Create a FULL client interface
    pub async fn lcd_client(
        url: &'a str,
        chain_id: &'a str,
        gas_options: &'a GasOptions,
        debug: Option<bool>,
    ) -> anyhow::Result<Terra<'a>> {
        let client = reqwest::Client::new();
        match debug {
            Some(d) => Ok(Terra {
                client,
                url,
                chain_id,
                gas_options: Some(gas_options),
                debug: d,
            }),
            None => Ok(Terra {
                client,
                url,
                chain_id,
                gas_options: Some(gas_options),
                debug: false,
            }),
        }
    }
    /// Create a read-only / query client interface
    pub async fn lcd_client_no_tx(url: &'a str, chain_id: &'a str) -> anyhow::Result<Terra<'a>> {
        let client = reqwest::Client::new();
        Ok(Terra {
            client,
            url,
            chain_id,
            gas_options: None,
            debug: false,
        })
    }
    /// Auth API functions
    pub fn auth(&self) -> auth::Auth {
        auth::Auth::create(self)
    }
    /// Staking API functions
    pub fn staking(&self) -> staking::Staking {
        staking::Staking::create(self)
    }
    /// Market API functions
    pub fn market(&self) -> market::Market {
        market::Market::create(self)
    }
    /// Oracle API functions
    pub fn oracle(&self) -> oracle::Oracle {
        oracle::Oracle::create(self)
    }
    /// Tendermint (MISC) API Functions
    pub fn tendermint(&self) -> tendermint::Tendermint {
        tendermint::Tendermint::create(self)
    }
    /// TXS API Functions
    pub fn tx(&self) -> tx::TX {
        tx::TX::create(self)
    }
    /// WASM module / smart contract API Functions
    pub fn wasm(&self) -> wasm::Wasm {
        wasm::Wasm::create(self)
    }

    pub fn construct_headers() -> HeaderMap {
        let mut headers = HeaderMap::new();

        headers.insert(
            USER_AGENT,
            format!(
                "PFC-{}/{}",
                NAME.unwrap_or("terra-rust-api"),
                VERSION.unwrap_or("-?-")
            )
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
    ) -> anyhow::Result<T> {
        let request_url = match args {
            Some(a) => format!("{}{}{}", self.url.to_owned(), path, a),
            None => format!("{}{}", self.url.to_owned(), path),
        };

        if self.debug {
            log::debug!("URL={}", &request_url);
        }
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
    ) -> anyhow::Result<T> {
        let request_url = format!("{}{}", self.url.to_owned(), path);

        if self.debug {
            log::debug!("URL={}", &request_url);
        }

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
    ) -> anyhow::Result<T> {
        let response = req.send().await?;
        if !response.status().is_success() {
            let status_text = response.text().await?;
            //  eprintln!("{}", &request_url);
            log::error!("URL={} - {}", &request_url, &status_text);
            Err(TerraRustAPIError::Terra(status_text).into())
        } else {
            let struct_response: T = response.json::<T>().await?;
            Ok(struct_response)
        }
    }
    /// Generate Fee structure, either by estimation method or hardcoded
    ///

    pub async fn calc_fees(&self, messages: &[Message]) -> anyhow::Result<StdFee> {
        if self.gas_options.is_none() {
            return Err(TerraRustAPIError::NoGasOpts.into());
        }
        let gas = self.gas_options.unwrap();
        match &gas.fees {
            Some(f) => {
                let fee_coin: Coin = Coin::create(&f.denom, f.amount);
                Ok(StdFee::create(vec![fee_coin], gas.gas.unwrap_or(0)))
            }
            None => {
                let fee: StdFee = match &gas.estimate_gas {
                    true => {
                        let default_gas_coin = Coin::create("ukrw", dec!(1.0));
                        let gas_coin = match &gas.gas_price {
                            Some(c) => c,
                            None => &default_gas_coin,
                        };
                        let res: TxFeeResult = self
                            .tx()
                            .estimate_fee(messages, gas.gas_adjustment.unwrap_or(1.0), &[gas_coin])
                            .await?;
                        //  let gas_amount = gas.gas_adjustment.unwrap_or(1.0) * res.result.gas as f64;
                        let mut fees: Vec<Coin> = vec![];
                        for fee in res.result.fees {
                            fees.push(Coin::create(&fee.denom, fee.amount))
                        }
                        StdFee::create(fees, res.result.gas as u64)
                    }
                    false => {
                        let mut fees: Vec<Coin> = vec![];
                        match &gas.fees {
                            Some(fee) => {
                                fees.push(Coin::create(&fee.denom, fee.amount));
                            }
                            None => {}
                        }

                        StdFee::create(fees, gas.gas.unwrap_or(0))
                    }
                };
                Ok(fee)
            }
        }
    }
    /// helper function to generate a 'StdSignMsg' & 'Signature' blocks to be used to broadcast a transaction
    #[allow(clippy::too_many_arguments)]
    fn generate_transaction_to_broadcast_fees(
        chain_id: String,
        account_number: u64,
        sequence: u64,
        fee: StdFee,
        secp: &Secp256k1<All>,
        from: &'a PrivateKey,
        messages: &'a [Message],
        memo: Option<String>,
    ) -> anyhow::Result<(StdSignMsg<'a>, Vec<StdSignature>)> {
        let std_sign_msg = StdSignMsg {
            chain_id, //: String::from(self.chain_id),
            account_number,
            sequence,
            fee,
            msgs: messages,
            memo: memo.unwrap_or(format!(
                "PFC-{}/{}",
                NAME.unwrap_or("TERRA-RUST"),
                VERSION.unwrap_or("dev")
            )),
        };
        let js = serde_json::to_string(&std_sign_msg)?;
        log::debug!("TO SIGN - {}", js);
        // eprintln!("Client.rs:311\n{}", js);
        let sig = from.sign(secp, &js)?;
        let sigs: Vec<StdSignature> = vec![sig];

        Ok((std_sign_msg, sigs))
    }

    /// helper function to generate a 'StdSignMsg' & 'Signature' blocks to be used to broadcast a transaction
    /// This version calculates fees, and obtains account# and sequence# as well

    pub async fn generate_transaction_to_broadcast(
        &self,
        secp: &Secp256k1<All>,
        from: &'a PrivateKey,
        messages: &'a [Message],
        memo: Option<String>,
    ) -> anyhow::Result<(StdSignMsg<'a>, Vec<StdSignature>)> {
        let from_public = from.public_key(secp);
        let from_account = from_public.account()?;

        self.auth()
            .account(&from_account)
            .map_ok(move |auth_result| {
                self.calc_fees(messages).map_ok(move |std_fee| {
                    Terra::generate_transaction_to_broadcast_fees(
                        self.chain_id.into(),
                        auth_result.result.value.account_number,
                        auth_result.result.value.sequence,
                        std_fee,
                        secp,
                        from,
                        messages,
                        memo,
                    )
                })
            })
            .await?
            .await?
    }
    /// fetch the address book for the production network
    pub async fn production_address_book() -> anyhow::Result<AddressBook> {
        Self::address_book(Self::NETWORK_PROD_ADDRESS_BOOK).await
    }
    /// fetch the address book for the testnet network
    pub async fn testnet_address_book() -> anyhow::Result<AddressBook> {
        Self::address_book(Self::NETWORK_TEST_ADDRESS_BOOK).await
    }
    /// fetch a address book json structure
    pub async fn address_book(addr_url: &str) -> anyhow::Result<AddressBook> {
        if addr_url.starts_with("file://") {
            let file = File::open(&addr_url[7..]).unwrap();
            let add: AddressBook = serde_json::from_reader(file)?;
            Ok(add)
        } else {
            let client = reqwest::Client::new();

            let req = client.get(addr_url).headers(Self::construct_headers());
            Self::resp::<AddressBook>(addr_url, req).await
        }
    }
}
#[cfg(test)]
mod tst {
    use super::*;
    use crate::core_types::{Coin, StdTx};
    use crate::messages::MsgSend;
    use crate::{MsgExecuteContract, PrivateKey, Terra};
    use bitcoin::secp256k1::Secp256k1;

    #[test]
    pub fn test_send() -> anyhow::Result<()> {
        let str_1 = "island relax shop such yellow opinion find know caught erode blue dolphin behind coach tattoo light focus snake common size analyst imitate employ walnut";
        let secp = Secp256k1::new();
        let pk = PrivateKey::from_words(&secp, str_1)?;
        let pub_k = pk.public_key(&secp);
        let from_address = pub_k.account()?;
        assert_eq!(from_address, "terra1n3g37dsdlv7ryqftlkef8mhgqj4ny7p8v78lg7");

        let send = MsgSend::create_single(
            from_address,
            "terra1usws7c2c6cs7nuc8vma9qzaky5pkgvm2uag6rh".into(),
            Coin::parse("100000uluna")?.unwrap(),
        );
        let json = serde_json::to_string(&send)?;
        let json_eq = r#"{"type":"bank/MsgSend","value":{"amount":[{"amount":"100000","denom":"uluna"}],"from_address":"terra1n3g37dsdlv7ryqftlkef8mhgqj4ny7p8v78lg7","to_address":"terra1usws7c2c6cs7nuc8vma9qzaky5pkgvm2uag6rh"}}"#;

        assert_eq!(json, json_eq);
        let std_fee = StdFee::create_single(Coin::parse("50000uluna")?.unwrap(), 90000);

        let messages: Vec<Message> = vec![send];
        let (sign_message, signatures) = Terra::generate_transaction_to_broadcast_fees(
            "tequila-0004".into(),
            43045,
            3,
            std_fee,
            &secp,
            &pk,
            &messages,
            Some("PFC-terra-rust/0.1.5".into()),
        )?;
        let json_sign_message = serde_json::to_string(&sign_message)?;
        let json_sign_message_eq = r#"{"account_number":"43045","chain_id":"tequila-0004","fee":{"amount":[{"amount":"50000","denom":"uluna"}],"gas":"90000"},"memo":"PFC-terra-rust/0.1.5","msgs":[{"type":"bank/MsgSend","value":{"amount":[{"amount":"100000","denom":"uluna"}],"from_address":"terra1n3g37dsdlv7ryqftlkef8mhgqj4ny7p8v78lg7","to_address":"terra1usws7c2c6cs7nuc8vma9qzaky5pkgvm2uag6rh"}}],"sequence":"3"}"#;
        assert_eq!(json_sign_message, json_sign_message_eq);
        let json_sig = serde_json::to_string(&signatures)?;
        let json_sig_eq = r#"[{"signature":"f1wYTzbSyAYqN2tGR0A4PGmfyNYBUExpuoU7UOiBDpNoRlChF/BMtE7h6pdgbpu/V7jNzitu1Eb0fO35dxVkWA==","pub_key":{"type":"tendermint/PubKeySecp256k1","value":"AiMzHaA2bvnDXfHzkjMM+vkSE/p0ymBtAFKUnUtQAeXe"}}]"#;
        assert_eq!(json_sig, json_sig_eq);
        let std_tx: StdTx = StdTx::from_StdSignMsg(&sign_message, &signatures, "sync");
        let js_sig = serde_json::to_string(&std_tx)?;
        let js_sig_eq = r#"{"tx":{"msg":[{"type":"bank/MsgSend","value":{"amount":[{"amount":"100000","denom":"uluna"}],"from_address":"terra1n3g37dsdlv7ryqftlkef8mhgqj4ny7p8v78lg7","to_address":"terra1usws7c2c6cs7nuc8vma9qzaky5pkgvm2uag6rh"}}],"fee":{"amount":[{"amount":"50000","denom":"uluna"}],"gas":"90000"},"signatures":[{"signature":"f1wYTzbSyAYqN2tGR0A4PGmfyNYBUExpuoU7UOiBDpNoRlChF/BMtE7h6pdgbpu/V7jNzitu1Eb0fO35dxVkWA==","pub_key":{"type":"tendermint/PubKeySecp256k1","value":"AiMzHaA2bvnDXfHzkjMM+vkSE/p0ymBtAFKUnUtQAeXe"}}],"memo":"PFC-terra-rust/0.1.5"},"mode":"sync"}"#;
        assert_eq!(js_sig, js_sig_eq);
        Ok(())
    }

    #[test]
    pub fn test_wasm() -> anyhow::Result<()> {
        let key_words = "sell raven long age tooth still predict idea quit march gasp bamboo hurdle problem voyage east tiger divide machine brain hole tiger find smooth";
        let secp = Secp256k1::new();
        let private = PrivateKey::from_words(&secp, key_words)?;
        let public_key = private.public_key(&secp);
        let account = public_key.account()?;
        assert_eq!(account, "terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew");
        //  let gas = GasOptions::create_with_fees("70000uluna", 200000)?;
        //   let terra =
        //       Terra::lcd_client("https://tequila-lcd.terra.dev", "tequila-0004", &gas, None).await?;
        let msg = MsgExecuteContract::create_from_b64(
            &account,
            "terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu",
            "eyJjYXN0X3ZvdGUiOnsicG9sbF9pZCI6NDQsInZvdGUiOiJ5ZXMiLCJhbW91bnQiOiIxMDAwMDAwIn19",
            &vec![],
        );

        let json = serde_json::to_string(&msg)?;
        let json_eq = r#"{"type":"wasm/MsgExecuteContract","value":{"coins":[],"contract":"terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu","execute_msg":"eyJjYXN0X3ZvdGUiOnsicG9sbF9pZCI6NDQsInZvdGUiOiJ5ZXMiLCJhbW91bnQiOiIxMDAwMDAwIn19","sender":"terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew"}}"#;

        assert_eq!(json, json_eq);
        let std_fee = StdFee::create_single(Coin::parse("70000uluna")?.unwrap(), 200000);

        let messages: Vec<Message> = vec![msg];
        let (sign_message, signatures) = Terra::generate_transaction_to_broadcast_fees(
            "tequila-0004".into(),
            49411,
            0,
            std_fee,
            &secp,
            &private,
            &messages,
            Some("PFC-terra-rust-anchor/0.1.1".into()),
        )?;
        let json_sign_message = serde_json::to_string(&sign_message)?;
        let json_sign_message_eq = r#"{"account_number":"49411","chain_id":"tequila-0004","fee":{"amount":[{"amount":"70000","denom":"uluna"}],"gas":"200000"},"memo":"PFC-terra-rust-anchor/0.1.1","msgs":[{"type":"wasm/MsgExecuteContract","value":{"coins":[],"contract":"terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu","execute_msg":"eyJjYXN0X3ZvdGUiOnsicG9sbF9pZCI6NDQsInZvdGUiOiJ5ZXMiLCJhbW91bnQiOiIxMDAwMDAwIn19","sender":"terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew"}}],"sequence":"0"}"#;
        assert_eq!(json_sign_message, json_sign_message_eq);
        let json_sig = serde_json::to_string(&signatures)?;
        let json_sig_eq = r#"[{"signature":"pCkd+nBaz1U3DYw0oY2Arxqc+3jI8QRdaXtYbIle9uh60POxvcUHVk2aN7VklgvnPKF7XGIF04U0sxpq/05Vqg==","pub_key":{"type":"tendermint/PubKeySecp256k1","value":"A3K4ruHQP1yY4dkCp41Djnx6z7KfMjDcvkIB93L3Po9C"}}]"#;
        assert_eq!(json_sig, json_sig_eq);
        let std_tx: StdTx = StdTx::from_StdSignMsg(&sign_message, &signatures, "sync");
        let js_sig = serde_json::to_string(&std_tx)?;
        let js_sig_eq = r#"{"tx":{"msg":[{"type":"wasm/MsgExecuteContract","value":{"coins":[],"contract":"terra16ckeuu7c6ggu52a8se005mg5c0kd2kmuun63cu","execute_msg":"eyJjYXN0X3ZvdGUiOnsicG9sbF9pZCI6NDQsInZvdGUiOiJ5ZXMiLCJhbW91bnQiOiIxMDAwMDAwIn19","sender":"terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew"}}],"fee":{"amount":[{"amount":"70000","denom":"uluna"}],"gas":"200000"},"signatures":[{"signature":"pCkd+nBaz1U3DYw0oY2Arxqc+3jI8QRdaXtYbIle9uh60POxvcUHVk2aN7VklgvnPKF7XGIF04U0sxpq/05Vqg==","pub_key":{"type":"tendermint/PubKeySecp256k1","value":"A3K4ruHQP1yY4dkCp41Djnx6z7KfMjDcvkIB93L3Po9C"}}],"memo":"PFC-terra-rust-anchor/0.1.1"},"mode":"sync"}"#;
        assert_eq!(js_sig, js_sig_eq);
        Ok(())
    }

    #[tokio::test]
    pub async fn test_address_book() -> anyhow::Result<()> {
        let prod = Terra::production_address_book().await?;
        assert!(prod.addrs.len() > 0);
        let test = Terra::testnet_address_book().await?;
        assert!(test.addrs.len() > 0);
        let file_version = Terra::address_book("file://resources/addressbook.json").await?;
        assert_eq!(file_version.key, "775cf30a073ca5e97fb07a00");
        assert!(file_version.addrs.len() > 1);
        assert_eq!(
            file_version.addrs[0].addr.id,
            "ebca6b5d3cc2da9dfdfe4b1c045043fce686f143"
        );

        Ok(())
    }
}
