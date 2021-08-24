use crate::terra_u64_format;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
/// ALL interactions with LCD will return a LCD result
pub struct LCDResult<T> {
    /// height of chain
    #[serde(with = "terra_u64_format")]
    pub height: u64,
    /// the LCD response
    pub result: T,
}
#[allow(missing_docs)]
#[derive(Deserialize, Serialize, Debug)]
pub struct LCDTypeValue<T> {
    #[serde(rename = "type")]
    pub stype: String,
    pub value: T,
}
