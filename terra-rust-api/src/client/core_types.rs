use crate::client::client_types::{terra_decimal_format, terra_u64_format};

use regex::Regex;

use rust_decimal_macros::dec;
// use rust_decimal::prelude::*;
use rust_decimal::Decimal;

use crate::errors::TerraRustAPIError;
use crate::messages::Message;
use rustc_serialize::base64::{ToBase64, STANDARD};
use serde::{Deserialize, Serialize};
use std::fmt;

/// The primary way to denote currency
/// NB: Internally everything is represented by their uXXX format.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Coin {
    #[allow(missing_docs)]
    #[serde(with = "terra_decimal_format")]
    pub amount: Decimal,
    /// the coin type. in uXXX format
    pub denom: String,
}

impl Coin {
    /// Standard Coin creation
    pub fn create(denom: &str, amount: Decimal) -> Coin {
        Coin {
            denom: denom.to_string(),
            amount,
        }
    }
    /// Parse the string "nnnnnXXXX" format where XXXX is the coin type
    pub fn parse(str: &str) -> anyhow::Result<Option<Coin>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+[.]?\d*)([a-zA-Z]+)$").unwrap();
        }
        //    let RE: Regex = Regex::new(r"^(\d+)(\s+)$").unwrap();

        match RE.captures(str) {
            Some(cap) => Ok(Some(Coin::create(
                &cap[2],
                cap.get(1).unwrap().as_str().parse::<Decimal>()?,
            ))),
            None => Ok(None),
        }
    }
    /// this will take a comma delimited string of coins and return a sorted (by denom) vector of coins
    /// eg "22.707524482460197756uaud,21.882510617180501989ucad,16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,17.639582167170638049uusd"
    ///
    pub fn parse_coins(str: &str) -> anyhow::Result<Vec<Coin>> {
        let vec_res_opt_coins = str
            .split(',')
            .map(|coin_str| Coin::parse(coin_str))
            .collect::<Vec<anyhow::Result<Option<Coin>>>>();
        let mut coins: Vec<Coin> = Vec::with_capacity(vec_res_opt_coins.len());
        for vroc in vec_res_opt_coins {
            let coin_opt = vroc.map_err(|source| TerraRustAPIError::CoinParseErrV {
                parse: str.parse().unwrap(),
                source,
            })?;

            match coin_opt {
                None => {
                    return Err(TerraRustAPIError::CoinParseErr(str.parse().unwrap()).into());
                }
                Some(coin) => {
                    coins.push(coin);
                }
            };
        }
        coins.sort_by(|a, b| a.denom.cmp(&b.denom));
        Ok(coins)
    }
}
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.amount == dec!(0.0) {
            write!(f, "0.0{}", self.denom)
        } else {
            write!(f, "{:.}{}", self.amount, self.denom)
        }
    }
}
impl PartialEq for Coin {
    fn eq(&self, other: &Self) -> bool {
        self.denom == other.denom && self.amount == other.amount
    }
}
/// Every Message sent must implement this trait
//pub trait Msg: erased_serde::Serialize {}
//serialize_trait_object!(Msg);
pub trait MsgInternal: erased_serde::Serialize {}
serialize_trait_object!(MsgInternal);

/// The fee the Transaction will pay. either in gas, or Fee (or both)
#[derive(Deserialize, Serialize, Debug)]
pub struct StdFee {
    /// The fee paid in various coins
    pub amount: Vec<Coin>,
    /// the amount of gas to use
    #[serde(with = "terra_u64_format")]
    pub gas: u64,
}
impl StdFee {
    /// single coin fee
    pub fn create_single(amount: Coin, gas: u64) -> StdFee {
        let amt: Vec<Coin> = vec![amount];
        StdFee { amount: amt, gas }
    }
    /// multi-coin fee
    pub fn create(amount: Vec<Coin>, gas: u64) -> StdFee {
        StdFee { amount, gas }
    }
}
/// The messages component of the transaction that gets signed.
/// Note: The ordering is alphabetical and *IS* important
#[derive(Serialize)]
pub struct StdSignMsg<'a> {
    #[serde(with = "terra_u64_format")]
    /// from auth::account response
    pub account_number: u64,
    /// this needs to match the chain you are sending the transaction too
    pub chain_id: String,

    /// the fee to the validator
    pub fee: StdFee,
    /// the note you want to attach to the transaction.
    pub memo: String,
    /// the messages in the transaction
    pub msgs: &'a [Message],
    /// from auth::account response
    #[serde(with = "terra_u64_format")]
    pub sequence: u64,
}

/// The structure used in sending the public key in a transaction
#[derive(Deserialize, Serialize, Debug)]
pub struct PubKeySig {
    #[allow(missing_docs)]
    #[serde(rename = "type")]
    pub stype: String,
    #[allow(missing_docs)]
    pub value: String, // base64 of the public key
}

impl PubKeySig {
    /// create from Public key structure
    pub fn create(bpub: &bitcoin::util::key::PublicKey) -> PubKeySig {
        let v = bpub.key.serialize().to_base64(STANDARD);
        PubKeySig {
            stype: "tendermint/PubKeySecp256k1".to_string(),
            value: v,
        }
    }
}
/// Messages are both signed, and have the public key of the signature generator
#[derive(Deserialize, Serialize, Debug)]
pub struct StdSignature {
    #[allow(missing_docs)]
    pub signature: String, // base 64 encoded json
    #[allow(missing_docs)]
    pub pub_key: PubKeySig,
}
impl StdSignature {
    /// the signature generated and the public key
    pub fn create(sig: &[u8; 64], bpub: &bitcoin::util::key::PublicKey) -> StdSignature {
        StdSignature {
            signature: sig.to_base64(STANDARD),
            pub_key: PubKeySig::create(bpub),
        }
    }
}

#[allow(missing_docs)]
#[derive(Serialize)]
pub struct StdTxInner<'a> {
    pub msg: &'a [Message],
    pub fee: &'a StdFee,
    pub signatures: &'a [StdSignature],
    pub memo: &'a str,
}

#[allow(missing_docs)]
#[derive(Serialize)]
pub struct StdTx<'a> {
    // #[serde(rename = "type")]
    pub tx: StdTxInner<'a>,
    pub mode: &'a str,
}
impl<'a> StdTx<'a> {
    /// create the TX which is used to POST to LCD
    pub fn create(
        msg: &'a [Message],
        fee: &'a StdFee,
        signatures: &'a [StdSignature],
        memo: &'a str,
        mode: &'a str,
    ) -> StdTx<'a> {
        StdTx {
            mode,
            tx: StdTxInner {
                msg,
                fee,
                signatures,
                memo,
            },
        }
    }
    /// Sign a standard message
    /// mode can be async, sync, or block (for debugging)
    #[allow(non_snake_case)]
    pub fn from_StdSignMsg(
        std_sign_msg: &'a StdSignMsg,
        signatures: &'a [StdSignature],
        mode: &'a str,
    ) -> StdTx<'a> {
        StdTx {
            mode,
            tx: StdTxInner {
                msg: &std_sign_msg.msgs,
                fee: &std_sign_msg.fee,
                signatures,
                memo: &std_sign_msg.memo,
            },
        }
    }
}

#[cfg(test)]
mod tst {
    use super::*;
    #[test]
    pub fn test_coin() -> anyhow::Result<()> {
        let c = Coin::create("uluna", dec!(1000.0));
        assert_eq!(c.amount, dec!(1000.0));
        assert_eq!(c.denom, "uluna");
        let d = Coin::parse("1000uluna")?;
        match d {
            Some(c) => {
                assert_eq!(c.amount, dec!(1000.0));
                assert_eq!(c.denom, "uluna");
            }
            None => {
                assert!(false)
            }
        }

        let e = Coin::parse("1000")?;
        assert!(e.is_none());
        let f = Coin::parse("")?;
        assert!(f.is_none());
        Ok(())
    }
    #[test]
    pub fn test_rate() -> anyhow::Result<()> {
        let d = Coin::parse("50292.255931832196576203umnt")?;
        match d {
            Some(c) => {
                assert_eq!(c.denom, "umnt");
                assert_eq!(c.amount, dec!(50292.255931832196576203));
                assert_eq!(c.to_string(), "50292.255931832196576203umnt");
            }
            None => assert!(false),
        }
        let e = Coin::parse("0umnt")?;
        match e {
            Some(c) => {
                assert_eq!(c.denom, "umnt");
                assert_eq!(c.amount, dec!(0.0));
                assert_eq!(c.to_string(), "0.0umnt");
            }
            None => {
                eprintln!("Regex not working for whole numbers?");
                assert!(false)
            }
        }
        Ok(())
    }
    #[test]
    fn test_coins() -> anyhow::Result<()> {
        let exchange_rates3="22.707524482460197756uaud,21.882510617180501989ucad,16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,17.639582167170638049uusd";
        let vec = Coin::parse_coins(exchange_rates3)?;
        assert_eq!(vec.len(), 15);
        let c = vec.get(2).unwrap();
        assert_eq!(c.denom, "uchf");
        assert_eq!(c.amount, dec!(16.107413560222631626));
        let exchange_rates_unsorted="16.107413560222631626uchf,114.382279464849248732ucny,14.594888140543189388ueur,12.689498975492463452ugbp,136.932658449160933002uhkd,1315.661396873891976912uinr,1917.803659404458501345ujpy,20710.846165266109229516ukrw,50292.255931832196576203umnt,12.276992042852615569usdr,23.395036036859944228usgd,0.0uthb,21.882510617180501989ucad,17.639582167170638049uusd,22.707524482460197756uaud";
        let vec2 = Coin::parse_coins(exchange_rates_unsorted)?;
        assert_eq!(vec2.len(), 15);
        let c = vec2.get(2).unwrap();
        assert_eq!(c.denom, "uchf");
        assert_eq!(c.amount, dec!(16.107413560222631626));
        for i in 0..vec2.len() {
            let c_v1 = vec.get(i).unwrap();
            let c_v2 = vec2.get(i).unwrap();

            assert_eq!(c_v1, c_v2);
        }

        Ok(())
    }
}
