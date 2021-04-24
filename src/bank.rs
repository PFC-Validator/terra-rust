use structopt::StructOpt;
use terra_rust_api::Terra;

use crate::errors::Result;
use crate::keys::get_private_key;
use bitcoin::secp256k1::Secp256k1;
use terra_rust_api::auth::Auth;
use terra_rust_api::auth_types::AuthAccountResult;
use terra_rust_api::bank::MsgSend;
use terra_rust_api::core_types::{Coin, Msg, StdFee, StdSignMsg, StdSignature};

#[derive(StructOpt)]
pub enum BankCommand {
    #[structopt(name = "send", about = "send some coin[s] to an account")]
    Send {
        /// from account (specify the key name in the wallet
        from: String,
        /// the to account in 'terra' format
        to: String,
        /// the amount
        amount: u64,
        /// denom
        denom: String,
    },
}

pub async fn bank_cmd_parse(
    terra: &Terra<'_>,
    wallet: &str,
    seed: Option<&str>,
    bank_cmd: BankCommand,
) -> Result<()> {
    match bank_cmd {
        BankCommand::Send {
            from,
            to,
            amount,
            denom,
        } => {
            let secp = Secp256k1::new();
            let from_key = get_private_key(&secp, wallet, &from, seed)?;
            let from_public_key = from_key.public_key(&secp);
            let coin: Coin = Coin::create(&denom, amount);
            let send: MsgSend = MsgSend::create(from_public_key.account()?, to, vec![coin]);

            let messages: Vec<Box<dyn Msg>> = vec![Box::new(send)];
            let std_fee = StdFee::create_single(Coin::create("uluna", 233471), 1156472);

            let auth = Auth::create(terra);
            let auth_result: AuthAccountResult = auth.account(&from_public_key.account()?).await?;
            let account_number = auth_result.result.value.account_number;

            let std_sign_msg = StdSignMsg {
                chain_id: terra.chain_id.to_string(),
                account_number,
                sequence: auth_result.result.value.sequence,
                fee: std_fee,
                msgs: messages,
                memo: "".to_string(),
            };

            let js = serde_json::to_string(&std_sign_msg)?;
            eprintln!("{}", js);
            let sig = from_key.sign(&secp, &js)?;
            //      eprintln!("{}", sig.pub_key.value);
            //   eprintln!("{}", sig.signature);
            let sigs: Vec<StdSignature> = vec![sig];

            let resp = terra.tx().broadcast_block(&std_sign_msg, &sigs).await?;
            log::debug!("{:#?}", resp);
        }
    }
    Ok(())
}
