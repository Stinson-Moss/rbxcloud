//! Helpers for Luau `buffer` values stored in data stores.
//!
//! When Roblox serializes a Luau `buffer` to JSON (e.g. when it is saved in a
//! data store), it is written as a tagged object:
//!
//! ```json
//! {"m": null, "t": "buffer", "base64": "AQID"}
//! {"m": null, "t": "buffer", "zbase64": "KLUv/..."}
//! ```
//!
//! `base64` holds the raw bytes. `zbase64` holds the bytes compressed with
//! [zstd](https://facebook.github.io/zstd/) and then base64-encoded; Roblox
//! uses it when compression makes the payload smaller.
//!
//! This encoding is not formally documented by Roblox, so treat it as
//! best-effort.
//!
//! ```rust
//! use rbxcloud::rbx::buffer;
//!
//! let value = buffer::encode(&[1, 2, 3]);
//! assert!(buffer::is_buffer(&value));
//! assert_eq!(buffer::decode(&value).unwrap(), Some(vec![1, 2, 3]));
//! ```

use std::io::Read;

use base64::{engine::general_purpose::STANDARD, Engine as _};
use serde_json::{json, Value};

use crate::rbx::error::Error;

const TAG_KEY: &str = "t";
const TAG_BUFFER: &str = "buffer";
const KEY_BASE64: &str = "base64";
const KEY_ZBASE64: &str = "zbase64";

/// Returns `true` if `value` is a tagged Luau buffer object.
pub fn is_buffer(value: &Value) -> bool {
    value
        .as_object()
        .and_then(|o| o.get(TAG_KEY))
        .and_then(|t| t.as_str())
        == Some(TAG_BUFFER)
}

/// Decode a tagged Luau buffer object into its bytes.
///
/// Returns `Ok(None)` if `value` is not a buffer object, and an error if it
/// is a buffer whose payload cannot be decoded.
pub fn decode(value: &Value) -> Result<Option<Vec<u8>>, Error> {
    if !is_buffer(value) {
        return Ok(None);
    }
    let obj = value.as_object().expect("checked by is_buffer");

    if let Some(data) = obj.get(KEY_BASE64).and_then(|v| v.as_str()) {
        return decode_base64(data).map(Some);
    }

    if let Some(data) = obj.get(KEY_ZBASE64).and_then(|v| v.as_str()) {
        let compressed = decode_base64(data)?;
        let mut decoder = ruzstd::decoding::StreamingDecoder::new(compressed.as_slice())
            .map_err(|e| Error::BufferDecodeError(format!("invalid zstd frame: {e}")))?;
        let mut out = Vec::new();
        decoder
            .read_to_end(&mut out)
            .map_err(|e| Error::BufferDecodeError(format!("zstd decompression failed: {e}")))?;
        return Ok(Some(out));
    }

    Err(Error::BufferDecodeError(
        "buffer object has neither a `base64` nor a `zbase64` field".to_string(),
    ))
}

/// Encode bytes as an (uncompressed) tagged Luau buffer object, suitable for
/// writing to a data store so that Roblox reads it back as a `buffer`.
pub fn encode(bytes: &[u8]) -> Value {
    json!({
        "m": null,
        TAG_KEY: TAG_BUFFER,
        KEY_BASE64: STANDARD.encode(bytes),
    })
}

/// Recursively replace every tagged buffer inside `value` with the result of
/// `f(bytes)`. Useful for turning buffers into a readable form (e.g. hex or
/// UTF-8 text) before displaying a data store entry.
pub fn map_buffers<F>(value: &Value, f: &F) -> Result<Value, Error>
where
    F: Fn(Vec<u8>) -> Value,
{
    if let Some(bytes) = decode(value)? {
        return Ok(f(bytes));
    }
    Ok(match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|v| map_buffers(v, f))
                .collect::<Result<_, _>>()?,
        ),
        Value::Object(obj) => Value::Object(
            obj.iter()
                .map(|(k, v)| Ok((k.clone(), map_buffers(v, f)?)))
                .collect::<Result<_, Error>>()?,
        ),
        other => other.clone(),
    })
}

fn decode_base64(data: &str) -> Result<Vec<u8>, Error> {
    STANDARD
        .decode(data)
        .map_err(|e| Error::BufferDecodeError(format!("invalid base64: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Build a minimal zstd frame containing a single raw (uncompressed)
    /// block, so the decoder can be tested without a zstd encoder.
    fn raw_zstd_frame(data: &[u8]) -> Vec<u8> {
        assert!(data.len() < 256);
        let mut frame = vec![0x28, 0xB5, 0x2F, 0xFD]; // magic number
        frame.push(0x20); // descriptor: single segment, 1-byte content size
        frame.push(data.len() as u8);
        let header: u32 = 1 | ((data.len() as u32) << 3); // last block, raw
        frame.extend_from_slice(&header.to_le_bytes()[..3]);
        frame.extend_from_slice(data);
        frame
    }

    #[test]
    fn roundtrip_base64() {
        let value = encode(b"hello");
        assert_eq!(value["t"], "buffer");
        assert_eq!(value["base64"], "aGVsbG8=");
        assert_eq!(decode(&value).unwrap(), Some(b"hello".to_vec()));
    }

    #[test]
    fn decode_zbase64() {
        let frame = raw_zstd_frame(b"compressed hello");
        let encoded = STANDARD.encode(&frame);
        assert!(encoded.starts_with("KLUv/"));
        let value = json!({"m": null, "t": "buffer", "zbase64": encoded});
        assert_eq!(decode(&value).unwrap(), Some(b"compressed hello".to_vec()));
    }

    #[test]
    fn non_buffers_are_ignored() {
        assert_eq!(decode(&json!({"t": "numeric", "v": "inf"})).unwrap(), None);
        assert_eq!(decode(&json!("buffer")).unwrap(), None);
        assert!(!is_buffer(&json!([1, 2, 3])));
    }

    #[test]
    fn malformed_buffer_errors() {
        assert!(decode(&json!({"t": "buffer"})).is_err());
        assert!(decode(&json!({"t": "buffer", "base64": "!!!"})).is_err());
        assert!(decode(&json!({"t": "buffer", "zbase64": "AQID"})).is_err());
    }

    #[test]
    fn map_nested_buffers() {
        let value = json!({"inventory": [encode(&[0xAB]), 5], "name": "x"});
        let mapped = map_buffers(&value, &|bytes| json!(bytes.len())).unwrap();
        assert_eq!(mapped, json!({"inventory": [1, 5], "name": "x"}));
    }
}
