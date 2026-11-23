//! Place instances (Open Cloud v2 Instance API).
//!
//! Read the data model of a published place and edit script sources without
//! opening Studio. Every method returns a long-running [`Operation`]; the
//! high-level [`InstanceClient`](super::InstanceClient) waits for it.
//!
//! Use the instance ID `root` to start from the root of the data model.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::{PlaceId, UniverseId},
    util::{encode_path_segment as seg, QueryString},
};

use super::operation::Operation;

/// Instance ID of the root of a place's data model.
pub const ROOT: &str = "root";

/// Largest script source Roblox accepts in an update, in UTF-8 bytes.
pub const MAX_SOURCE_BYTES: usize = 200_000;

/// Properties of `Script` and `LocalScript` instances.
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct ScriptDetails {
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    /// `Legacy`, `Server`, `Client`, or `Plugin`.
    #[serde(
        rename = "RunContext",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_context: Option<String>,
    #[serde(rename = "Source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Properties of `ModuleScript` instances.
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct ModuleScriptDetails {
    #[serde(rename = "Source", default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

/// Class-specific properties. At most one field is set; instances of classes
/// the API does not expose details for have none set.
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct InstanceDetails {
    #[serde(rename = "Folder", default, skip_serializing_if = "Option::is_none")]
    pub folder: Option<Value>,
    #[serde(rename = "Script", default, skip_serializing_if = "Option::is_none")]
    pub script: Option<ScriptDetails>,
    #[serde(
        rename = "LocalScript",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_script: Option<ScriptDetails>,
    #[serde(
        rename = "ModuleScript",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub module_script: Option<ModuleScriptDetails>,
}

/// Script classes whose source can be read and updated.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScriptKind {
    Script,
    LocalScript,
    ModuleScript,
}

impl InstanceDetails {
    /// The class name, if the API exposes details for it.
    pub fn class_name(&self) -> Option<&'static str> {
        if self.script.is_some() {
            Some("Script")
        } else if self.local_script.is_some() {
            Some("LocalScript")
        } else if self.module_script.is_some() {
            Some("ModuleScript")
        } else if self.folder.is_some() {
            Some("Folder")
        } else {
            None
        }
    }

    /// The script kind, if this is a script.
    pub fn script_kind(&self) -> Option<ScriptKind> {
        if self.script.is_some() {
            Some(ScriptKind::Script)
        } else if self.local_script.is_some() {
            Some(ScriptKind::LocalScript)
        } else if self.module_script.is_some() {
            Some(ScriptKind::ModuleScript)
        } else {
            None
        }
    }

    /// The source code, if this is a script.
    pub fn source(&self) -> Option<&str> {
        self.script
            .as_ref()
            .or(self.local_script.as_ref())
            .and_then(|s| s.source.as_deref())
            .or_else(|| {
                self.module_script
                    .as_ref()
                    .and_then(|s| s.source.as_deref())
            })
    }

    /// Details that set only the `Source` of a script of the given kind.
    pub fn with_source(kind: ScriptKind, source: String) -> Self {
        match kind {
            ScriptKind::Script => Self {
                script: Some(ScriptDetails {
                    source: Some(source),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ScriptKind::LocalScript => Self {
                local_script: Some(ScriptDetails {
                    source: Some(source),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ScriptKind::ModuleScript => Self {
                module_script: Some(ModuleScriptDetails {
                    source: Some(source),
                }),
                ..Default::default()
            },
        }
    }
}

/// The engine-level data of an instance.
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct EngineInstance {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// ID of the parent instance.
    #[serde(rename = "Parent", default, skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Details", default, skip_serializing_if = "Option::is_none")]
    pub details: Option<InstanceDetails>,
}

/// An instance in a place's data model.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Instance {
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub has_children: bool,
    #[serde(default)]
    pub engine_instance: EngineInstance,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListInstanceChildrenResponse {
    #[serde(default)]
    pub instances: Vec<Instance>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Identifies an instance within a place.
#[derive(Debug, Clone)]
pub struct InstanceParams {
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    /// Instance ID, or [`ROOT`].
    pub instance_id: String,
}

impl InstanceParams {
    fn url(&self) -> String {
        http::url(&format!(
            "/cloud/v2/universes/{}/places/{}/instances/{}",
            self.universe_id,
            self.place_id,
            seg(&self.instance_id)
        ))
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UpdateInstanceBody<'a> {
    engine_instance: UpdateEngineInstance<'a>,
}

#[derive(Serialize)]
struct UpdateEngineInstance<'a> {
    #[serde(rename = "Details")]
    details: &'a InstanceDetails,
}

/// Start fetching an instance and its properties.
pub async fn get_instance(
    http: &HttpClient,
    params: &InstanceParams,
) -> Result<Operation<Instance>, Error> {
    let res = http.get(params.url()).send().await?;
    http::json(res).await
}

/// Start listing an instance's direct children.
pub async fn list_children(
    http: &HttpClient,
    params: &InstanceParams,
    max_page_size: Option<u32>,
    page_token: Option<String>,
) -> Result<Operation<ListInstanceChildrenResponse>, Error> {
    let mut query: QueryString = vec![];
    if let Some(n) = max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = page_token {
        query.push(("pageToken", token));
    }
    let url = format!("{}:listChildren", params.url());
    let res = http.get(url).query(&query).send().await?;
    http::json(res).await
}

/// Start updating an instance's properties.
///
/// `update_mask` optionally limits which fields are written.
pub async fn update_instance(
    http: &HttpClient,
    params: &InstanceParams,
    details: &InstanceDetails,
    update_mask: Option<String>,
) -> Result<Operation<Instance>, Error> {
    if let Some(source) = details.source() {
        if source.len() > MAX_SOURCE_BYTES {
            return Err(Error::EndpointError(format!(
                "script source is {} bytes; Roblox accepts at most {MAX_SOURCE_BYTES}",
                source.len()
            )));
        }
    }
    let mut query: QueryString = vec![];
    if let Some(mask) = update_mask {
        query.push(("updateMask", mask));
    }
    let body = UpdateInstanceBody {
        engine_instance: UpdateEngineInstance { details },
    };
    let res = http
        .patch(params.url())
        .query(&query)
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decode_children_operation() {
        let op: Operation<ListInstanceChildrenResponse> = serde_json::from_value(json!({
            "path": "universes/1/places/2/instances/root/operations/abc",
            "done": true,
            "response": {
                "@type": "type.googleapis.com/roblox.open_cloud.cloud.v2.ListInstanceChildrenResponse",
                "instances": [
                    {
                        "path": "universes/1/places/2/instances/w",
                        "hasChildren": true,
                        "engineInstance": {"Id": "w", "Parent": "root", "Name": "Workspace", "Details": {}}
                    },
                    {
                        "path": "universes/1/places/2/instances/s",
                        "engineInstance": {
                            "Id": "s", "Parent": "w", "Name": "Main",
                            "Details": {"Script": {"Enabled": true, "RunContext": "Server", "Source": "print(1)"}}
                        }
                    }
                ]
            }
        }))
        .unwrap();
        let children = op.into_result().unwrap().instances;
        let workspace = &children[0].engine_instance;
        assert_eq!(workspace.name.as_deref(), Some("Workspace"));
        assert_eq!(workspace.details.as_ref().unwrap().class_name(), None);
        let script = children[1].engine_instance.details.as_ref().unwrap();
        assert_eq!(script.script_kind(), Some(ScriptKind::Script));
        assert_eq!(script.source(), Some("print(1)"));
    }

    #[test]
    fn update_body_sets_only_source() {
        let details = InstanceDetails::with_source(ScriptKind::ModuleScript, "return {}".into());
        let body = UpdateInstanceBody {
            engine_instance: UpdateEngineInstance { details: &details },
        };
        assert_eq!(
            serde_json::to_value(body).unwrap(),
            json!({"engineInstance": {"Details": {"ModuleScript": {"Source": "return {}"}}}})
        );
    }
}
