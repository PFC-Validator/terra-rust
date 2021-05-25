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
        ReqwestError(::reqwest::Error);
        SerdeJsonError(serde_json::Error);
        HexError(hex::FromHexError);
        ParseIntError(std::num::ParseIntError);
        ParseFloatError(std::num::ParseFloatError);
        Secp256k1(bitcoin::secp256k1::Error);
        SubtleEncoding(subtle_encoding::Error);
        Utf8(std::string::FromUtf8Error);
        Ed25519(ed25519_dalek::ed25519::Error);
        Bip32(bitcoin::util::bip32::Error);
        Decimal(rust_decimal::Error);
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
        Phrasing {
           description("Mnemonic - Bad Phrase")
           display("Mnemonic - Bad Phrase")
        }
        Implementation {
            description("Bad Implementation. Missing Component")
            display("Bad Implementation. Missing Component")
        }
        Conversion(err:String) {
            description("Unable to convert into public key")
            display("Unable to convert into public key {}",err)
        }
        NoGasOpts {
            description("Can't call Transactions without some gas rules")
            display("Can't call transactions without some gas rules")
        }
        CoinParseErr(err:String) {
            display("Can't parse {} into a coin", err)
            description("coin parse error")
        }
    }
}
