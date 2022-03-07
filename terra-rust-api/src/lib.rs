// `error_chain!` can recurse deeply
// #![recursion_limit = "1024"]
#![allow(missing_docs)]
/*!
* This crate provides an interface into the Terra LCD HTTP service.
* # PFC
*
* This work is sponsored by the PFC (Pretty Freaking Cool) Validator,
* feel free to delegate to the [PFC](https://station.terra.money/validator/terravaloper12g4nkvsjjnl0t7fvq3hdcw7y8dc9fq69nyeu9q) validator.
*
* It will help defray the costs.
*
* # Warning
* This uses crytpographic routines that have not gone through any security audit.
*
* The manner which it stores private keys may be unsecure/subject to hacks, and if you use it, may put the assets behind those keys at risk.
*
* This is ALPHA software.
*
* # Usage
* ```toml
* [dependencies]
* terra-rust-api="0.1"
* tokio = { version = "1.4", features = ["full"] }
* ```
* ```
* use terra_rust_api::{Terra, GasOptions, PrivateKey};
* use terra_rust_api::core_types::{Coin, StdSignMsg, StdSignature};
* use terra_rust_api::messages::{MsgSend, Message};
* use bitcoin::secp256k1::Secp256k1;
*
*
* async fn demo() -> anyhow::Result<()> {
* // set up the LCD client
* let gas_opts = GasOptions::create_with_gas_estimate("50ukrw",1.4)?;
* let terra = Terra::lcd_client("https://bombay-lcd.terra.dev/", "bombay-12", &gas_opts,None);
* // generate a private key
* let secp = Secp256k1::new();
* let from_key = PrivateKey::from_words(&secp,"your secret words",0,0)?;
* let from_public_key = from_key.public_key(&secp);
* // generate the message SEND 1000 uluna from your private key to someone else
* let coin: Coin = Coin::parse("1000uluna")?.unwrap();
* let from_account = from_public_key.account()?;
* let send: Message = MsgSend::create(from_account, String::from("terra1usws7c2c6cs7nuc8vma9qzaky5pkgvm2uag6rh"), vec![coin])?;
* // generate the transaction & calc fees
* let messages: Vec<Message> = vec![send];
* let (std_sign_msg, sigs) = terra
*                .generate_transaction_to_broadcast(
*                    &secp,
*                    &from_key,
*                    messages,
*                    None
*                )
*                .await?;
* // send it out
*  let resp = terra.tx().broadcast_sync(&std_sign_msg, &sigs).await?;
*  match resp.code {
*      Some(code) => {
*          log::error!("{}", serde_json::to_string(&resp)?);
*          eprintln!("Transaction returned a {} {}", code, resp.txhash)
*      }
*      None => {
*          println!("{}", resp.txhash)
*      }
*  }
* Ok(())
* }
* ```
*/
/// address book definition
pub mod addressbook;
/// APIs
pub mod client;
/// Error Messages
pub mod errors;
mod keys;
/// definitions of the different type of Messages we have implemented
pub mod messages;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate erased_serde;
//extern crate rustc_serialize;
//
//#[macro_use]
//extern crate error_chain;
extern crate reqwest;
//use crate::Error;

pub use crate::client_types::{
    terra_datetime_format, terra_decimal_format, terra_f64_format, terra_opt_decimal_format,
    terra_opt_u64_format, terra_u64_format,
};
pub use addressbook::AddressBook;
pub use client::lcd_types::{LCDResult, LCDResultVec, LCDTypeValue};
pub use client::{auth_types, client_types, core_types, staking_types, tendermint_types};
pub use client::{GasOptions, Terra};
pub use keys::{PrivateKey, PublicKey, Signature};
pub use messages::bank;
pub use messages::wasm::MsgExecuteContract;
pub use messages::Message;
