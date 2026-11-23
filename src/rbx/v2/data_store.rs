//! Data stores (Open Cloud v2).
//!
//! Most usage should go through [`DataStoreClient`](super::DataStoreClient),
//! obtained from [`Client::data_store`](super::Client::data_store).
//!
//! Entry values are arbitrary JSON. Luau `buffer` values appear as tagged
//! objects; see [`crate::rbx::buffer`] for helpers to decode them.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::{RobloxUserId, UniverseId},
    util::{encode_path_segment as seg, QueryString},
};

#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DataStoreState {
    StateUnspecified,
    Active,
    Deleted,
    /// A state added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

/// A data store within a universe.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataStore {
    pub path: String,
    #[serde(default)]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// Set when the data store is scheduled for permanent deletion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DataStoreState>,
}

/// An entry (key) within a data store.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DataStoreEntry {
    pub path: String,
    #[serde(default)]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub revision_create_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: Option<DataStoreState>,
    /// Pass back on update to only apply the change if the entry is unchanged.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    /// The entry's value. Omitted by list operations.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    /// User resource paths (`users/{id}`) associated with the entry.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub users: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Map<String, Value>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListDataStoresResponse {
    #[serde(default)]
    pub data_stores: Vec<DataStore>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Response for listing entries, or listing the revisions of one entry.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListDataStoreEntriesResponse {
    #[serde(default)]
    pub data_store_entries: Vec<DataStoreEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SnapshotDataStoresResponse {
    #[serde(default)]
    pub new_snapshot_taken: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub latest_snapshot_time: Option<String>,
}

/// Identifies a data store, optionally narrowed to a scope.
#[derive(Debug, Clone)]
pub struct DataStoreLocation {
    pub universe_id: UniverseId,
    pub data_store_id: String,
    /// The scope to operate in. When `None`, the scope-less endpoints are
    /// used, which operate on the default (`global`) scope.
    pub scope: Option<String>,
}

impl DataStoreLocation {
    fn data_store_url(&self) -> String {
        http::url(&format!(
            "/cloud/v2/universes/{}/data-stores/{}",
            self.universe_id,
            seg(&self.data_store_id)
        ))
    }

    fn entries_url(&self) -> String {
        match &self.scope {
            Some(scope) => format!("{}/scopes/{}/entries", self.data_store_url(), seg(scope)),
            None => format!("{}/entries", self.data_store_url()),
        }
    }

    fn entry_url(&self, entry_id: &str) -> String {
        format!("{}/{}", self.entries_url(), seg(entry_id))
    }
}

pub struct ListDataStoresParams {
    pub universe_id: UniverseId,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    /// Only `id.startsWith("prefix")` is supported.
    pub filter: Option<String>,
    pub show_deleted: Option<bool>,
}

pub struct ListEntriesParams {
    pub location: DataStoreLocation,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    /// Only `id.startsWith("prefix")` is supported.
    pub filter: Option<String>,
    pub show_deleted: Option<bool>,
}

pub struct EntryParams {
    pub location: DataStoreLocation,
    /// The entry key. To read a specific revision, use `key@revision_id`.
    pub entry_id: String,
}

pub struct CreateEntryParams {
    pub location: DataStoreLocation,
    pub entry_id: String,
    pub value: Value,
    pub users: Option<Vec<RobloxUserId>>,
    pub attributes: Option<Map<String, Value>>,
}

pub struct UpdateEntryParams {
    pub location: DataStoreLocation,
    pub entry_id: String,
    pub value: Value,
    /// Only apply the update if the entry's current etag matches.
    pub etag: Option<String>,
    pub users: Option<Vec<RobloxUserId>>,
    pub attributes: Option<Map<String, Value>>,
    /// Create the entry if it does not exist.
    pub allow_missing: Option<bool>,
}

pub struct IncrementEntryParams {
    pub location: DataStoreLocation,
    pub entry_id: String,
    pub amount: f64,
    pub users: Option<Vec<RobloxUserId>>,
    pub attributes: Option<Map<String, Value>>,
}

pub struct ListEntryRevisionsParams {
    pub location: DataStoreLocation,
    pub entry_id: String,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    /// e.g. `revision_create_time >= 2024-01-01T00:00:00Z`
    pub filter: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct EntryBody<'a> {
    value: &'a Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<&'a Map<String, Value>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IncrementBody<'a> {
    amount: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    users: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    attributes: Option<&'a Map<String, Value>>,
}

fn user_paths(users: &Option<Vec<RobloxUserId>>) -> Option<Vec<String>> {
    users
        .as_ref()
        .map(|ids| ids.iter().map(|id| format!("users/{id}")).collect())
}

fn page_query(
    max_page_size: Option<u32>,
    page_token: &Option<String>,
    filter: &Option<String>,
    show_deleted: Option<bool>,
) -> QueryString {
    let mut query: QueryString = vec![];
    if let Some(n) = max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = page_token {
        query.push(("pageToken", token.clone()));
    }
    if let Some(filter) = filter {
        query.push(("filter", filter.clone()));
    }
    if let Some(show_deleted) = show_deleted {
        query.push(("showDeleted", show_deleted.to_string()));
    }
    query
}

/// List the data stores in a universe.
pub async fn list_data_stores(
    http: &HttpClient,
    params: &ListDataStoresParams,
) -> Result<ListDataStoresResponse, Error> {
    let url = http::url(&format!(
        "/cloud/v2/universes/{}/data-stores",
        params.universe_id
    ));
    let query = page_query(
        params.max_page_size,
        &params.page_token,
        &params.filter,
        params.show_deleted,
    );
    let res = http.get(url).query(&query).send().await?;
    http::json(res).await
}

/// Schedule a data store for deletion. It can be restored with
/// [`undelete_data_store`] until its `expire_time`.
pub async fn delete_data_store(
    http: &HttpClient,
    universe_id: UniverseId,
    data_store_id: &str,
) -> Result<DataStore, Error> {
    let location = DataStoreLocation {
        universe_id,
        data_store_id: data_store_id.to_string(),
        scope: None,
    };
    let res = http.delete(location.data_store_url()).send().await?;
    http::json(res).await
}

/// Restore a data store that was scheduled for deletion.
pub async fn undelete_data_store(
    http: &HttpClient,
    universe_id: UniverseId,
    data_store_id: &str,
) -> Result<DataStore, Error> {
    let location = DataStoreLocation {
        universe_id,
        data_store_id: data_store_id.to_string(),
        scope: None,
    };
    let url = format!("{}:undelete", location.data_store_url());
    let res = http.post(url).json(&Map::new()).send().await?;
    http::json(res).await
}

/// Take a snapshot of every data store in the universe. At most one snapshot
/// is taken per UTC day; `new_snapshot_taken` reports whether this call
/// created one.
pub async fn snapshot_data_stores(
    http: &HttpClient,
    universe_id: UniverseId,
) -> Result<SnapshotDataStoresResponse, Error> {
    let url = http::url(&format!(
        "/cloud/v2/universes/{universe_id}/data-stores:snapshot"
    ));
    let res = http.post(url).json(&Map::new()).send().await?;
    http::json(res).await
}

/// List the entries in a data store.
pub async fn list_entries(
    http: &HttpClient,
    params: &ListEntriesParams,
) -> Result<ListDataStoreEntriesResponse, Error> {
    let query = page_query(
        params.max_page_size,
        &params.page_token,
        &params.filter,
        params.show_deleted,
    );
    let res = http
        .get(params.location.entries_url())
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get an entry, including its value.
pub async fn get_entry(http: &HttpClient, params: &EntryParams) -> Result<DataStoreEntry, Error> {
    let res = http
        .get(params.location.entry_url(&params.entry_id))
        .send()
        .await?;
    http::json(res).await
}

/// Create an entry. Fails if the entry already exists.
pub async fn create_entry(
    http: &HttpClient,
    params: &CreateEntryParams,
) -> Result<DataStoreEntry, Error> {
    let body = EntryBody {
        value: &params.value,
        etag: None,
        users: user_paths(&params.users),
        attributes: params.attributes.as_ref(),
    };
    let res = http
        .post(params.location.entries_url())
        .query(&[("id", &params.entry_id)])
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

/// Update an entry's value, users, and attributes.
pub async fn update_entry(
    http: &HttpClient,
    params: &UpdateEntryParams,
) -> Result<DataStoreEntry, Error> {
    let body = EntryBody {
        value: &params.value,
        etag: params.etag.as_deref(),
        users: user_paths(&params.users),
        attributes: params.attributes.as_ref(),
    };
    let mut query: QueryString = vec![];
    if let Some(allow_missing) = params.allow_missing {
        query.push(("allowMissing", allow_missing.to_string()));
    }
    let res = http
        .patch(params.location.entry_url(&params.entry_id))
        .query(&query)
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

/// Increment a numeric entry, creating it if it does not exist.
pub async fn increment_entry(
    http: &HttpClient,
    params: &IncrementEntryParams,
) -> Result<DataStoreEntry, Error> {
    let body = IncrementBody {
        amount: params.amount,
        users: user_paths(&params.users),
        attributes: params.attributes.as_ref(),
    };
    let url = format!("{}:increment", params.location.entry_url(&params.entry_id));
    let res = http.post(url).json(&body).send().await?;
    http::json(res).await
}

/// Mark an entry as deleted.
pub async fn delete_entry(http: &HttpClient, params: &EntryParams) -> Result<(), Error> {
    let res = http
        .delete(params.location.entry_url(&params.entry_id))
        .send()
        .await?;
    http::empty(res).await
}

/// List the revisions of an entry. Each returned entry's `revision_id` can be
/// read with [`get_entry`] using an `entry_id` of `key@revision_id`.
pub async fn list_entry_revisions(
    http: &HttpClient,
    params: &ListEntryRevisionsParams,
) -> Result<ListDataStoreEntriesResponse, Error> {
    let query = page_query(
        params.max_page_size,
        &params.page_token,
        &params.filter,
        None,
    );
    let url = format!(
        "{}:listRevisions",
        params.location.entry_url(&params.entry_id)
    );
    let res = http.get(url).query(&query).send().await?;
    http::json(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn loc(scope: Option<&str>) -> DataStoreLocation {
        DataStoreLocation {
            universe_id: UniverseId(42),
            data_store_id: "Player Data".to_string(),
            scope: scope.map(str::to_string),
        }
    }

    #[test]
    fn urls() {
        assert_eq!(
            loc(None).entry_url("user/1"),
            "https://apis.roblox.com/cloud/v2/universes/42/data-stores/Player%20Data/entries/user%2F1"
        );
        assert_eq!(
            loc(Some("global")).entries_url(),
            "https://apis.roblox.com/cloud/v2/universes/42/data-stores/Player%20Data/scopes/global/entries"
        );
    }

    #[test]
    fn entry_body_omits_unset_fields() {
        let value = json!({"coins": 5});
        let body = EntryBody {
            value: &value,
            etag: None,
            users: user_paths(&Some(vec![RobloxUserId(7)])),
            attributes: None,
        };
        assert_eq!(
            serde_json::to_value(&body).unwrap(),
            json!({"value": {"coins": 5}, "users": ["users/7"]})
        );
    }

    #[test]
    fn decode_entry() {
        let entry: DataStoreEntry = serde_json::from_value(json!({
            "path": "universes/42/data-stores/PlayerData/entries/user_1",
            "createTime": "2024-01-01T00:00:00Z",
            "revisionId": "08DC",
            "revisionCreateTime": "2024-01-02T00:00:00Z",
            "state": "ACTIVE",
            "etag": "08DC",
            "value": {"coins": 5},
            "id": "user_1",
            "users": ["users/1"],
            "attributes": {"a": 1}
        }))
        .unwrap();
        assert_eq!(entry.id, "user_1");
        assert_eq!(entry.state, Some(DataStoreState::Active));
        assert_eq!(entry.value, Some(json!({"coins": 5})));
    }

    #[test]
    fn decode_list_without_token() {
        let res: ListDataStoresResponse = serde_json::from_value(json!({
            "dataStores": [{"path": "universes/42/data-stores/A", "id": "A", "state": "SOMETHING_NEW"}]
        }))
        .unwrap();
        assert_eq!(res.data_stores[0].state, Some(DataStoreState::Unknown));
        assert!(res.next_page_token.is_none());
    }
}
