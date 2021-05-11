#![allow(missing_docs)]
use error_chain::error_chain;

impl From<Box<dyn std::error::Error>> for Error {
    fn from(e: Box<dyn std::error::Error>) -> Self {
        Self::from(format!("{:?}", e))
    }
}

impl From<Box<dyn std::error::Error + Sync + Send>> for Error {
    fn from(e: Box<dyn std::error::Error + Sync + Send>) -> Self {
        Self::from(format!("{:?}", e))
    }
}

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
           SerdeJsonError(serde_json::Error);
    }
    errors {

    }
}
