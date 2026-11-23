//! Long-running operations.
//!
//! Some Open Cloud methods (e.g. the Instance API) don't return their result
//! directly. They return an [`Operation`] whose `path` can be polled until
//! `done` is `true`, at which point either `response` or `error` is set.

use std::time::Duration;

use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
};

/// Error status of a failed operation.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationStatus {
    #[serde(default)]
    pub code: i64,
    #[serde(default)]
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<Value>,
}

/// A long-running operation whose eventual result is `T`.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
#[serde(bound(deserialize = "T: DeserializeOwned"))]
pub struct Operation<T> {
    /// Resource path, e.g. `universes/1/places/2/instances/root/operations/abc`.
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub done: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<OperationStatus>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<T>,
}

impl<T> Operation<T> {
    /// Convert a finished operation into its result.
    ///
    /// Returns an error if the operation failed, or if it is not done yet.
    pub fn into_result(self) -> Result<T, Error> {
        if let Some(status) = self.error {
            return Err(Error::OperationError {
                code: status.code,
                msg: status.message,
            });
        }
        match (self.done, self.response) {
            (true, Some(response)) => Ok(response),
            (true, None) => Err(Error::OperationError {
                code: 0,
                msg: "operation finished without a response".to_string(),
            }),
            (false, _) => Err(Error::OperationError {
                code: 0,
                msg: format!("operation {} has not finished", self.path),
            }),
        }
    }
}

/// How often and how long to poll an operation.
#[derive(Debug, Clone, Copy)]
pub struct PollOptions {
    /// Delay between polls.
    pub interval: Duration,
    /// Give up after this many polls.
    pub max_attempts: u32,
}

impl Default for PollOptions {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            max_attempts: 60,
        }
    }
}

/// Fetch the current state of an operation by its path.
pub async fn get_operation<T: DeserializeOwned>(
    http: &HttpClient,
    path: &str,
) -> Result<Operation<T>, Error> {
    // Some APIs return paths that already include the `cloud/v2/` prefix.
    let path = path.trim_start_matches('/');
    let path = path.strip_prefix("cloud/v2/").unwrap_or(path);
    let res = http
        .get(http::url(&format!("/cloud/v2/{path}")))
        .send()
        .await?;
    http::json(res).await
}

/// Poll an operation until it finishes, then return its result.
///
/// If `operation` is already done, no requests are made.
pub async fn wait<T: DeserializeOwned>(
    http: &HttpClient,
    operation: Operation<T>,
    options: PollOptions,
) -> Result<T, Error> {
    let mut operation = operation;
    let mut attempts = 0;
    while !operation.done {
        if attempts >= options.max_attempts {
            return Err(Error::OperationError {
                code: 0,
                msg: format!(
                    "timed out waiting for operation {} after {attempts} polls",
                    operation.path
                ),
            });
        }
        tokio::time::sleep(options.interval).await;
        operation = get_operation(http, &operation.path).await?;
        attempts += 1;
    }
    operation.into_result()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn done_with_response() {
        let op: Operation<Value> = serde_json::from_value(json!({
            "path": "universes/1/places/2/instances/root/operations/abc",
            "done": true,
            "response": {"@type": "x", "value": 1}
        }))
        .unwrap();
        assert_eq!(op.into_result().unwrap()["value"], 1);
    }

    #[test]
    fn done_with_error() {
        let op: Operation<Value> = serde_json::from_value(json!({
            "path": "p",
            "done": true,
            "error": {"code": 422, "message": "too large"}
        }))
        .unwrap();
        match op.into_result() {
            Err(Error::OperationError { code, msg }) => {
                assert_eq!(code, 422);
                assert_eq!(msg, "too large");
            }
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn pending_is_not_a_result() {
        let op: Operation<Value> = serde_json::from_value(json!({"path": "p"})).unwrap();
        assert!(!op.done);
        assert!(op.into_result().is_err());
    }
}
