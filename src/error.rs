use std::fmt;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use cashu_crab::lightning_invoice::ParseOrSemanticError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    InvoiceNotPaid,
    InvoiceExpired,
    DecodeInvoice,
    StatusCode(StatusCode),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvoiceNotPaid => write!(f, "Lightning invoice not paid yet."),
            Self::InvoiceExpired => write!(f, "Lightning invoice expired."),
            Self::DecodeInvoice => write!(f, "Failed to decode LN Invoice"),
            Self::StatusCode(code) => write!(f, "{}", code.as_str()),
        }
    }
}

impl From<StatusCode> for Error {
    fn from(code: StatusCode) -> Self {
        Self::StatusCode(code)
    }
}

impl From<ParseOrSemanticError> for Error {
    fn from(_err: ParseOrSemanticError) -> Self {
        Self::DecodeInvoice
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    code: u16,
    error: String,
}

impl ErrorResponse {
    pub fn new(code: u16, error: &str) -> Self {
        Self {
            code,
            error: error.to_string(),
        }
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::InvoiceNotPaid => (
                StatusCode::OK,
                ErrorResponse::new(0, &self.to_string()).as_json(),
            )
                .into_response(),
            Error::DecodeInvoice => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::InvoiceExpired => (StatusCode::BAD_REQUEST, self.to_string()).into_response(),
            Error::StatusCode(code) => (code, "").into_response(),
        }
    }
}
