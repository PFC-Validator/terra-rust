//use crate::client::client_types::terra_u64_format;
use crate::core_types::{Coin, MsgInternal};

use crate::messages::Message;
use serde::Serialize;

#[derive(Serialize, Debug)]
/// Message: Exec Contract

pub struct MsgExecuteContract {
    pub coins: Vec<Coin>,
    pub contract: String,
    pub execute_msg: String,
    pub sender: String,
}

impl MsgInternal for MsgExecuteContract {}
impl MsgExecuteContract {
    /// use provided base64 exec message
    pub fn create_from_b64(
        sender: &str,
        contract: &str,
        execute_msg: &str,
        coins: &[Coin],
    ) -> Message {
        let internal = MsgExecuteContract {
            sender: sender.into(),
            contract: contract.into(),
            execute_msg: execute_msg.into(),
            coins: coins.to_vec(),
        };
        Message {
            s_type: "wasm/MsgExecuteContract".into(),
            value: Box::new(internal),
        }
    }
    /// use provided base64 exec message
    pub fn create_from_json(
        sender: &str,
        contract: &str,
        execute_msg_json: &str,
        coins: &[Coin],
    ) -> Message {
        let exec_b64 = base64::encode(execute_msg_json);
        MsgExecuteContract::create_from_b64(sender, contract, &exec_b64, coins)
    }
}

#[cfg(test)]
mod tst {
    use super::*;
    use crate::errors::Result;
    #[test]
    pub fn test_b64() -> Result<()> {
        let vote_1 = MsgExecuteContract::create_from_b64(
            "terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew",
            "terra1f32xyep306hhcxxxf7mlyh0ucggc00rm2s9da5",
            "eyJjYXN0X3ZvdGUiOnsicG9sbF9pZCI6NDQsInZvdGUiOiJ5ZXMiLCJhbW91bnQiOiIxMDAwMDAwMCJ9fQ==",
            &vec![],
        );
        let vote_2 = MsgExecuteContract::create_from_json(
            "terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew",
            "terra1f32xyep306hhcxxxf7mlyh0ucggc00rm2s9da5",
            r#"{"cast_vote":{"poll_id":44,"vote":"yes","amount":"10000000"}}"#,
            &vec![],
        );

        let js_1 = serde_json::to_string(&vote_1)?;
        let js_2 = serde_json::to_string(&vote_2)?;

        assert_eq!(js_1, js_2);
        let js_real = r#"{"type":"wasm/MsgExecuteContract","value":{"coins":[],"contract":"terra1f32xyep306hhcxxxf7mlyh0ucggc00rm2s9da5","execute_msg":"eyJjYXN0X3ZvdGUiOnsicG9sbF9pZCI6NDQsInZvdGUiOiJ5ZXMiLCJhbW91bnQiOiIxMDAwMDAwMCJ9fQ==","sender":"terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew"}}"#;
        assert_eq!(js_1, js_real);
        Ok(())
    }
}
