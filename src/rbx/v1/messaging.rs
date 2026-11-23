//! Low-level Messaging API operations.
//!
//! Roblox also exposes message publishing through the v2 Universe API; see
//! `v2::universe::publish_message`.
use serde_json::json;

use crate::rbx::error::Error;
use crate::rbx::http::{self, HttpClient};
use crate::rbx::v1::UniverseId;

/// Message publishing parameters.
pub struct PublishMessageParams {
    pub universe_id: UniverseId,
    pub topic: String,
    pub message: String,
}

/// Publish a message.
pub async fn publish_message(
    http: &HttpClient,
    params: &PublishMessageParams,
) -> Result<(), Error> {
    let url = http::url(&format!(
        "/messaging-service/v1/universes/{universeId}/topics/{topic}",
        universeId = params.universe_id,
        topic = params.topic,
    ));
    let res = http
        .post(url)
        .json(&json!({ "message": &params.message }))
        .send()
        .await?;
    http::empty(res).await
}
