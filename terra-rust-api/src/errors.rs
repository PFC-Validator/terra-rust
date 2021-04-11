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
error_chain! { foreign_links {
    ReqwestError(::reqwest::Error);
    SerdeJsonError(serde_json::Error);
    Bech32Error(bech32::Error);
}
    errors {
        Terra(err:String) {
            description("Terra CLI Error")
            display("Terra CLI: {}" ,err)
        }
        Bech32DecodeErr {
            description("Bech32 Address Decode Error")
            display("Bech32 Address Decode Error")
        }

    }
}
