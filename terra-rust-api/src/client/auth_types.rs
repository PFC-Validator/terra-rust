use crate::client::client_types::terra_u64_format;

use crate::core_types::{Coin, PubKeySig};
use serde::{Deserialize, Serialize};

// {"height":"3548868","result":{"type":"core/Account","value":{"address":"terra1n3g37dsdlv7ryqftlkef8mhgqj4ny7p8v78lg7","coins":[{"denom":"uluna","amount":"799995804"}],"public_key":{"type":"tendermint/PubKeySecp256k1","value":"AiMzHaA2bvnDXfHzkjMM+vkSE/p0ymBtAFKUnUtQAeXe"},"account_number":"43045","sequence":"2"}}}
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthAccount {
    pub address: String,
    pub coins: Vec<Coin>,
    pub public_key: PubKeySig,
    #[serde(with = "terra_u64_format")]
    pub account_number: u64,
    #[serde(with = "terra_u64_format")]
    pub sequence: u64,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthAccountTV {
    #[serde(rename = "type")]
    pub stype: String,
    pub value: AuthAccount,
}
#[derive(Deserialize, Serialize, Debug)]
pub struct AuthAccountResult {
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    pub result: AuthAccountTV,
}
