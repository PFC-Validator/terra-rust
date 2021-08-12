#![allow(missing_docs)]
use thiserror::Error;

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
    #[error("TX submit returned `{0}` - {1} '{2}'")]
    TxResultError(usize, String, String),

    #[error("unknown Terra-Rust API error")]
    Unknown,
}
