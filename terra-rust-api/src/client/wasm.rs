use crate::client::wasm_types::{
    WasmCodeResult, WasmContractInfoResult, WasmParameterResult, WasmQueryRawResult,
};
use crate::{Message, PrivateKey, Terra};
use secp256k1::{Secp256k1, Signing};
use std::path::Path;

use crate::client::tx_types::TXResultSync;
use crate::core_types::Coin;
use crate::errors::TerraRustAPIError;
use crate::messages::wasm::{MsgInstantiateContract, MsgMigrateContract, MsgStoreCode};
use serde::Deserialize;

pub struct Wasm<'a> {
    terra: &'a Terra,
}

impl Wasm<'_> {
    pub fn create(terra: &'_ Terra) -> Wasm<'_> {
        Wasm { terra }
    }
    pub async fn codes(&self, code_id: u64) -> Result<WasmCodeResult, TerraRustAPIError> {
        let code = self
            .terra
            .send_cmd::<WasmCodeResult>(&format!("/wasm/codes/{}", code_id), None)
            .await?;
        Ok(code)
    }
    pub async fn info(
        &self,
        contract_address: &str,
    ) -> Result<WasmContractInfoResult, TerraRustAPIError> {
        let code = self
            .terra
            .send_cmd::<WasmContractInfoResult>(
                &format!("/wasm/contracts/{}", contract_address),
                None,
            )
            .await?;
        Ok(code)
    }
    pub async fn parameters(&self) -> Result<WasmParameterResult, TerraRustAPIError> {
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
    ) -> Result<T, TerraRustAPIError> {
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
    ) -> Result<(String, String), TerraRustAPIError> {
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
    /// store a wasm file onto the chain.
    pub async fn store<C: Signing + Signing>(
        &self,
        secp: &Secp256k1<C>,
        from: &PrivateKey,
        wasm: &str,
        memo: Option<String>,
    ) -> Result<TXResultSync, TerraRustAPIError> {
        let from_public_key = from.public_key(secp);

        let wasm_path = Path::new(wasm);

        let store_message = MsgStoreCode::create_from_file(&from_public_key.account()?, wasm_path)?;
        let messages: Vec<Message> = vec![store_message];

        let resp = self
            .terra
            .submit_transaction_sync(secp, from, messages, memo)
            .await;
        resp
    }
    /// create a contract using code_id, json init args, and optionally admin on the chain
    #[allow(clippy::too_many_arguments)]
    pub async fn instantiate<C: Signing + Signing>(
        &self,
        secp: &Secp256k1<C>,
        from: &PrivateKey,
        code_id: u64,
        json: String,
        coins: Vec<Coin>,
        admin: Option<String>,
        memo: Option<String>,
    ) -> Result<TXResultSync, TerraRustAPIError> {
        let from_public_key = from.public_key(secp);
        let init_message = MsgInstantiateContract::create_from_json(
            &from_public_key.account()?,
            admin,
            code_id,
            &json,
            coins,
        )?;
        let messages: Vec<Message> = vec![init_message];

        let resp = self
            .terra
            .submit_transaction_sync(secp, from, messages, memo)
            .await;

        resp
    }

    /// migrate an existing contract to new_code_id, optionally with a migrate args
    pub async fn migrate<C: Signing + Signing>(
        &self,
        secp: &Secp256k1<C>,
        from: &PrivateKey,
        contract: &str,
        new_code_id: u64,
        migrate: Option<String>,
        memo: Option<String>,
    ) -> Result<TXResultSync, TerraRustAPIError> {
        let from_public_key = from.public_key(secp);

        let migrate_message = if let Some(migrate_string) = migrate {
            MsgMigrateContract::create_from_json(
                &from_public_key.account()?,
                contract,
                new_code_id,
                &migrate_string,
            )?
        } else {
            MsgMigrateContract::create_from_json(
                &from_public_key.account()?,
                contract,
                new_code_id,
                "{}",
            )?
        };

        let messages: Vec<Message> = vec![migrate_message];

        let resp = self
            .terra
            .submit_transaction_sync(secp, from, messages, memo)
            .await;
        resp
    }
}
