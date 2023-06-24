use std::{fmt, string::FromUtf8Error};

use axum::http::StatusCode;
use cashu_crab::lightning_invoice::ParseOrSemanticError;

#[derive(Debug)]
pub enum Error {
    StatusCode(StatusCode),
    SerdeError(serde_json::Error),
    Custom(String),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StatusCode(code) => write!(f, "{}", code.as_str()),
            Self::SerdeError(code) => write!(f, "{}", code.to_string()),
            Self::Custom(code) => write!(f, "{}", code),
        }
    }
}

impl From<StatusCode> for Error {
    fn from(code: StatusCode) -> Self {
        Self::StatusCode(code)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::SerdeError(err)
    }
}

impl From<cln_rpc::RpcError> for Error {
    fn from(err: cln_rpc::RpcError) -> Self {
        Self::Custom(err.to_string())
    }
}

impl From<anyhow::Error> for Error {
    fn from(err: anyhow::Error) -> Self {
        Self::Custom(err.to_string())
    }
}

impl From<bitcoin_hashes::hex::Error> for Error {
    fn from(err: bitcoin_hashes::hex::Error) -> Self {
        Self::Custom(err.to_string())
    }
}

impl From<ParseOrSemanticError> for Error {
    fn from(err: ParseOrSemanticError) -> Self {
        Self::Custom(err.to_string())
    }
}

impl From<FromUtf8Error> for Error {
    fn from(err: FromUtf8Error) -> Self {
        Self::Custom(err.to_string())
    }
}

impl From<ldk_node::NodeError> for Error {
    fn from(err: ldk_node::NodeError) -> Self {
        Self::Custom(err.to_string())
    }
}
