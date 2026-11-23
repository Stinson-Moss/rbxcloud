use base64::{engine::general_purpose::STANDARD, Engine as _};
use md5::{Digest, Md5};
use serde::{Deserialize, Deserializer};

pub type QueryString = Vec<(&'static str, String)>;

#[inline]
pub fn get_checksum_base64(data: &String) -> String {
    let mut md5_hash = Md5::new();
    md5_hash.update(data.as_bytes());
    STANDARD.encode(md5_hash.finalize())
}

/// Percent-encode a single URL path segment.
///
/// Data store names, scopes, and entry keys may contain any character,
/// including `/`, `:`, `?`, `#`, and spaces, so every byte outside the RFC 3986
/// unreserved set is encoded.
pub fn encode_path_segment(segment: &str) -> String {
    let mut out = String::with_capacity(segment.len());
    for byte in segment.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'.' | b'_' | b'~' => {
                out.push(byte as char)
            }
            _ => out.push_str(&format!("%{byte:02X}")),
        }
    }
    out
}

/// Deserialize an integer that Open Cloud may send as a JSON integer, a float
/// with no fractional part (e.g. `5.0`), or a string (proto3 int64 encoding).
pub fn de_lenient_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Lenient {
        Int(i64),
        Float(f64),
        Str(String),
    }

    match Lenient::deserialize(deserializer)? {
        Lenient::Int(n) => Ok(n),
        Lenient::Float(f) => Ok(f.round() as i64),
        Lenient::Str(s) => s
            .parse::<i64>()
            .or_else(|_| s.parse::<f64>().map(|f| f.round() as i64))
            .map_err(serde::de::Error::custom),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encodes_reserved_characters() {
        assert_eq!(encode_path_segment("Player_123"), "Player_123");
        assert_eq!(encode_path_segment("a b/c:d"), "a%20b%2Fc%3Ad");
        assert_eq!(encode_path_segment("ü"), "%C3%BC");
    }

    #[test]
    fn lenient_i64() {
        #[derive(Deserialize)]
        struct W {
            #[serde(deserialize_with = "de_lenient_i64")]
            v: i64,
        }
        let parse = |s: &str| serde_json::from_str::<W>(s).unwrap().v;
        assert_eq!(parse(r#"{"v": 5}"#), 5);
        assert_eq!(parse(r#"{"v": 5.0}"#), 5);
        assert_eq!(parse(r#"{"v": "-12"}"#), -12);
        assert_eq!(parse(r#"{"v": "9007199254740993"}"#), 9007199254740993);
    }
}
