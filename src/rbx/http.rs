//! Shared HTTP client used for every Open Cloud request.
//!
//! An [`HttpClient`] owns a single connection pool along with the API key,
//! request timeouts, and proxy configuration. It is cheap to clone, so the
//! high-level clients (`v1::RbxCloud`, `v2::Client`) hand out clones of it to
//! each sub-client rather than creating a new connection pool per request.
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use rbxcloud::rbx::http::HttpClient;
//!
//! # fn main() -> Result<(), rbxcloud::rbx::error::Error> {
//! let http = HttpClient::builder("API_KEY")
//!     .timeout(Duration::from_secs(30))
//!     .proxy("http://localhost:8080")
//!     .build()?;
//! # Ok(())
//! # }
//! ```

use std::time::Duration;

use reqwest::{IntoUrl, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;

use crate::rbx::error::Error;

/// Default total time allowed for a single request, including uploading the
/// request body and downloading the response.
pub const DEFAULT_TIMEOUT: Duration = Duration::from_secs(120);

/// Default time allowed to establish a connection.
pub const DEFAULT_CONNECT_TIMEOUT: Duration = Duration::from_secs(30);

/// Base URL for all Roblox Open Cloud APIs.
pub(crate) const BASE_URL: &str = "https://apis.roblox.com";

const USER_AGENT: &str = concat!("rbxcloud/", env!("CARGO_PKG_VERSION"));

/// HTTP client configured for Roblox Open Cloud requests.
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: reqwest::Client,
    api_key: String,
}

/// Builder for [`HttpClient`].
#[derive(Debug, Clone)]
pub struct HttpClientBuilder {
    api_key: String,
    timeout: Option<Duration>,
    connect_timeout: Option<Duration>,
    proxy: Option<String>,
    no_proxy: bool,
    user_agent: Option<String>,
}

impl HttpClientBuilder {
    /// Set the total timeout for each request. Defaults to [`DEFAULT_TIMEOUT`].
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Disable the total request timeout. Requests may then hang
    /// indefinitely if Roblox never responds.
    pub fn no_timeout(mut self) -> Self {
        self.timeout = None;
        self
    }

    /// Set the connection timeout. Defaults to [`DEFAULT_CONNECT_TIMEOUT`].
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.connect_timeout = Some(timeout);
        self
    }

    /// Route all requests through the given proxy URL (e.g.
    /// `http://user:pass@host:port` or `socks5://host:port`).
    ///
    /// Without this, the standard `HTTPS_PROXY`/`HTTP_PROXY`/`ALL_PROXY`
    /// environment variables are honored.
    pub fn proxy(mut self, url: impl Into<String>) -> Self {
        self.proxy = Some(url.into());
        self
    }

    /// Ignore proxy environment variables and connect directly.
    pub fn no_proxy(mut self) -> Self {
        self.no_proxy = true;
        self
    }

    /// Override the `User-Agent` header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = Some(user_agent.into());
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<HttpClient, Error> {
        let mut builder = reqwest::Client::builder()
            .user_agent(self.user_agent.unwrap_or_else(|| USER_AGENT.to_string()));
        if let Some(timeout) = self.timeout {
            builder = builder.timeout(timeout);
        }
        if let Some(timeout) = self.connect_timeout {
            builder = builder.connect_timeout(timeout);
        }
        if self.no_proxy {
            builder = builder.no_proxy();
        }
        if let Some(proxy) = &self.proxy {
            builder = builder.proxy(reqwest::Proxy::all(proxy)?);
        }
        Ok(HttpClient {
            client: builder.build()?,
            api_key: self.api_key,
        })
    }
}

impl HttpClient {
    /// Create a client with default settings.
    ///
    /// # Panics
    ///
    /// Panics if the TLS backend cannot be initialized, in the same way as
    /// `reqwest::Client::new()`. Use [`HttpClient::builder`] to handle that
    /// error instead.
    pub fn new(api_key: &str) -> Self {
        Self::builder(api_key)
            .build()
            .expect("failed to build HTTP client")
    }

    /// Start building a client with custom settings.
    pub fn builder(api_key: &str) -> HttpClientBuilder {
        HttpClientBuilder {
            api_key: api_key.to_string(),
            timeout: Some(DEFAULT_TIMEOUT),
            connect_timeout: Some(DEFAULT_CONNECT_TIMEOUT),
            proxy: None,
            no_proxy: false,
            user_agent: None,
        }
    }

    /// Wrap an existing `reqwest::Client`, keeping whatever configuration it
    /// was built with.
    pub fn from_reqwest(api_key: &str, client: reqwest::Client) -> Self {
        Self {
            client,
            api_key: api_key.to_string(),
        }
    }

    /// The API key sent with each request.
    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    /// The underlying `reqwest::Client`.
    pub fn reqwest_client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Start a request with the API key attached.
    pub fn request(&self, method: Method, url: impl IntoUrl) -> RequestBuilder {
        self.client
            .request(method, url)
            .header("x-api-key", &self.api_key)
    }

    pub(crate) fn get(&self, url: impl IntoUrl) -> RequestBuilder {
        self.request(Method::GET, url)
    }

    pub(crate) fn post(&self, url: impl IntoUrl) -> RequestBuilder {
        self.request(Method::POST, url)
    }

    pub(crate) fn patch(&self, url: impl IntoUrl) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }

    pub(crate) fn delete(&self, url: impl IntoUrl) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }
}

/// Build a full Open Cloud URL from a path beginning with `/`.
pub(crate) fn url(path: &str) -> String {
    format!("{BASE_URL}{path}")
}

/// Return the response if successful, otherwise convert it into an
/// [`Error::HttpStatusError`] that carries Roblox's error message.
pub(crate) async fn check(res: Response) -> Result<Response, Error> {
    let status = res.status();
    if status.is_success() {
        return Ok(res);
    }
    let body = res.text().await.unwrap_or_default();
    Err(status_error(status.as_u16(), &body))
}

/// Deserialize a successful JSON response.
pub(crate) async fn json<T: DeserializeOwned>(res: Response) -> Result<T, Error> {
    let res = check(res).await?;
    // Read as bytes first so that decode errors report the serde path
    // rather than an opaque reqwest decode error.
    let bytes = res.bytes().await?;
    Ok(serde_json::from_slice(&bytes)?)
}

/// Discard the body of a successful response.
pub(crate) async fn empty(res: Response) -> Result<(), Error> {
    check(res).await.map(|_| ())
}

/// Build an error from a non-success status and response body.
///
/// Open Cloud returns several error shapes depending on the API:
/// - v2: `{"code": "INVALID_ARGUMENT", "message": "..."}`
/// - v1: `{"error": "INVALID_ARGUMENT", "message": "..."}`
/// - legacy: `{"errors": [{"code": 0, "message": "..."}]}`
pub(crate) fn status_error(code: u16, body: &str) -> Error {
    let reason = reqwest::StatusCode::from_u16(code)
        .ok()
        .and_then(|s| s.canonical_reason())
        .unwrap_or("unknown error");

    let msg = match error_message(body) {
        Some(detail) => format!("{reason}: {detail}"),
        None if body.trim().is_empty() => reason.to_string(),
        None => format!("{reason}: {}", truncate(body.trim(), 500)),
    };

    Error::HttpStatusError { code, msg }
}

fn error_message(body: &str) -> Option<String> {
    let value: serde_json::Value = serde_json::from_str(body).ok()?;
    let obj = value.as_object()?;

    let message = obj
        .get("message")
        .and_then(|m| m.as_str())
        .or_else(|| {
            obj.get("errors")
                .and_then(|e| e.as_array())
                .and_then(|e| e.first())
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
        })
        .map(str::to_string);

    let code = obj
        .get("code")
        .or_else(|| obj.get("error"))
        .and_then(|c| c.as_str());

    match (code, message) {
        (Some(code), Some(message)) if !message.is_empty() => Some(format!("{code}: {message}")),
        (_, Some(message)) if !message.is_empty() => Some(message),
        (Some(code), _) => Some(code.to_string()),
        _ => None,
    }
}

fn truncate(s: &str, max: usize) -> &str {
    match s.char_indices().nth(max) {
        Some((idx, _)) => &s[..idx],
        None => s,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn msg(code: u16, body: &str) -> String {
        match status_error(code, body) {
            Error::HttpStatusError { msg, .. } => msg,
            other => panic!("unexpected error: {other:?}"),
        }
    }

    #[test]
    fn v2_error_shape() {
        let body = r#"{"code":"INVALID_ARGUMENT","message":"Entry key is too long.","details":[]}"#;
        assert_eq!(
            msg(400, body),
            "Bad Request: INVALID_ARGUMENT: Entry key is too long."
        );
    }

    #[test]
    fn v1_error_shape() {
        let body = r#"{"error":"NOT_FOUND","message":"Entry not found.","errorDetails":[]}"#;
        assert_eq!(msg(404, body), "Not Found: NOT_FOUND: Entry not found.");
    }

    #[test]
    fn legacy_error_shape() {
        let body = r#"{"errors":[{"code":0,"message":"Unauthorized"}]}"#;
        assert_eq!(msg(401, body), "Unauthorized: Unauthorized");
    }

    #[test]
    fn non_json_body() {
        assert_eq!(
            msg(503, "upstream down"),
            "Service Unavailable: upstream down"
        );
        assert_eq!(msg(500, ""), "Internal Server Error");
    }

    #[test]
    fn builder_rejects_bad_proxy() {
        assert!(HttpClient::builder("key")
            .proxy("::not a url::")
            .build()
            .is_err());
    }

    #[test]
    fn builder_defaults() {
        let http = HttpClient::builder("key").build().unwrap();
        assert_eq!(http.api_key(), "key");
    }
}
