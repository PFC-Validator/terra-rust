use crate::core_types::{Coin, MsgInternal};
use crate::terra_u64_format;
use std::path::Path;

use crate::messages::Message;
use serde::Serialize;

#[derive(Serialize, Debug)]
/// Message: Exec Contract
pub struct MsgExecuteContract {
    pub coins: Vec<Coin>,
    pub contract: String,
    pub execute_msg: serde_json::Value,
    pub sender: String,
}

impl MsgInternal for MsgExecuteContract {}
impl MsgExecuteContract {
    /// use provided base64 exec message
    pub fn create_from_value(
        sender: &str,
        contract: &str,
        execute_msg: &serde_json::Value,
        coins: &[Coin],
    ) -> Message {
        let internal = MsgExecuteContract {
            sender: sender.into(),
            contract: contract.into(),
            execute_msg: execute_msg.clone(),
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
    ) -> anyhow::Result<Message> {
        let exec_b64: serde_json::Value = serde_json::from_str(execute_msg_json)?;
        Ok(MsgExecuteContract::create_from_value(
            sender, contract, &exec_b64, coins,
        ))
    }
}

#[derive(Serialize, Debug)]
/// Message: Exec Contract
pub struct MsgStoreCode {
    pub sender: String,
    pub wasm_byte_code: String,
}

impl MsgInternal for MsgStoreCode {}
impl MsgStoreCode {
    /// use provided base64 exec message
    pub fn create_from_b64(sender: &str, wasm_byte_code: &str) -> Message {
        let internal = MsgStoreCode {
            sender: sender.into(),
            wasm_byte_code: wasm_byte_code.into(),
        };
        Message {
            s_type: "wasm/MsgStoreCode".into(),
            value: Box::new(internal),
        }
    }
    /// use provided base64 exec message
    pub fn create_from_file(sender: &str, file_name: &Path) -> anyhow::Result<Message> {
        let file_contents = std::fs::read(file_name)?;
        let exec_b64 = base64::encode(file_contents);
        Ok(MsgStoreCode::create_from_b64(sender, &exec_b64))
    }
}

#[derive(Serialize, Debug)]
/// Message: Exec Contract
pub struct MsgInstantiateContract {
    pub admin: String,
    #[serde(with = "terra_u64_format")]
    pub code_id: u64,
    pub sender: String,
    pub init_coins: Vec<Coin>,
    pub init_msg: serde_json::Value,
}

impl MsgInternal for MsgInstantiateContract {}
impl MsgInstantiateContract {
    /*
    /// use provided base64 exec message
    pub fn create_from_b64(
        sender: &str,
        admin: Option<String>,
        code_id: usize,
        init_msg: &str,
        init_coins: Vec<Coin>,
    ) -> Message {
        let internal = MsgInstantiateContract {
            admin: admin.map(|f| f.into()),
            code_id: code_id.to_string(),
            sender: sender.into(),
            init_coins,
            init_msg: init_msg.into(),
        };
        Message {
            s_type: "wasm/MsgInstantiateContract".into(),
            value: Box::new(internal),
        }
    }

     */
    /// create from JSON
    pub fn create_from_json(
        _sender: &str,
        _admin: Option<String>,
        _code_id: u64,
        _init_msg: &str,
        _init_coins: Vec<Coin>,
    ) -> anyhow::Result<Message> {
        panic!("This message does not function");
        /*
               let contents: serde_json::Value = serde_json::from_str(init_msg)?;
               //let exec_b64 = base64::encode(contents.to_string());

               let internal = MsgInstantiateContract {
                   admin: admin.unwrap_or("".into()),
                   code_id,
                   sender: sender.into(),
                   init_coins,
                   init_msg: contents,
               };
               Ok(Message {
                   s_type: "wasm/MsgInstantiateContract".into(),
                   value: Box::new(internal),
               })

        */
    }
    /// use provided base64 exec message
    /// switches ##SENDER##, ##ADMIN##, ##CODE_ID## with respective values
    pub fn create_from_file(
        sender: &str,
        admin: Option<String>,
        code_id: u64,
        init_file: &Path,
        init_coins: Vec<Coin>,
    ) -> anyhow::Result<Message> {
        let contents = std::fs::read_to_string(init_file)?;
        let new_contents = contents
            .replace("##SENDER##", sender)
            .replace("##CODE_ID##", &format!("{}", code_id));
        let admin_contents = match admin.clone() {
            Some(admin_str) => new_contents.replace("##ADMIN##", &admin_str),
            None => new_contents.replace("##ADMIN##", ""),
        };
        Self::create_from_json(sender, admin, code_id, &admin_contents, init_coins)
    }
}

#[cfg(test)]
mod tst {
    use super::*;
    /*
    #[test]
    pub fn test_b64() -> anyhow::Result<()> {
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

     */
    #[test]
    pub fn test_file_conversion() -> anyhow::Result<()> {
        let token_file = Path::new("./resources/terraswap_pair.wasm");
        let output = Path::new("./resources/terraswap_pair.output");
        let out_json = std::fs::read_to_string(output)?;
        let msg = MsgStoreCode::create_from_file(
            "terra1vr0e7kylhu9am44v0s3gwkccmz7k3naxysrwew",
            token_file,
        )?;

        let js = serde_json::to_string(&msg)?;
        println!("{} - {}", out_json.len(), js.len());
        assert_eq!(out_json.trim(), js);
        Ok(())
    }
}
