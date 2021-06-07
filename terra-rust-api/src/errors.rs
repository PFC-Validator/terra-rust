#![allow(missing_docs)]
use thiserror::Error;
/*
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

 */
/*
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

 */
#[derive(Error, Debug)]
pub enum TerraRustAPIError {
    #[error("Reqwest HTTP(s) Error")]
    ReqwestError(#[from] ::reqwest::Error),
    #[error("Terra `{0}` CLI Error")]
    Terra(String),
    #[error("Bech32 Decode Error")]
    Bech32DecodeErr,
    #[error("Bech32 Decode Error: Key Failed prefix {0} or length {1} Wanted:{2}/{3}")]
    Bech32DecodeExpanded(String, usize, String, usize),
    #[error("Mnemonic - Bad Phrase")]
    Phrasing,
    #[error("Mnemonic - Missing Phrase")]
    MissingPhrase,
    #[error("Bad Implementation. Missing Component")]
    Implementation,
    #[error("Unable to convert into public key `{key}`")]
    Conversion {
        key: String,
        source: bitcoin::bech32::Error,
    },
    #[error("83 length-missing SECP256K1 prefix")]
    ConversionSECP256k1,
    #[error("82 length-missing ED25519 prefix")]
    ConversionED25519,
    #[error("Expected Key length of 82 or 83 length was {0}")]
    ConversionLength(usize),
    #[error("Expected ED25519 key of length 32 with a BECH32 ED25519 prefix of 5 chars - Len {0} - Hex {1}")]
    ConversionPrefixED25519(usize, String),
    #[error("Can't call Transactions without some gas rules")]
    NoGasOpts,
    #[error("Can't parse `{parse}` into a coin")]
    CoinParseErrV {
        parse: String,
        source: anyhow::Error,
    },
    #[error("Can't parse `{0}` into a coin")]
    CoinParseErr(String),

    #[error("unknown Terra-Rust API error")]
    Unknown,
}
