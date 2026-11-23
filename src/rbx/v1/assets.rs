use std::{fs, path::Path};

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    util::QueryString,
};
use reqwest::multipart;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetUserCreator {
    pub user_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetGroupCreator {
    pub group_id: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AssetCreator {
    User(AssetUserCreator),
    Group(AssetGroupCreator),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetCreationContext {
    pub creator: AssetCreator,
    /// Expected upload fee in Robux. Omitted from the request when `None`;
    /// Roblox rejects an explicit `null` (see issue #33).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_price: Option<u64>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub asset_type: String,
    pub asset_id: u64,
    pub creation_context: AssetCreationContext,
    pub description: String,
    pub display_name: String,
    pub path: String,
    pub revision_id: String,
    pub revision_create_time: String,
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetCreation {
    pub asset_type: AssetType,
    pub display_name: String,
    pub description: String,
    pub creation_context: AssetCreationContext,
}

pub struct CreateAssetParams {
    pub asset: AssetCreation,
    pub filepath: String,
}

pub struct CreateAssetParamsWithContents<'a> {
    pub asset: AssetCreation,
    pub contents: &'a [u8],
}

pub struct UpdateAssetParams {
    pub asset_id: u64,
    pub asset_type: AssetType,
    pub filepath: String,
}

pub struct GetAssetOperationParams {
    pub operation_id: String,
}

pub struct GetAssetParams {
    pub asset_id: u64,
    pub read_mask: Option<String>,
}

pub struct ArchiveAssetParams {
    pub asset_id: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetOperation {
    pub path: Option<String>,
    pub metadata: Option<ProtobufAny>,
    pub done: Option<bool>,
    pub error: Option<AssetErrorStatus>,
    pub response: Option<ProtobufAny>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ProtobufAny {
    #[serde(rename = "@type")]
    pub message_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetGetOperation {
    pub path: String,
    pub done: Option<bool>,
    pub response: Option<AssetGetOperationResponse>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetGetOperationResponse {
    pub path: String,
    pub revision_id: String,
    pub revision_create_time: String,
    pub asset_id: String,
    pub display_name: String,
    pub description: String,
    pub asset_type: String,
    pub creation_context: AssetCreationContext,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfo {
    pub asset_type: AssetTypeCategory,
    pub asset_id: String,
    pub creation_context: AssetCreationContext,
    pub description: String,
    pub display_name: String,
    pub path: String,
    pub revision_id: String,
    pub revision_create_time: String,
    pub moderation_result: ModerationResult,
    pub state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModerationResult {
    /// Note: There's a discrepancy between the Open Cloud docs and the actual
    /// data from the API regarding what this value can be, hence it has been
    /// left as a string and not an enum.
    pub moderation_state: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AssetErrorStatus {
    pub code: u64,
    pub message: String,
    pub details: Vec<ProtobufAny>,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Deserialize)]
pub enum AssetType {
    AudioMp3,
    AudioOgg,
    AudioFlac,
    AudioWav,
    DecalPng,
    DecalJpeg,
    DecalBmp,
    DecalTga,
    ModelFbx,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Serialize, Deserialize)]
pub enum AssetTypeCategory {
    Audio,
    Decal,
    Model,
}

impl AssetType {
    fn content_type(&self) -> &'static str {
        match self {
            Self::AudioMp3 => "audio/mpeg",
            Self::AudioOgg => "audio/ogg",
            Self::AudioFlac => "audio/flac",
            Self::AudioWav => "audio/wav",
            Self::DecalPng => "image/png",
            Self::DecalJpeg => "image/jpeg",
            Self::DecalBmp => "image/bmp",
            Self::DecalTga => "image/tga",
            Self::ModelFbx => "model/fbx",
        }
    }

    fn asset_type(&self) -> &'static str {
        match self {
            Self::AudioMp3 | Self::AudioOgg | Self::AudioFlac | Self::AudioWav => "Audio",
            Self::DecalPng | Self::DecalJpeg | Self::DecalBmp | Self::DecalTga => "Decal",
            Self::ModelFbx => "Model",
        }
    }

    pub fn try_from_extension(extension: &str) -> Result<Self, crate::rbx::error::Error> {
        match extension.to_lowercase().as_str() {
            "mp3" => Ok(Self::AudioMp3),
            "ogg" => Ok(Self::AudioOgg),
            "flac" => Ok(Self::AudioFlac),
            "wav" => Ok(Self::AudioWav),
            "png" => Ok(Self::DecalPng),
            "jpg" => Ok(Self::DecalJpeg),
            "jpeg" => Ok(Self::DecalJpeg),
            "bmp" => Ok(Self::DecalBmp),
            "tga" => Ok(Self::DecalTga),
            "fbx" => Ok(Self::ModelFbx),
            _ => Err(crate::rbx::error::Error::InferAssetTypeError(
                "Unknown extension".to_string(),
            )),
        }
    }
}

impl Serialize for AssetType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.asset_type())
    }
}

fn build_url(asset_id: Option<u64>) -> String {
    if let Some(asset_id) = asset_id {
        format!("https://apis.roblox.com/assets/v1/assets/{asset_id}")
    } else {
        "https://apis.roblox.com/assets/v1/assets".to_string()
    }
}

pub async fn create_asset(
    http: &HttpClient,
    params: &CreateAssetParams,
) -> Result<AssetOperation, Error> {
    let file_name = Path::new(&params.filepath)
        .file_name()
        .ok_or_else(|| Error::FileLoadError("Failed to parse file name from file path".into()))?
        .to_os_string()
        .into_string()
        .map_err(|err| {
            Error::FileLoadError(format!("Failed to convert file name into string: {err:?}"))
        })?;

    let asset_info = serde_json::to_string(&params.asset)?;
    let file = multipart::Part::bytes(fs::read(&params.filepath)?)
        .file_name(file_name)
        .mime_str(params.asset.asset_type.content_type())?;

    let form = multipart::Form::new()
        .text("request", asset_info)
        .part("fileContent", file);

    let url = build_url(None);
    let res = http.post(url).multipart(form).send().await?;
    http::json::<AssetOperation>(res).await
}

pub async fn create_asset_with_contents<'a>(
    http: &HttpClient,
    params: &CreateAssetParamsWithContents<'a>,
) -> Result<AssetOperation, Error> {
    let file = multipart::Part::bytes(params.contents.to_vec())
        .file_name(params.asset.display_name.clone())
        .mime_str(params.asset.asset_type.content_type())?;
    let asset_info = serde_json::to_string(&params.asset)?;

    let form = multipart::Form::new()
        .text("request", asset_info)
        .part("fileContent", file);

    let url = build_url(None);
    let res = http.post(url).multipart(form).send().await?;
    http::json::<AssetOperation>(res).await
}

pub async fn update_asset(
    http: &HttpClient,
    params: &UpdateAssetParams,
) -> Result<AssetOperation, Error> {
    let file_name = Path::new(&params.filepath)
        .file_name()
        .ok_or_else(|| Error::FileLoadError("Failed to parse file name from file path".into()))?
        .to_os_string()
        .into_string()
        .map_err(|err| {
            Error::FileLoadError(format!("Failed to convert file name into string: {err:?}"))
        })?;

    let file = multipart::Part::bytes(fs::read(&params.filepath)?)
        .file_name(file_name)
        .mime_str(params.asset_type.content_type())?;

    let request_json = json!({
        "assetId": params.asset_id,
    });
    let request = serde_json::to_string(&request_json)?;

    let form = multipart::Form::new()
        .text("request", request)
        .part("fileContent", file);

    let url = build_url(Some(params.asset_id));
    let res = http.patch(url).multipart(form).send().await?;
    http::json::<AssetOperation>(res).await
}

pub async fn get_operation(
    http: &HttpClient,
    params: &GetAssetOperationParams,
) -> Result<AssetGetOperation, Error> {
    let url = format!(
        "https://apis.roblox.com/assets/v1/operations/{operationId}",
        operationId = params.operation_id
    );
    let res = http.get(url).send().await?;
    http::json::<AssetGetOperation>(res).await
}

pub async fn get_asset(http: &HttpClient, params: &GetAssetParams) -> Result<AssetInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/assets/v1/assets/{assetId}",
        assetId = params.asset_id
    );
    let mut query: QueryString = vec![];
    if let Some(read_mask) = &params.read_mask {
        query.push(("readMask", read_mask.clone()));
    }
    let res = http.get(url).query(&query).send().await?;
    http::json::<AssetInfo>(res).await
}

pub async fn archive_asset(
    http: &HttpClient,
    params: &ArchiveAssetParams,
) -> Result<AssetInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/assets/v1/assets/{assetId}:archive",
        assetId = params.asset_id
    );
    let res = http
        .post(url)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    http::json::<AssetInfo>(res).await
}

pub async fn restore_asset(
    http: &HttpClient,
    params: &ArchiveAssetParams,
) -> Result<AssetInfo, Error> {
    let url = format!(
        "https://apis.roblox.com/assets/v1/assets/{assetId}:restore",
        assetId = params.asset_id
    );
    let res = http
        .post(url)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    http::json::<AssetInfo>(res).await
}

// ---------------------------------------------------------------- versions

/// A version of an asset.
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssetVersion {
    /// `assets/{asset_id}/versions/{version_number}`
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creation_context: Option<serde_json::Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub moderation_result: Option<serde_json::Value>,
    #[serde(default)]
    pub published: bool,
}

impl AssetVersion {
    /// The version number (last segment of `path`).
    pub fn version_number(&self) -> Option<u64> {
        self.path.rsplit('/').next()?.parse().ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListAssetVersionsResponse {
    #[serde(default)]
    pub asset_versions: Vec<AssetVersion>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// List an asset's versions, newest first.
pub async fn list_asset_versions(
    http: &HttpClient,
    asset_id: u64,
    max_page_size: Option<u32>,
    page_token: Option<String>,
) -> Result<ListAssetVersionsResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(n) = max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = page_token {
        query.push(("pageToken", token));
    }
    let res = http
        .get(format!("{}/versions", build_url(Some(asset_id))))
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get a specific version of an asset.
pub async fn get_asset_version(
    http: &HttpClient,
    asset_id: u64,
    version_number: u64,
) -> Result<AssetVersion, Error> {
    let res = http
        .get(format!(
            "{}/versions/{version_number}",
            build_url(Some(asset_id))
        ))
        .send()
        .await?;
    http::json(res).await
}

/// Roll an asset back to a previous version. This creates a new version with
/// the old content.
pub async fn rollback_asset_version(
    http: &HttpClient,
    asset_id: u64,
    version_number: u64,
) -> Result<AssetVersion, Error> {
    let res = http
        .post(format!("{}/versions:rollback", build_url(Some(asset_id))))
        .json(&json!({
            "assetVersion": format!("assets/{asset_id}/versions/{version_number}")
        }))
        .send()
        .await?;
    http::json(res).await
}

// ------------------------------------------------------------- permissions

/// Who is being granted access to assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PermissionSubject {
    User(u64),
    Group(u64),
    Universe(u64),
}

/// What the subject may do with the assets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum AssetPermissionAction {
    /// Use the asset in experiences (e.g. audio, animations, models).
    Use,
    /// Edit and manage the asset.
    Edit,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct GrantRequest {
    asset_id: u64,
    grant_to_dependencies: bool,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BatchGrantPermissionsRequest {
    subject_type: &'static str,
    subject_id: String,
    action: &'static str,
    requests: Vec<GrantRequest>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GrantPermissionError {
    pub asset_id: u64,
    pub code: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BatchGrantPermissionsResponse {
    #[serde(default)]
    pub success_asset_ids: Vec<u64>,
    #[serde(default)]
    pub errors: Vec<GrantPermissionError>,
}

fn grant_request(
    subject: PermissionSubject,
    action: AssetPermissionAction,
    asset_ids: &[u64],
    grant_to_dependencies: bool,
) -> BatchGrantPermissionsRequest {
    let (subject_type, subject_id) = match subject {
        PermissionSubject::User(id) => ("User", id),
        PermissionSubject::Group(id) => ("Group", id),
        PermissionSubject::Universe(id) => ("Universe", id),
    };
    BatchGrantPermissionsRequest {
        subject_type,
        subject_id: subject_id.to_string(),
        action: match action {
            AssetPermissionAction::Use => "Use",
            AssetPermissionAction::Edit => "Edit",
        },
        requests: asset_ids
            .iter()
            .map(|&asset_id| GrantRequest {
                asset_id,
                grant_to_dependencies,
            })
            .collect(),
    }
}

/// Grant a user, group, or universe permission to use or edit assets.
///
/// Assets that could not be granted are listed in the response's `errors`
/// rather than failing the whole request.
pub async fn grant_permissions(
    http: &HttpClient,
    subject: PermissionSubject,
    action: AssetPermissionAction,
    asset_ids: &[u64],
    grant_to_dependencies: bool,
) -> Result<BatchGrantPermissionsResponse, Error> {
    let body = grant_request(subject, action, asset_ids, grant_to_dependencies);
    let res = http
        .patch(http::url("/asset-permissions-api/v1/assets/permissions"))
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grant_body() {
        let body = grant_request(
            PermissionSubject::Universe(42),
            AssetPermissionAction::Use,
            &[1, 2],
            true,
        );
        assert_eq!(
            serde_json::to_value(body).unwrap(),
            json!({
                "subjectType": "Universe",
                "subjectId": "42",
                "action": "Use",
                "requests": [
                    {"assetId": 1, "grantToDependencies": true},
                    {"assetId": 2, "grantToDependencies": true}
                ]
            })
        );
    }

    #[test]
    fn decode_versions() {
        let res: ListAssetVersionsResponse = serde_json::from_value(json!({
            "assetVersions": [{
                "path": "assets/123/versions/7",
                "createTime": "2024-01-01T00:00:00Z",
                "moderationResult": {"moderationState": "Approved"},
                "published": true
            }]
        }))
        .unwrap();
        assert_eq!(res.asset_versions[0].version_number(), Some(7));
    }

    #[test]
    fn expected_price_omitted_when_none() {
        let ctx = AssetCreationContext {
            creator: AssetCreator::User(AssetUserCreator {
                user_id: "1".into(),
            }),
            expected_price: None,
        };
        assert_eq!(
            serde_json::to_value(ctx).unwrap(),
            json!({"creator": {"userId": "1"}})
        );
    }
}
