//! Universe secrets store.
//!
//! Secret values are encrypted locally with a libsodium sealed box against
//! the universe's public key before being sent; Roblox never receives the
//! plaintext over the wire, and secret values can never be read back.
//! [`create_secret`] and [`update_secret`] handle the encryption.

use base64::{engine::general_purpose::STANDARD, Engine as _};
use crypto_box::{aead::OsRng, PublicKey};
use serde::{Deserialize, Serialize};

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::UniverseId,
    util::{encode_path_segment as seg, QueryString},
};

/// A secret's metadata. `secret` is only populated for the public key.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct Secret {
    #[serde(default)]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key_id: Option<String>,
    /// Domain the secret may be sent to from game servers (e.g.
    /// `*.example.com`), if restricted.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListSecretsResponse {
    #[serde(default)]
    pub secrets: Vec<Secret>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_page_cursor: Option<String>,
}

/// A universe's public key, used to encrypt secrets.
#[derive(Debug, Clone)]
pub struct UniversePublicKey {
    pub key_id: String,
    key: PublicKey,
}

impl UniversePublicKey {
    /// Parse the public key response.
    pub fn from_secret(secret: &Secret) -> Result<Self, Error> {
        let encoded = secret
            .secret
            .as_deref()
            .ok_or_else(|| Error::EncryptionError("public key response has no key".into()))?;
        let key_id = secret
            .key_id
            .clone()
            .ok_or_else(|| Error::EncryptionError("public key response has no key_id".into()))?;
        let bytes = STANDARD
            .decode(encoded)
            .map_err(|e| Error::EncryptionError(format!("public key is not base64: {e}")))?;
        let bytes: [u8; 32] = bytes.try_into().map_err(|b: Vec<u8>| {
            Error::EncryptionError(format!("public key is {} bytes, expected 32", b.len()))
        })?;
        Ok(Self {
            key_id,
            key: PublicKey::from(bytes),
        })
    }

    /// Encrypt `plaintext` into a base64 sealed box.
    pub fn seal(&self, plaintext: &[u8]) -> Result<String, Error> {
        let sealed = self
            .key
            .seal(&mut OsRng, plaintext)
            .map_err(|e| Error::EncryptionError(e.to_string()))?;
        Ok(STANDARD.encode(sealed))
    }
}

fn secrets_url(universe_id: UniverseId) -> String {
    http::url(&format!("/cloud/v2/universes/{universe_id}/secrets"))
}

#[derive(Serialize)]
struct SecretBody<'a> {
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<&'a str>,
    secret: String,
    key_id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    domain: Option<&'a str>,
}

/// List the universe's secrets (metadata only; values are never returned).
pub async fn list_secrets(
    http: &HttpClient,
    universe_id: UniverseId,
    limit: Option<u32>,
    cursor: Option<String>,
) -> Result<ListSecretsResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(limit) = limit {
        query.push(("limit", limit.to_string()));
    }
    if let Some(cursor) = cursor {
        query.push(("cursor", cursor));
    }
    let res = http
        .get(secrets_url(universe_id))
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get the universe's public key for encrypting secrets.
pub async fn get_public_key(
    http: &HttpClient,
    universe_id: UniverseId,
) -> Result<UniversePublicKey, Error> {
    let res = http
        .get(format!("{}/public-key", secrets_url(universe_id)))
        .send()
        .await?;
    let secret: Secret = http::json(res).await?;
    UniversePublicKey::from_secret(&secret)
}

/// Create a secret. The value is encrypted before it is sent.
///
/// `domain` restricts which domains game servers may send the secret to.
pub async fn create_secret(
    http: &HttpClient,
    universe_id: UniverseId,
    secret_id: &str,
    value: &[u8],
    domain: Option<&str>,
) -> Result<Secret, Error> {
    let key = get_public_key(http, universe_id).await?;
    let body = SecretBody {
        id: Some(secret_id),
        secret: key.seal(value)?,
        key_id: &key.key_id,
        domain,
    };
    let res = http
        .post(secrets_url(universe_id))
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

/// Replace a secret's value (and optionally its allowed domain).
pub async fn update_secret(
    http: &HttpClient,
    universe_id: UniverseId,
    secret_id: &str,
    value: &[u8],
    domain: Option<&str>,
) -> Result<Secret, Error> {
    let key = get_public_key(http, universe_id).await?;
    let body = SecretBody {
        id: None,
        secret: key.seal(value)?,
        key_id: &key.key_id,
        domain,
    };
    let res = http
        .patch(format!("{}/{}", secrets_url(universe_id), seg(secret_id)))
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

/// Delete a secret.
pub async fn delete_secret(
    http: &HttpClient,
    universe_id: UniverseId,
    secret_id: &str,
) -> Result<(), Error> {
    let res = http
        .delete(format!("{}/{}", secrets_url(universe_id), seg(secret_id)))
        .send()
        .await?;
    http::empty(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crypto_box::SecretKey;

    #[test]
    fn sealed_box_roundtrip() {
        // Encrypt against a known key pair and confirm the recipient can open
        // it, as Roblox would.
        let recipient = SecretKey::generate(&mut OsRng);
        let public = Secret {
            id: "public-key".into(),
            secret: Some(STANDARD.encode(recipient.public_key().as_bytes())),
            key_id: Some("1200590785272263122".into()),
            ..Default::default()
        };
        let key = UniversePublicKey::from_secret(&public).unwrap();
        let sealed = STANDARD
            .decode(key.seal(b"my_api_key_content").unwrap())
            .unwrap();

        // Sealed box = 32-byte ephemeral key + 16-byte MAC + ciphertext.
        assert_eq!(sealed.len(), 32 + 16 + b"my_api_key_content".len());
        assert_eq!(recipient.unseal(&sealed).unwrap(), b"my_api_key_content");
    }

    #[test]
    fn rejects_bad_public_key() {
        let bad = Secret {
            secret: Some(STANDARD.encode([1u8; 16])),
            key_id: Some("1".into()),
            ..Default::default()
        };
        assert!(UniversePublicKey::from_secret(&bad).is_err());
    }

    #[test]
    fn create_body_shape() {
        let body = SecretBody {
            id: Some("mySecret"),
            secret: "abc".into(),
            key_id: "123",
            domain: None,
        };
        assert_eq!(
            serde_json::to_value(body).unwrap(),
            serde_json::json!({"id": "mySecret", "secret": "abc", "key_id": "123"})
        );
    }
}
