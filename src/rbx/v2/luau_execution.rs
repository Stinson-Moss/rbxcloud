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
    /// Path of an uploaded binary input (see [`upload_binary_input`]). The
    /// script receives it as `({...})[1].BinaryInput`.
    pub binary_input: Option<String>,
    /// Allow the script to return a large `BinaryOutput` buffer.
    pub enable_binary_output: bool,
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub binary_input: Option<String>,
    #[serde(skip_serializing_if = "std::ops::Not::not")]
    pub enable_binary_output: bool,
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
        binary_input: params.binary_input.clone(),
        enable_binary_output: params.enable_binary_output,
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

/// Fetch a task by the `path` Roblox returned for it. Works whether or not
/// the task was created against a specific place version.
pub async fn get_luau_execution_task_by_path(
    http: &HttpClient,
    path: &str,
) -> Result<LuauExecutionSessionTask, Error> {
    let path = path.trim_start_matches('/');
    let res = http
        .get(http::url(&format!("/cloud/v2/{path}")))
        .send()
        .await?;
    http::json(res).await
}

/// Poll a task until it completes, fails, or is cancelled.
pub async fn wait_for_luau_execution_task(
    http: &HttpClient,
    task: LuauExecutionSessionTask,
    poll: super::operation::PollOptions,
) -> Result<LuauExecutionSessionTask, Error> {
    let mut task = task;
    let mut attempts = 0;
    while !task.is_done() {
        if attempts >= poll.max_attempts {
            return Err(Error::OperationError {
                code: 0,
                msg: format!(
                    "timed out waiting for Luau task {} after {attempts} polls",
                    task.path
                ),
            });
        }
        tokio::time::sleep(poll.interval).await;
        task = get_luau_execution_task_by_path(http, &task.path).await?;
        attempts += 1;
    }
    Ok(task)
}

// ------------------------------------------------------------ binary input

/// Largest binary input Roblox accepts (100 MiB).
pub const MAX_BINARY_INPUT_BYTES: usize = 100 * 1024 * 1024;

/// A binary input slot. Valid for 15 minutes after creation, and reusable
/// across tasks in the same universe during that time.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LuauExecutionSessionTaskBinaryInput {
    pub path: String,
    #[serde(default)]
    pub size: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub upload_uri: Option<String>,
}

/// Create a binary input slot of `size` bytes.
pub async fn create_binary_input(
    http: &HttpClient,
    universe_id: UniverseId,
    size: u64,
) -> Result<LuauExecutionSessionTaskBinaryInput, Error> {
    let url = http::url(&format!(
        "/cloud/v2/universes/{universe_id}/luau-execution-session-task-binary-inputs"
    ));
    let res = http
        .post(url)
        .json(&serde_json::json!({ "size": size }))
        .send()
        .await?;
    http::json(res).await
}

/// Create a binary input and upload `data` to it, returning the input whose
/// `path` can be passed as `binary_input` when creating a task.
pub async fn upload_binary_input(
    http: &HttpClient,
    universe_id: UniverseId,
    data: Vec<u8>,
) -> Result<LuauExecutionSessionTaskBinaryInput, Error> {
    if data.len() > MAX_BINARY_INPUT_BYTES {
        return Err(Error::EndpointError(format!(
            "binary input is {} bytes; Roblox accepts at most {MAX_BINARY_INPUT_BYTES}",
            data.len()
        )));
    }
    let input = create_binary_input(http, universe_id, data.len() as u64).await?;
    let upload_uri = input.upload_uri.clone().ok_or_else(|| {
        Error::EndpointError("binary input response has no uploadUri".to_string())
    })?;
    // The upload URI is pre-signed: send it without the API key.
    let res = http
        .reqwest_client()
        .put(upload_uri)
        .body(data)
        .send()
        .await?;
    http::empty(res).await?;
    Ok(input)
}

/// Download a task's binary output from its `binary_output_uri`.
pub async fn download_binary_output(http: &HttpClient, uri: &str) -> Result<Vec<u8>, Error> {
    // Pre-signed URI: no API key.
    let res = http.reqwest_client().get(uri).send().await?;
    let res = http::check(res).await?;
    Ok(res.bytes().await?.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decode_failed_task() {
        // Issue #78: failed tasks have `error` and no `output`.
        let task: LuauExecutionSessionTask = serde_json::from_value(json!({
            "path": "universes/1/places/2/luau-execution-sessions/s/tasks/t",
            "createTime": "2024-01-01T00:00:00Z",
            "updateTime": "2024-01-01T00:00:01Z",
            "user": "users/1",
            "state": "FAILED",
            "script": "error('boom')",
            "error": {"code": "SCRIPT_ERROR", "message": "boom"}
        }))
        .unwrap();
        match task.outcome() {
            LuauExecutionOutcome::Failed(err) => {
                assert_eq!(err.code, LuauExecutionErrorCode::ScriptError);
                assert_eq!(err.message, "boom");
            }
            other => panic!("unexpected outcome: {other:?}"),
        }
    }

    #[test]
    fn decode_complete_and_pending_tasks() {
        let done: LuauExecutionSessionTask = serde_json::from_value(json!({
            "path": "p", "state": "COMPLETE", "output": {"results": [7]}
        }))
        .unwrap();
        assert!(matches!(done.outcome(), LuauExecutionOutcome::Complete(r) if r == [json!(7)]));
        let queued: LuauExecutionSessionTask =
            serde_json::from_value(json!({"path": "p", "state": "QUEUED"})).unwrap();
        assert!(!queued.is_done());
    }

    #[test]
    fn input_body_omits_defaults() {
        let input = LuauExecutionInput {
            script: "return 1".into(),
            timeout: None,
            binary_input: Some("universes/1/luau-execution-session-task-binary-inputs/x".into()),
            enable_binary_output: false,
        };
        assert_eq!(
            serde_json::to_value(input).unwrap(),
            json!({
                "script": "return 1",
                "binaryInput": "universes/1/luau-execution-session-task-binary-inputs/x"
            })
        );
    }
}
