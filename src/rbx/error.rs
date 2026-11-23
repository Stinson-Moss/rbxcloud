//! Error handling.
use crate::rbx::v1::ds_error::DataStoreErrorResponse;

/// `rbxcloud` error.
#[derive(Debug)]
pub enum Error {
    /// An error occurred regarding reading a file from the file system.
    FileLoadError(String),

    /// Failed to infer asset type.
    InferAssetTypeError(String),

    /// A non-OK HTTP status was returned.
    HttpStatusError { code: u16, msg: String },

    /// An error within the `reqwest` module occurred.
    ReqwestError(reqwest::Error),

    /// An IO error occurred.
    IOError(std::io::Error),

    /// A JSON serialization error occurred.
    SerdeJsonError(serde_json::Error),

    /// A DataStore error occurred.
    DataStoreError(DataStoreErrorResponse),

    /// Failed to parse a float.
    ParseFloatError(std::num::ParseFloatError),

    /// Endpoint error.
    EndpointError(String),

    /// A Luau buffer value could not be decoded.
    BufferDecodeError(String),

    /// A long-running operation failed or did not finish in time.
    OperationError { code: i64, msg: String },

    /// A secret could not be encrypted.
    EncryptionError(String),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::ReqwestError(e) => Some(e),
            Self::IOError(e) => Some(e),
            Self::SerdeJsonError(e) => Some(e),
            Self::ParseFloatError(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Self::ReqwestError(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Self::IOError(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Self::SerdeJsonError(e)
    }
}

impl From<std::num::ParseFloatError> for Error {
    fn from(e: std::num::ParseFloatError) -> Self {
        Self::ParseFloatError(e)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match self {
            Self::FileLoadError(s) => write!(f, "failed to read file: {s}"),
            Self::InferAssetTypeError(s) => write!(f, "failed to infer asset type: {s}"),
            Self::HttpStatusError { code, msg } => write!(f, "http {code}: {msg}"),
            Self::ReqwestError(e) => write!(f, "request failed: {e}"),
            Self::IOError(e) => write!(f, "io error: {e}"),
            Self::SerdeJsonError(e) => write!(f, "json error: {e}"),
            Self::DataStoreError(e) => write!(f, "datastore error: {e}"),
            Self::ParseFloatError(e) => write!(f, "{e}"),
            Self::EndpointError(s) => write!(f, "endpoint error: {s}"),
            Self::BufferDecodeError(s) => write!(f, "failed to decode buffer: {s}"),
            Self::OperationError { code, msg } => write!(f, "operation failed ({code}): {msg}"),
            Self::EncryptionError(s) => write!(f, "failed to encrypt secret: {s}"),
        }
    }
}
