use crate::core_types::Msg;

use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize, Debug)]
#[allow(missing_docs)]
pub struct MsgAggregateExchangeRatePreVote2 {
    pub(crate) hash: String,
    pub(crate) feeder: String,
    pub(crate) validator: String,
}
/// used in feeder oracle
#[derive(Deserialize, Serialize, Debug)]
pub struct MsgAggregateExchangeRatePreVote {
    #[serde(rename = "type")]
    stype: String,
    value: MsgAggregateExchangeRatePreVote2,
}
impl Msg for MsgAggregateExchangeRatePreVote {}
impl MsgAggregateExchangeRatePreVote {
    /// Create a pre vote message
    pub fn create<'a>(
        hash: String,
        feeder: String,
        validator: String,
    ) -> MsgAggregateExchangeRatePreVote {
        let msg = MsgAggregateExchangeRatePreVote2 {
            hash,
            feeder,
            validator,
        };
        MsgAggregateExchangeRatePreVote {
            stype: String::from("oracle/MsgAggregateExchangeRatePrevote"),
            value: msg,
        }
    }
}
