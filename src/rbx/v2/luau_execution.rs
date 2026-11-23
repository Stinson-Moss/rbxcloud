use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rbx::{
    error::Error,
    types::{PlaceId, UniverseId},
    util::QueryString,
};

use crate::rbx::http::{self, HttpClient};

#[derive(Debug)]
pub struct CreateLuauExecutionTaskParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
    pub script: String,
    pub timeout: Option<String>,
}

/// The task returned when creating a Luau execution task. It has the same
/// shape as a task fetched later with `get_luau_execution_task`.
pub type NewLuauExecutionSessionTask = LuauExecutionSessionTask;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLuauExecutionSessionTaskParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
    pub session_id: String,
    pub task_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GetLuauExecutionSessionTaskLogsParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
    pub session_id: String,
    pub task_id: String,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub view: LuauExecutionTaskLogView,
}

#[derive(Debug, Clone, Copy, clap::ValueEnum, Deserialize, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuauExecutionTaskLogView {
    Flat,
    Structured,
}

impl std::fmt::Display for LuauExecutionTaskLogView {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::Flat => "FLAT",
                Self::Structured => "STRUCTURED",
            }
        )
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTaskLogPage {
    #[serde(default)]
    pub luau_execution_session_task_logs: Vec<LuauExecutionSessionTaskLog>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTaskLog {
    pub path: String,
    #[serde(default)]
    pub messages: Vec<String>,
    #[serde(default)]
    pub structured_messages: Vec<StructuredMessage>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StructuredMessage {
    pub message: String,
    pub create_time: String,
    pub message_type: String,
}

/// A Luau execution session task.
///
/// Exactly one of `output` (when `state` is `Complete`) or `error` (when
/// `state` is `Failed`) is populated once the task finishes. Use
/// [`LuauExecutionSessionTask::outcome`] to match on the result.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTask {
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
    pub state: LuauExecutionState,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub script: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<LuauExecutionError>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub output: Option<LuauExecutionOutput>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_input: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enable_binary_output: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub binary_output_uri: Option<String>,
}

/// The result of a Luau execution task, derived from its state.
#[derive(Debug)]
pub enum LuauExecutionOutcome<'a> {
    /// The task is queued or still processing.
    Pending,
    /// The task finished; contains the values returned by the script.
    Complete(&'a [Value]),
    /// The task failed; contains the error reported by Roblox.
    Failed(&'a LuauExecutionError),
    /// The task was cancelled.
    Cancelled,
}

impl LuauExecutionSessionTask {
    /// Get the outcome of the task.
    pub fn outcome(&self) -> LuauExecutionOutcome<'_> {
        match (&self.state, &self.output, &self.error) {
            (LuauExecutionState::Cancelled, _, _) => LuauExecutionOutcome::Cancelled,
            (_, _, Some(error)) => LuauExecutionOutcome::Failed(error),
            (LuauExecutionState::Complete, Some(output), _) => {
                LuauExecutionOutcome::Complete(&output.results)
            }
            (LuauExecutionState::Complete, None, _) => LuauExecutionOutcome::Complete(&[]),
            _ => LuauExecutionOutcome::Pending,
        }
    }

    /// Whether the task has finished (completed, failed, or cancelled).
    pub fn is_done(&self) -> bool {
        !matches!(self.outcome(), LuauExecutionOutcome::Pending)
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionOutput {
    #[serde(default)]
    pub results: Vec<Value>,
}

/// Error reported by a failed Luau execution task.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionError {
    pub code: LuauExecutionErrorCode,
    #[serde(default)]
    pub message: String,
}

impl std::fmt::Display for LuauExecutionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}: {}", self.code, self.message)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuauExecutionErrorCode {
    ErrorCodeUnspecified,
    ScriptError,
    DeadlineExceeded,
    OutputSizeLimitExceeded,
    InternalError,
    /// A code added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum LuauExecutionState {
    StateUnspecified,
    Queued,
    Processing,
    Cancelled,
    Complete,
    Failed,
    /// A state added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LuauExecutionInput {
    pub script: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<String>,
}

pub async fn create_luau_execution_task(
    http: &HttpClient,
    params: &CreateLuauExecutionTaskParams,
) -> Result<NewLuauExecutionSessionTask, Error> {
    let url = if let Some(version_id) = &params.version_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/versions/{versionId}/luau-execution-session-tasks",
			universeId = params.universe_id,
			placeId = params.place_id,
			versionId = version_id,
		)
    } else {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/luau-execution-session-tasks",
			universeId = params.universe_id,
			placeId = params.place_id,
		)
    };

    let input = LuauExecutionInput {
        script: params.script.clone(),
        timeout: params.timeout.clone(),
    };

    let req_body = serde_json::to_string(&input)?;

    let res = http
        .post(url)
        .header("Content-Type", "application/json")
        .body(req_body)
        .send()
        .await?;

    http::json::<NewLuauExecutionSessionTask>(res).await
}

pub async fn get_luau_execution_task(
    http: &HttpClient,
    params: &GetLuauExecutionSessionTaskParams,
) -> Result<LuauExecutionSessionTask, Error> {
    let url = if let Some(version_id) = &params.version_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/versions/{versionId}/luau-execution-sessions/{sessionId}/tasks/{taskId}",
			universeId = params.universe_id,
			placeId = params.place_id,
			versionId = version_id,
			sessionId = params.session_id,
			taskId = params.task_id,
		)
    } else {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/luau-execution-sessions/{sessionId}/tasks/{taskId}",
			universeId = params.universe_id,
			placeId = params.place_id,
			sessionId = params.session_id,
			taskId = params.task_id,
		)
    };

    let res = http.get(url).send().await?;

    http::json::<LuauExecutionSessionTask>(res).await
}

pub async fn get_luau_execution_task_logs(
    http: &HttpClient,
    params: &GetLuauExecutionSessionTaskLogsParams,
) -> Result<LuauExecutionSessionTaskLogPage, Error> {
    let url = if let Some(version_id) = &params.version_id {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/versions/{versionId}/luau-execution-sessions/{sessionId}/tasks/{taskId}/logs",
			universeId = params.universe_id,
			placeId = params.place_id,
			versionId = version_id,
			sessionId = params.session_id,
			taskId = params.task_id,
		)
    } else {
        format!(
			"https://apis.roblox.com/cloud/v2/universes/{universeId}/places/{placeId}/luau-execution-sessions/{sessionId}/tasks/{taskId}/logs",
			universeId = params.universe_id,
			placeId = params.place_id,
			sessionId = params.session_id,
			taskId = params.task_id,
		)
    };

    let mut query: QueryString = vec![("view", params.view.to_string())];
    if let Some(max_page_size) = params.max_page_size {
        query.push(("maxPageSize", format!("{max_page_size}")));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.clone()));
    }

    let res = http.get(url).query(&query).send().await?;

    http::json::<LuauExecutionSessionTaskLogPage>(res).await
}
