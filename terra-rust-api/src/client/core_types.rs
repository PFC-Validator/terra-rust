use crate::client::client_types::{terra_f64_format, terra_u64_format};
use crate::errors::Result;
use regex::Regex;
use rustc_serialize::base64::{ToBase64, STANDARD};
use serde::{Deserialize, Serialize};
use std::fmt;
/// The primary way to denote currency
/// NB: Internally everything is represented by their uXXX format.
#[derive(Deserialize, Serialize, Debug)]
pub struct Coin {
    #[allow(missing_docs)]
    #[serde(with = "terra_f64_format")]
    pub amount: f64,
    /// the coin type. in uXXX format
    pub denom: String,
}

impl Coin {
    /// Standard Coin creation
    pub fn create(denom: &str, amount: f64) -> Coin {
        Coin {
            denom: denom.to_string(),
            amount,
        }
    }
    /// Parse the string "nnnnnXXXX" format where XXXX is the coin type
    pub fn parse(str: &str) -> Result<Option<Coin>> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+)([a-zA-Z]+)$").unwrap();
        }
        //    let RE: Regex = Regex::new(r"^(\d+)(\s+)$").unwrap();

        match RE.captures(str) {
            Some(cap) => Ok(Some(Coin::create(
                &cap[2],
                cap.get(1).unwrap().as_str().parse::<f64>()?,
            ))),
            None => Ok(None),
        }
    }
}
impl fmt::Display for Coin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.amount, self.denom)
    }
}
/// Every Message sent must implement this trait
pub trait Msg: erased_serde::Serialize {}
serialize_trait_object!(Msg);

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
/// Note: The ordering is important
#[derive(Serialize)]
pub struct StdSignMsg {
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
    pub msgs: Vec<Box<dyn Msg>>,
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
    pub msg: &'a [Box<dyn Msg>],
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
        msg: &'a [Box<dyn Msg>],
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
    pub fn test_coin() -> Result<()> {
        let c = Coin::create("uluna", 1000.0);
        assert_eq!(c.amount, 1000.0 as f64);
        assert_eq!(c.denom, "uluna");
        let d = Coin::parse("1000uluna")?;
        match d {
            Some(c) => {
                assert_eq!(c.amount, 1000.0 as f64);
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
}
