use crate::client::wasm_types::{
    WasmCodeResult, WasmContractInfoResult, WasmParameterResult, WasmQueryRawResult,
};
use crate::errors::Result;
use crate::Terra;

use serde::Deserialize;

pub struct Wasm<'a> {
    terra: &'a Terra<'a>,
}

impl Wasm<'_> {
    pub fn create<'a>(terra: &'a Terra) -> Wasm<'a> {
        Wasm { terra }
    }
    pub async fn codes(&self, code_id: u64) -> Result<WasmCodeResult> {
        let code = self
            .terra
            .send_cmd::<WasmCodeResult>(&format!("/wasm/codes/{}", code_id), None)
            .await?;
        Ok(code)
    }
    pub async fn info(&self, contract_address: &str) -> Result<WasmContractInfoResult> {
        let code = self
            .terra
            .send_cmd::<WasmContractInfoResult>(
                &format!("/wasm/contracts/{}", contract_address),
                None,
            )
            .await?;
        Ok(code)
    }
    pub async fn parameters(&self) -> Result<WasmParameterResult> {
        let code = self
            .terra
            .send_cmd::<WasmParameterResult>("/wasm/parameters", None)
            .await?;
        Ok(code)
    }
    pub async fn query<T: for<'de> Deserialize<'de>>(
        &self,
        contract_address: &str,
        json_query: &str,
    ) -> Result<T> {
        let code = self
            .terra
            .send_cmd::<T>(
                &format!("/wasm/contracts/{}/store?", contract_address),
                Some(&format!("query_msg={}", json_query)),
            )
            .await?;
        Ok(code)
    }
    pub async fn query_raw(
        &self,
        contract_address: &str,
        key: &str,
        sub_key: &Option<String>,
    ) -> Result<(String, String)> {
        let json_query = match sub_key {
            Some(sub_key_str) => format!("key={}&subkey={}", key, &sub_key_str),
            None => format!("key={}", key),
        };

        let code = self
            .terra
            .send_cmd::<WasmQueryRawResult>(
                &format!("/wasm/contracts/{}/store/raw?", contract_address),
                Some(&json_query),
            )
            .await?;
        let key_vec = subtle_encoding::base64::decode(code.result.key.as_bytes())?;
        let key = String::from_utf8(key_vec)?;
        eprintln!("{}", code.result.key);
        let value_vec = subtle_encoding::base64::decode(code.result.value)?;
        let value = String::from_utf8(value_vec)?;

        Ok((key, value))
    }
}
