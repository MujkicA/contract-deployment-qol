use core::str::Utf8Error;
use fuels_core::errors::CodecError;
use fuels_core::InvalidOutputType;
use fuels_signers::wallet::WalletError;
use thiserror::Error;
pub type Result<T> = core::result::Result<T, Error>;
use std::net;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid name: {0}")]
    InvalidName(String),
    #[error("Invalid data")]
    InvalidData,
    #[error("Missing data: {0}")]
    MissingData(String),
    #[error("Serialization error: {0}")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Invalid type: {0}")]
    InvalidType(String),
    #[error("Parse integer error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Parse boolean error: {0}")]
    ParseBoolError(#[from] std::str::ParseBoolError),
    #[error("Parse hex error: {0}")]
    ParseHexError(#[from] hex::FromHexError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] Utf8Error),
    #[error("Compilation error: {0}")]
    CompilationError(String),
    #[error("Network error: {0}")]
    NetworkError(#[from] net::AddrParseError),
    #[error("Transaction error: {0}")]
    TransactionError(String),
    #[error("Infrastructure error: {0}")]
    InfrastructureError(String),
    #[error("Contract call error: {0}")]
    ContractCallError(String),
    #[error("Wallet error: {0}")]
    WalletError(#[from] WalletError),
}

impl From<CodecError> for Error {
    fn from(err: CodecError) -> Error {
        match err {
            CodecError::InvalidData => Error::InvalidData,
            CodecError::Utf8Error(e) => Error::Utf8Error(e),
        }
    }
}
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::ContractCallError(err.to_string())
    }
}

impl From<InvalidOutputType> for Error {
    fn from(err: InvalidOutputType) -> Error {
        Error::ContractCallError(err.0)
    }
}
