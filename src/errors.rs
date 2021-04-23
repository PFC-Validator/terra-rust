use error_chain::error_chain;
#[cfg(test)]
impl PartialEq for Error {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}
error_chain! {
    foreign_links {
       TerraError ( terra_rust_api::errors::Error);
       KeyringError(keyring::KeyringError);
       Secp256k1(bitcoin::secp256k1::Error);
       SerdeJson(serde_json::Error);
    }
    errors {

    }
}
