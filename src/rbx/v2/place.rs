use crate::rbx::http::{self, HttpClient};
use crate::rbx::{
    error::Error,
    types::{PlaceId, UniverseId},
    util::QueryString,
};
use serde::{Deserialize, Serialize};

pub struct GetPlaceParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlaceInfo {
    pub path: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub server_size: u32,
    /// Whether this is the universe's root (start) place.
    #[serde(default)]
    pub root: bool,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdatePlaceInfo {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_size: Option<u32>,
}

pub struct UpdatePlaceParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub update_mask: String,
    pub info: UpdatePlaceInfo,
}

pub async fn get_place(http: &HttpClient, params: &GetPlaceParams) -> Result<PlaceInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}",
        universeId = params.universe_id,
        placeId = params.place_id,
    );

    let res = http.get(&url).send().await?;

    http::json::<PlaceInfo>(res).await
}

pub async fn update_place(
    http: &HttpClient,
    params: &UpdatePlaceParams,
) -> Result<PlaceInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}",
        universeId = params.universe_id,
        placeId = params.place_id,
    );

    let query: QueryString = vec![("updateMask", params.update_mask.clone())];

    let body = serde_json::to_string(&params.info)?;

    let res = http
        .patch(url)
        .header("Content-Type", "application/json")
        .body(body)
        .query(&query)
        .send()
        .await?;

    http::json::<PlaceInfo>(res).await
}
