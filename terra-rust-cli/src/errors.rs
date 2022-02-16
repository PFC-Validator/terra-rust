#![allow(missing_docs)]

use std::num::{ParseFloatError, ParseIntError};
use terra_rust_api::errors::TerraRustAPIError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraRustCLIError {
    #[error("Bad Implementation. Missing CLI Argument {0}")]
    MissingArgument(String),
    #[error("IO Error")]
    IOErr(#[from] ::std::io::Error),
    #[error("Number Float Error")]
    NumberFloatErr(#[from] ParseFloatError),
    #[error("Number Int Error")]
    NumberIntErr(#[from] ParseIntError),
    #[error("TerraRustAPIError Error")]
    TerraRustAPIError(#[from] TerraRustAPIError),
}
