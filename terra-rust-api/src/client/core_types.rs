use crate::client::client_types::terra_u64_format;
use rustc_serialize::base64::{ToBase64, STANDARD};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Coin {
    pub denom: String,
    #[serde(with = "terra_u64_format")]
    pub amount: u64,
}

impl Coin {
    pub fn create(denom: &str, amount: u64) -> Coin {
        Coin {
            denom: denom.to_string(),
            amount,
        }
    }
    pub fn to_string(&self) -> String {
        format!("{}{}", self.amount, self.denom)
    }
}

pub trait Msg: erased_serde::Serialize {}
serialize_trait_object!(Msg);

#[derive(Deserialize, Serialize, Debug)]
pub struct StdFee {
    pub amount: Vec<Coin>,
    #[serde(with = "terra_u64_format")]
    pub gas: u64,
}
impl StdFee {
    pub fn create_single(amount: Coin, gas: u64) -> StdFee {
        let amt: Vec<Coin> = vec![amount];
        StdFee { amount: amt, gas }
    }
    pub fn create(amount: Vec<Coin>, gas: u64) -> StdFee {
        StdFee { amount, gas }
    }
}
#[derive(Serialize)]
pub struct StdSignMsg {
    pub chain_id: String,
    #[serde(with = "terra_u64_format")]
    pub account_number: u64,
    #[serde(with = "terra_u64_format")]
    pub sequence: u64,
    pub fee: StdFee,
    pub msgs: Vec<Box<dyn Msg>>,
    pub memo: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PubKeySig {
    #[serde(rename = "type")]
    pub stype: String,
    pub value: String, // base64 of the public key
}

impl PubKeySig {
    pub fn create(bpub: &bitcoin::util::key::PublicKey) -> PubKeySig {
        let v = bpub.key.serialize().to_base64(STANDARD);
        PubKeySig {
            stype: "tendermint/PubKeySecp256k1".to_string(),
            value: v.to_string(),
        }
    }
}
#[derive(Deserialize, Serialize, Debug)]
pub struct StdSignature {
    pub signature: String, // base 64 encoded json
    pub pub_key: PubKeySig,
}
impl StdSignature {
    pub fn create(sig: &[u8; 64], bpub: &bitcoin::util::key::PublicKey) -> StdSignature {
        StdSignature {
            signature: sig.to_base64(STANDARD),
            pub_key: PubKeySig::create(bpub),
        }
    }
}
