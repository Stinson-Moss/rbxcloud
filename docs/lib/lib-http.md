# HTTP Configuration

Every request goes through a shared `HttpClient`, which holds the API key, a connection pool, and the timeout and proxy settings. `Client::new` (v2) and `RbxCloud::new` (v1) create one with default settings:

| Setting | Default |
| -- | -- |
| Total request timeout | 120 seconds |
| Connect timeout | 30 seconds |
| Proxy | `HTTPS_PROXY` / `HTTP_PROXY` / `ALL_PROXY` environment variables |

## Custom settings

Build an `HttpClient` and pass it to `with_http`:

```rust
use std::time::Duration;
use rbxcloud::{http::HttpClient, v2::Client};

let http = HttpClient::builder("API_KEY")
    .timeout(Duration::from_secs(30))
    .connect_timeout(Duration::from_secs(5))
    .proxy("http://user:pass@proxy.local:8080")
    .build()?;

let client = Client::with_http(http.clone());
let legacy = rbxcloud::v1::RbxCloud::with_http(http);
```

Builder options:

- `timeout(Duration)` / `no_timeout()`
- `connect_timeout(Duration)`
- `proxy(url)`: supports `http://`, `https://`, and `socks5://` URLs
- `no_proxy()`: ignore proxy environment variables
- `user_agent(String)`

An existing `reqwest::Client` can also be wrapped with `HttpClient::from_reqwest(api_key, client)`.

## CLI

The CLI exposes the same settings as global flags, available on every command:

```
--request-timeout <SECONDS>   Maximum seconds to wait for each request (0 = no limit) [env: RBXCLOUD_REQUEST_TIMEOUT] [default: 120]
--proxy <PROXY>               Proxy URL for all requests [env: RBXCLOUD_PROXY]
```

## Errors

When Roblox returns a non-success status, the error includes Roblox's own error code and message:

```
http 400: Bad Request: INVALID_ARGUMENT: Entry key is too long.
```
