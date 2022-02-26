use crate::core_types::{Coin, MsgInternal};
use crate::terra_u64_format;
use std::path::Path;

use crate::errors::TerraRustAPIError;
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
    ) -> Result<Message, TerraRustAPIError> {
        let internal = MsgExecuteContract {
            sender: sender.into(),
            contract: contract.into(),
            execute_msg: execute_msg.clone(),
            coins: coins.to_vec(),
        };
        Ok(Message {
            s_type: "wasm/MsgExecuteContract".into(),
            value: serde_json::to_value(internal)?,
        })
    }
    /// use provided base64 exec message
    pub fn create_from_json(
        sender: &str,
        contract: &str,
        execute_msg_json: &str,
        coins: &[Coin],
    ) -> Result<Message, TerraRustAPIError> {
        let exec_b64: serde_json::Value = serde_json::from_str(execute_msg_json)?;
        MsgExecuteContract::create_from_value(sender, contract, &exec_b64, coins)
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
    pub fn create_from_b64(
        sender: &str,
        wasm_byte_code: &str,
    ) -> Result<Message, TerraRustAPIError> {
        let internal = MsgStoreCode {
            sender: sender.into(),
            wasm_byte_code: wasm_byte_code.into(),
        };
        Ok(Message {
            s_type: "wasm/MsgStoreCode".into(),
            value: serde_json::to_value(internal)?,
        })
    }
    /// use provided base64 exec message
    pub fn create_from_file(sender: &str, file_name: &Path) -> Result<Message, TerraRustAPIError> {
        let file_contents = std::fs::read(file_name)?;
        let exec_b64 = base64::encode(file_contents);
        MsgStoreCode::create_from_b64(sender, &exec_b64)
    }
}

#[derive(Serialize, Debug)]
/// Message: Exec Contract
pub struct MsgInstantiateContract {
    pub admin: Option<String>,
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
        sender: &str,
        admin: Option<String>,
        code_id: u64,
        init_msg: &str,
        init_coins: Vec<Coin>,
    ) -> Result<Message, TerraRustAPIError> {
        // panic!("This message does not function");

        let contents: serde_json::Value = serde_json::from_str(init_msg)?;
        //let exec_b64 = base64::encode(contents.to_string());

        let internal = MsgInstantiateContract {
            admin, //: admin.unwrap_or_else(|| "".into()),
            code_id,
            sender: sender.into(),
            init_coins,
            init_msg: contents,
        };
        Ok(Message {
            s_type: "wasm/MsgInstantiateContract".into(),
            value: serde_json::to_value(internal)?,
        })
    }
    /// use provided base64 exec message
    /// switches ##SENDER##, ##ADMIN##, ##CODE_ID## with respective values
    pub fn create_from_file(
        sender: &str,
        admin: Option<String>,
        code_id: u64,
        init_file: &Path,
        init_coins: Vec<Coin>,
    ) -> Result<Message, TerraRustAPIError> {
        let contents = std::fs::read_to_string(init_file)?;
        let new_contents = Self::replace_parameters(sender, admin.clone(), code_id, &contents);
        Self::create_from_json(sender, admin, code_id, &new_contents, init_coins)
    }
    /// replace parts of the string with current values
    pub fn replace_parameters(
        sender: &str,
        admin: Option<String>,
        code_id: u64,
        instantiate_str: &str,
    ) -> String {
        let part_1 = instantiate_str
            .replace("##SENDER##", sender)
            .replace("##CODE_ID##", &format!("{}", code_id));
        match admin {
            Some(admin_str) => part_1.replace("##ADMIN##", &admin_str),
            None => part_1.replace("##ADMIN##", ""),
        }
    }
}

#[derive(Serialize, Debug)]
/// Message: Exec Contract
pub struct MsgMigrateContract {
    pub admin: String,
    pub contract: String,
    #[serde(with = "terra_u64_format")]
    pub new_code_id: u64,
    pub migrate_msg: serde_json::Value,
}

impl MsgInternal for MsgMigrateContract {}
impl MsgMigrateContract {
    /// create from JSON
    pub fn create_from_json(
        admin: &str,
        contract: &str,
        new_code_id: u64,
        migrate_msg: &str,
    ) -> Result<Message, TerraRustAPIError> {
        let contents: serde_json::Value = serde_json::from_str(migrate_msg)?;

        let internal = MsgMigrateContract {
            admin: String::from(admin),
            contract: String::from(contract),
            new_code_id,
            migrate_msg: contents,
        };
        Ok(Message {
            s_type: "wasm/MsgMigrateContract".into(),
            value: serde_json::to_value(internal)?,
        })
    }
    /// use provided base64 exec message
    /// switches ##SENDER##, ##ADMIN##, ##CODE_ID## with respective values
    pub fn create_from_file(
        admin: &str,
        contract: &str,
        new_code_id: u64,
        migrate_file: &Path,
    ) -> Result<Message, TerraRustAPIError> {
        let contents = std::fs::read_to_string(migrate_file)?;
        let new_contents = Self::replace_parameters(admin, contract, new_code_id, &contents);

        Self::create_from_json(admin, contract, new_code_id, &new_contents)
    }
    /// switches ##SENDER##, ##ADMIN##, ##CODE_ID## with respective values
    pub fn replace_parameters(
        admin: &str,
        contract: &str,
        new_code_id: u64,
        migrate_str: &str,
    ) -> String {
        migrate_str
            .replace("##ADMIN##", admin)
            .replace("##CONTRACT##", contract)
            .replace("##NEW_CODE_ID##", &format!("{}", new_code_id))
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
        log::debug!("Test file conversion: {} - {}", out_json.len(), js.len());
        assert_eq!(out_json.trim(), js);
        Ok(())
    }
}
