use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::RobloxUserId, util::QueryString};

use crate::rbx::http::{self, HttpClient};

pub struct GetUserParams {
    pub user_id: RobloxUserId,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserSocialNetworkVisibility {
    SocialNetworkVisibilityUnspecified,
    NoOne,
    Friends,
    FriendsAndFollowing,
    FriendsFollowingAndFollowers,
    Everyone,
}

#[derive(Deserialize, Serialize, Debug, Default)]
#[serde(rename_all = "camelCase")]
#[serde(default)]
pub struct UserSocialNetworkProfiles {
    pub facebook: String,
    pub twitter: String,
    pub youtube: String,
    pub twitch: String,
    pub guilded: String,
    pub visibility: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetUserResponse {
    pub path: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub display_name: String,
    #[serde(default)]
    pub about: String,
    #[serde(default)]
    pub locale: String,
    pub premium: Option<bool>,
    pub id_verified: Option<bool>,
    pub social_network_profiles: Option<UserSocialNetworkProfiles>,
}

pub struct GenerateUserThumbnailParams {
    pub user_id: RobloxUserId,
    pub size: Option<UserThumbnailSize>,
    pub format: Option<UserThumbnailFormat>,
    pub shape: Option<UserThumbnailShape>,
}

#[derive(Deserialize, Serialize, Debug, Clone, clap::ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserThumbnailFormat {
    Png,
    Jpeg,
}

#[derive(Deserialize, Serialize, Debug, Clone, clap::ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserThumbnailSize {
    Size48x48,
    Size50x50,
    Size60x60,
    Size75x75,
    Size100x100,
    Size110x110,
    Size150x150,
    Size180x180,
    Size352x352,
    Size420x420,
    Size720x720,
}

#[derive(Deserialize, Serialize, Debug, Clone, clap::ValueEnum)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum UserThumbnailShape {
    Round,
    Square,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateUserThumbnailOperationResponse {
    pub path: String,
    #[serde(default)]
    pub done: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response: Option<GenerateUserThumbnailResponse>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenerateUserThumbnailResponse {
    pub image_uri: String,
}

impl std::fmt::Display for UserThumbnailSize {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Size48x48 => "48",
                Self::Size50x50 => "50",
                Self::Size60x60 => "60",
                Self::Size75x75 => "75",
                Self::Size100x100 => "100",
                Self::Size110x110 => "110",
                Self::Size150x150 => "150",
                Self::Size180x180 => "180",
                Self::Size352x352 => "352",
                Self::Size420x420 => "420",
                Self::Size720x720 => "720",
            }
        )
    }
}

impl std::fmt::Display for UserThumbnailFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Png => "PNG",
                Self::Jpeg => "JPEG",
            }
        )
    }
}

impl std::fmt::Display for UserThumbnailShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Round => "ROUND",
                Self::Square => "SQUARE",
            }
        )
    }
}

pub async fn get_user(http: &HttpClient, params: &GetUserParams) -> Result<GetUserResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{user}",
        user = params.user_id,
    );

    let res = http.get(url).send().await?;

    http::json::<GetUserResponse>(res).await
}

pub async fn generate_thumbnail(
    http: &HttpClient,
    params: &GenerateUserThumbnailParams,
) -> Result<GenerateUserThumbnailOperationResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{user}:generateThumbnail",
        user = params.user_id,
    );

    let mut query: QueryString = vec![];
    if let Some(size) = &params.size {
        query.push(("size", size.to_string()))
    }
    if let Some(format) = &params.format {
        query.push(("format", format.to_string()))
    }
    if let Some(shape) = &params.shape {
        query.push(("shape", shape.to_string()))
    }

    let res = http.get(url).query(&query).send().await?;

    http::json::<GenerateUserThumbnailOperationResponse>(res).await
}
