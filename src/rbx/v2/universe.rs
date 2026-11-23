use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::UniverseId, util::QueryString};

use crate::rbx::http::{self, HttpClient};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniverseVisibility {
    VisibilityUnspecified,
    Public,
    Private,
    /// A value added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UniverseAgeRating {
    AgeRatingUnspecified,
    AgeRatingAll,
    AgeRating9Plus,
    AgeRating13Plus,
    AgeRating17Plus,
    /// A value added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UniverseSocialLink {
    pub title: String,
    pub uri: String,
}

pub struct GetUniverseParams {
    pub universe_id: UniverseId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UniverseInfo {
    pub path: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub update_time: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub description: String,
    pub user: Option<String>,
    pub group: Option<String>,
    pub visibility: UniverseVisibility,
    pub facebook_social_link: Option<UniverseSocialLink>,
    pub twitter_social_link: Option<UniverseSocialLink>,
    pub youtube_social_link: Option<UniverseSocialLink>,
    pub twitch_social_link: Option<UniverseSocialLink>,
    pub discord_social_link: Option<UniverseSocialLink>,
    pub roblox_group_social_link: Option<UniverseSocialLink>,
    pub guilded_social_link: Option<UniverseSocialLink>,
    #[serde(default)]
    pub voice_chat_enabled: bool,
    pub age_rating: UniverseAgeRating,
    /// Absent when private servers are disabled.
    #[serde(default)]
    pub private_server_price_robux: Option<u32>,
    #[serde(default)]
    pub desktop_enabled: bool,
    #[serde(default)]
    pub mobile_enabled: bool,
    #[serde(default)]
    pub tablet_enabled: bool,
    #[serde(default)]
    pub console_enabled: bool,
    #[serde(default)]
    pub vr_enabled: bool,
    /// Resource path of the universe's root place.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub root_place: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateUniverseInfo {
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
    pub user: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<UniverseVisibility>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub facebook_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitter_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub youtube_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub twitch_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discord_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roblox_group_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub guilded_social_link: Option<UniverseSocialLink>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voice_chat_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub age_rating: Option<UniverseAgeRating>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_price_robux: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desktop_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tablet_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub console_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vr_enabled: Option<bool>,
}

pub struct UpdateUniverseParams {
    pub universe_id: UniverseId,
    pub update_mask: String,
    pub info: UpdateUniverseInfo,
}

pub struct RestartUniverseServersParams {
    pub universe_id: UniverseId,
}

pub struct PublishUniverseMessageParams {
    pub universe_id: UniverseId,
    pub topic: String,
    pub message: String,
}

#[derive(Serialize)]
struct PublishUniverseMessageRequest<'a> {
    topic: &'a str,
    message: &'a str,
}

pub async fn get_universe(
    http: &HttpClient,
    params: &GetUniverseParams,
) -> Result<UniverseInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}",
        universeId = params.universe_id,
    );

    let res = http.get(url).send().await?;

    http::json::<UniverseInfo>(res).await
}

pub async fn update_universe(
    http: &HttpClient,
    params: &UpdateUniverseParams,
) -> Result<UniverseInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}",
        universeId = params.universe_id,
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

    http::json::<UniverseInfo>(res).await
}

pub async fn restart_universe_servers(
    http: &HttpClient,
    params: &RestartUniverseServersParams,
) -> Result<(), Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/universes/{universeId}:restartServers",
        universeId = params.universe_id,
    );

    let body = "{}";

    let res = http
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;

    http::empty(res).await
}

/// Publish a message to live servers in the universe. Servers receive it
/// through `MessagingService:SubscribeAsync(topic)`.
pub async fn publish_message(
    http: &HttpClient,
    params: &PublishUniverseMessageParams,
) -> Result<(), Error> {
    let url = http::url(&format!(
        "/cloud/v2/universes/{universeId}:publishMessage",
        universeId = params.universe_id,
    ));
    let res = http
        .post(url)
        .json(&PublishUniverseMessageRequest {
            topic: &params.topic,
            message: &params.message,
        })
        .send()
        .await?;
    http::empty(res).await
}
