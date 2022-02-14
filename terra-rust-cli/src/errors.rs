#![allow(missing_docs)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum TerraRustCLIError {
    #[error("Bad Implementation. Missing CLI Argument {0}")]
    MissingArgument(String),
}
