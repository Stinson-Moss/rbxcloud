//! Low-level Experience API operations.
use std::fmt;

use serde::{Deserialize, Serialize};

use crate::rbx::error::Error;
use crate::rbx::http::{self, HttpClient};
use crate::rbx::v1::{PlaceId, UniverseId};

/// The version type of a place publish operation.
#[derive(Debug, Clone)]
pub enum PublishVersionType {
    /// Place is saved as the most-recent version. Players who play
    /// the game will _not_ see this version, but it _will_ be the
    /// version that is seen when loaded in with Studio.
    Saved,

    /// Place is saved as the most-recent live version. Players who
    /// play the game will play this version.
    Published,
}

impl fmt::Display for PublishVersionType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

/// Parameters for publishing a place.
pub struct PublishExperienceParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_type: PublishVersionType,
    pub filename: String,
}

/// Response from Roblox when place is published or saved.
///
/// The version number represents the latest version uploaded
/// to Roblox.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PublishExperienceResponse {
    pub version_number: u64,
}

/// Publish a place under a specific experience.
pub async fn publish_experience(
    http: &HttpClient,
    params: &PublishExperienceParams,
) -> Result<PublishExperienceResponse, Error> {
    let bytes_data_buf = std::fs::read(&params.filename)?;
    let url = http::url(&format!(
        "/universes/v1/{universeId}/places/{placeId}/versions",
        universeId = params.universe_id,
        placeId = params.place_id,
    ));

    let res = http
        .post(url)
        .query(&[("versionType", params.version_type.to_string())])
        .header("Content-Type", "application/octet-stream")
        .body(bytes_data_buf)
        .send()
        .await?;
    http::json::<PublishExperienceResponse>(res).await
}
