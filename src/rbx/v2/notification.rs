use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::RobloxUserId};

use crate::rbx::http::{self, HttpClient};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum NotificationType {
    TypeUnspecified,
    Moment,
}

impl std::fmt::Display for NotificationType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::TypeUnspecified => "TYPE_UNSPECIFIED",
                Self::Moment => "MOMENT",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JoinExperience {
    pub launch_data: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub string_value: Option<String>,
    pub int64_value: Option<i64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSource {
    pub universe: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationPayload {
    pub message_id: String,
    #[serde(rename = "type")]
    pub notification_type: NotificationType,
    pub parameters: Option<HashMap<String, Parameter>>,
    pub join_experience: Option<JoinExperience>,
    pub analytics_data: Option<HashMap<String, String>>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    pub source: NotificationSource,
    pub payload: NotificationPayload,
}

#[derive(Debug)]
pub struct NotificationParams {
    pub user_id: RobloxUserId,
    pub notification: Notification,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NotificationResponse {
    pub path: String,
    pub id: String,
}

pub async fn send_notification(
    http: &HttpClient,
    params: &NotificationParams,
) -> Result<NotificationResponse, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{user}/notifications",
        user = params.user_id,
    );

    let res = http.post(url).json(&params.notification).send().await?;

    http::json::<NotificationResponse>(res).await
}
