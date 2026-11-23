//! Memory stores (Open Cloud v2): queues, sorted maps, and flushing.
//!
//! Most usage should go through [`MemoryStoreClient`](super::MemoryStoreClient),
//! obtained from [`Client::memory_store`](super::Client::memory_store).
//!
//! Durations (`ttl`, `invisibility_window`) are passed as whole seconds and
//! sent to Roblox in its `"<n>s"` duration format.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::UniverseId,
    util::{encode_path_segment as seg, QueryString},
};

/// Maximum TTL Roblox accepts for memory store items (45 days).
pub const MAX_TTL_SECONDS: u64 = 3_888_000;

fn duration(seconds: u64) -> String {
    format!("{seconds}s")
}

fn base_url(universe_id: UniverseId) -> String {
    http::url(&format!("/cloud/v2/universes/{universe_id}/memory-store"))
}

// ---------------------------------------------------------------- queues

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MemoryStoreQueueItem {
    #[serde(default)]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub priority: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<String>,
}

/// Items read from a queue. Pass `id` to [`discard_queue_items`] to remove
/// them; otherwise they reappear once the invisibility window ends.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ReadMemoryStoreQueueItemsResponse {
    #[serde(default)]
    pub queue_items: Vec<MemoryStoreQueueItem>,
    /// The read ID.
    #[serde(default)]
    pub id: String,
}

pub struct CreateQueueItemParams {
    pub universe_id: UniverseId,
    pub queue_id: String,
    pub data: Value,
    /// Higher priority items are read first.
    pub priority: Option<f64>,
    pub ttl_seconds: Option<u64>,
}

pub struct ReadQueueItemsParams {
    pub universe_id: UniverseId,
    pub queue_id: String,
    /// Number of items to read (1-100). Defaults to 1.
    pub count: Option<u32>,
    /// Fail with 404 unless `count` items are available.
    pub all_or_nothing: Option<bool>,
    /// How long read items stay hidden from other readers.
    pub invisibility_window_seconds: Option<u64>,
}

pub struct DiscardQueueItemsParams {
    pub universe_id: UniverseId,
    pub queue_id: String,
    /// The `id` from [`ReadMemoryStoreQueueItemsResponse`].
    pub read_id: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct CreateQueueItemBody<'a> {
    data: &'a Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    priority: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ttl: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct DiscardBody<'a> {
    read_id: &'a str,
}

fn queue_url(universe_id: UniverseId, queue_id: &str) -> String {
    format!("{}/queues/{}/items", base_url(universe_id), seg(queue_id))
}

/// Add an item to a queue.
pub async fn create_queue_item(
    http: &HttpClient,
    params: &CreateQueueItemParams,
) -> Result<MemoryStoreQueueItem, Error> {
    let body = CreateQueueItemBody {
        data: &params.data,
        priority: params.priority,
        ttl: params.ttl_seconds.map(duration),
    };
    let res = http
        .post(queue_url(params.universe_id, &params.queue_id))
        .json(&body)
        .send()
        .await?;
    http::json(res).await
}

/// Read items from a queue, hiding them from other readers for the
/// invisibility window.
pub async fn read_queue_items(
    http: &HttpClient,
    params: &ReadQueueItemsParams,
) -> Result<ReadMemoryStoreQueueItemsResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(count) = params.count {
        query.push(("count", count.to_string()));
    }
    if let Some(all_or_nothing) = params.all_or_nothing {
        query.push(("allOrNothing", all_or_nothing.to_string()));
    }
    if let Some(window) = params.invisibility_window_seconds {
        query.push(("invisibilityWindow", duration(window)));
    }
    let url = format!("{}:read", queue_url(params.universe_id, &params.queue_id));
    let res = http.get(url).query(&query).send().await?;
    http::json(res).await
}

/// Permanently remove items returned by a previous read.
pub async fn discard_queue_items(
    http: &HttpClient,
    params: &DiscardQueueItemsParams,
) -> Result<(), Error> {
    let url = format!(
        "{}:discard",
        queue_url(params.universe_id, &params.queue_id)
    );
    let res = http
        .post(url)
        .json(&DiscardBody {
            read_id: &params.read_id,
        })
        .send()
        .await?;
    http::empty(res).await
}

// ----------------------------------------------------------- sorted maps

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MemoryStoreSortedMapItem {
    #[serde(default)]
    pub path: String,
    /// The item key.
    #[serde(default)]
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub etag: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expire_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string_sort_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub numeric_sort_key: Option<f64>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListMemoryStoreSortedMapItemsResponse {
    #[serde(default)]
    pub items: Vec<MemoryStoreSortedMapItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Optional sort key for a sorted map item. Items are ordered by sort key
/// first, then by item ID.
#[derive(Debug, Clone, PartialEq)]
pub enum SortKey {
    String(String),
    Numeric(f64),
}

/// Sort direction when listing sorted map items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum SortedMapOrder {
    Asc,
    Desc,
}

pub struct ListSortedMapItemsParams {
    pub universe_id: UniverseId,
    pub sorted_map_id: String,
    /// 1-200. Roblox returns a single item when unset.
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub order: Option<SortedMapOrder>,
    /// CEL filter on `id` and `sortKey` using `<`, `>`, `&&`.
    pub filter: Option<String>,
}

pub struct SortedMapItemParams {
    pub universe_id: UniverseId,
    pub sorted_map_id: String,
    pub item_id: String,
}

pub struct SetSortedMapItemParams {
    pub universe_id: UniverseId,
    pub sorted_map_id: String,
    pub item_id: String,
    pub value: Value,
    /// Required by Roblox; at most [`MAX_TTL_SECONDS`].
    pub ttl_seconds: u64,
    pub sort_key: Option<SortKey>,
    /// On update, only apply the change if the item's etag matches.
    pub etag: Option<String>,
    /// On update, create the item if it does not exist.
    pub allow_missing: Option<bool>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SortedMapItemBody<'a> {
    value: &'a Value,
    ttl: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    etag: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    string_sort_key: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    numeric_sort_key: Option<f64>,
}

impl SetSortedMapItemParams {
    fn body(&self) -> SortedMapItemBody<'_> {
        let (string_sort_key, numeric_sort_key) = match &self.sort_key {
            Some(SortKey::String(s)) => (Some(s.as_str()), None),
            Some(SortKey::Numeric(n)) => (None, Some(*n)),
            None => (None, None),
        };
        SortedMapItemBody {
            value: &self.value,
            ttl: duration(self.ttl_seconds),
            etag: self.etag.as_deref(),
            string_sort_key,
            numeric_sort_key,
        }
    }
}

fn sorted_map_url(universe_id: UniverseId, sorted_map_id: &str) -> String {
    format!(
        "{}/sorted-maps/{}/items",
        base_url(universe_id),
        seg(sorted_map_id)
    )
}

fn sorted_map_item_url(universe_id: UniverseId, sorted_map_id: &str, item_id: &str) -> String {
    format!(
        "{}/{}",
        sorted_map_url(universe_id, sorted_map_id),
        seg(item_id)
    )
}

/// List items in a sorted map.
pub async fn list_sorted_map_items(
    http: &HttpClient,
    params: &ListSortedMapItemsParams,
) -> Result<ListMemoryStoreSortedMapItemsResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(n) = params.max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = &params.page_token {
        query.push(("pageToken", token.clone()));
    }
    if let Some(order) = params.order {
        let order_by = match order {
            SortedMapOrder::Asc => "asc",
            SortedMapOrder::Desc => "desc",
        };
        query.push(("orderBy", order_by.to_string()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.clone()));
    }
    let res = http
        .get(sorted_map_url(params.universe_id, &params.sorted_map_id))
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get a sorted map item.
pub async fn get_sorted_map_item(
    http: &HttpClient,
    params: &SortedMapItemParams,
) -> Result<MemoryStoreSortedMapItem, Error> {
    let url = sorted_map_item_url(params.universe_id, &params.sorted_map_id, &params.item_id);
    let res = http.get(url).send().await?;
    http::json(res).await
}

/// Create a sorted map item. Fails if it already exists.
pub async fn create_sorted_map_item(
    http: &HttpClient,
    params: &SetSortedMapItemParams,
) -> Result<MemoryStoreSortedMapItem, Error> {
    let res = http
        .post(sorted_map_url(params.universe_id, &params.sorted_map_id))
        .query(&[("id", &params.item_id)])
        .json(&params.body())
        .send()
        .await?;
    http::json(res).await
}

/// Update a sorted map item.
pub async fn update_sorted_map_item(
    http: &HttpClient,
    params: &SetSortedMapItemParams,
) -> Result<MemoryStoreSortedMapItem, Error> {
    let mut query: QueryString = vec![];
    if let Some(allow_missing) = params.allow_missing {
        query.push(("allowMissing", allow_missing.to_string()));
    }
    let url = sorted_map_item_url(params.universe_id, &params.sorted_map_id, &params.item_id);
    let res = http
        .patch(url)
        .query(&query)
        .json(&params.body())
        .send()
        .await?;
    http::json(res).await
}

/// Delete a sorted map item.
pub async fn delete_sorted_map_item(
    http: &HttpClient,
    params: &SortedMapItemParams,
) -> Result<(), Error> {
    let url = sorted_map_item_url(params.universe_id, &params.sorted_map_id, &params.item_id);
    let res = http.delete(url).send().await?;
    http::empty(res).await
}

// ----------------------------------------------------------------- flush

/// Which memory store scope to flush.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, clap::ValueEnum)]
pub enum MemoryStoreScope {
    /// Live game servers (the default).
    #[default]
    Live,
    /// Studio test sessions.
    Test,
}

impl MemoryStoreScope {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Live => "LIVE",
            Self::Test => "TEST",
        }
    }
}

/// A long-running flush operation.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MemoryStoreFlushOperation {
    /// Operation path. Note that, unlike other APIs, it already includes the
    /// `cloud/v2/` prefix.
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub done: bool,
}

impl MemoryStoreFlushOperation {
    /// The operation ID (last segment of `path`).
    pub fn operation_id(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or_default()
    }
}

/// Asynchronously delete every queue and sorted map in the universe. Poll the
/// returned operation with [`get_flush_operation`] until `done`.
pub async fn flush(
    http: &HttpClient,
    universe_id: UniverseId,
    scope: Option<MemoryStoreScope>,
) -> Result<MemoryStoreFlushOperation, Error> {
    let mut query: QueryString = vec![];
    if let Some(scope) = scope {
        query.push(("scope", scope.as_str().to_string()));
    }
    let url = format!("{}:flush", base_url(universe_id));
    let res = http
        .post(url)
        .query(&query)
        .json(&serde_json::Map::new())
        .send()
        .await?;
    http::json(res).await
}

/// Check the status of a flush operation.
pub async fn get_flush_operation(
    http: &HttpClient,
    universe_id: UniverseId,
    operation_id: &str,
    scope: Option<MemoryStoreScope>,
) -> Result<MemoryStoreFlushOperation, Error> {
    let mut query: QueryString = vec![];
    if let Some(scope) = scope {
        query.push(("scope", scope.as_str().to_string()));
    }
    let url = format!("{}/operations/{}", base_url(universe_id), seg(operation_id));
    let res = http.get(url).query(&query).send().await?;
    http::json(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn sorted_map_body() {
        let params = SetSortedMapItemParams {
            universe_id: UniverseId(1),
            sorted_map_id: "Lobby".into(),
            item_id: "match-1".into(),
            value: json!({"players": 4}),
            ttl_seconds: 60,
            sort_key: Some(SortKey::Numeric(4.0)),
            etag: None,
            allow_missing: None,
        };
        assert_eq!(
            serde_json::to_value(params.body()).unwrap(),
            json!({"value": {"players": 4}, "ttl": "60s", "numericSortKey": 4.0})
        );
    }

    #[test]
    fn queue_body() {
        let data = json!("job");
        let body = CreateQueueItemBody {
            data: &data,
            priority: None,
            ttl: Some(duration(30)),
        };
        assert_eq!(
            serde_json::to_value(body).unwrap(),
            json!({"data": "job", "ttl": "30s"})
        );
    }

    #[test]
    fn flush_operation_id() {
        let op: MemoryStoreFlushOperation = serde_json::from_value(json!({
            "path": "cloud/v2/universes/1/memory-store/operations/abc123",
            "done": false
        }))
        .unwrap();
        assert_eq!(op.operation_id(), "abc123");
        assert!(!op.done);
    }

    #[test]
    fn decode_read_response() {
        let res: ReadMemoryStoreQueueItemsResponse = serde_json::from_value(json!({
            "queueItems": [{"path": "p", "data": {"x": 1}, "priority": 2.0}],
            "id": "read-1"
        }))
        .unwrap();
        assert_eq!(res.id, "read-1");
        assert_eq!(res.queue_items[0].priority, Some(2.0));
    }
}
