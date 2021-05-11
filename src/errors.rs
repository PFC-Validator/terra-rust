use error_chain::error_chain;
#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
error_chain! {
    foreign_links {
       TerraApi ( terra_rust_api::errors::Error);
       TerraWallet ( terra_rust_wallet::errors::Error);

       Secp256k1(bitcoin::secp256k1::Error);
       SerdeJson(serde_json::Error);
       ParseInt(std::num::ParseIntError);
    }
    errors {

    }
}
