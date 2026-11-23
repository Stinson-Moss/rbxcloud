//! Ordered data stores (Open Cloud v2).
//!
//! Most usage should go through
//! [`OrderedDataStoreClient`](super::OrderedDataStoreClient), obtained from
//! [`Client::ordered_data_store`](super::Client::ordered_data_store).

use serde::{Deserialize, Serialize};

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::UniverseId,
    util::{de_lenient_i64, encode_path_segment as seg, QueryString},
};

/// Default scope used by `OrderedDataStore` in Luau.
pub const DEFAULT_SCOPE: &str = "global";

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OrderedDataStoreEntry {
    pub path: String,
    /// The entry key.
    #[serde(default)]
    pub id: String,
    #[serde(deserialize_with = "de_lenient_i64")]
    pub value: i64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListOrderedDataStoreEntriesResponse {
    #[serde(default)]
    pub ordered_data_store_entries: Vec<OrderedDataStoreEntry>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Identifies an ordered data store scope.
#[derive(Debug, Clone)]
pub struct OrderedDataStoreLocation {
    pub universe_id: UniverseId,
    pub ordered_data_store_id: String,
    /// Defaults to [`DEFAULT_SCOPE`] when `None`.
    pub scope: Option<String>,
}

impl OrderedDataStoreLocation {
    fn entries_url(&self) -> String {
        http::url(&format!(
            "/cloud/v2/universes/{}/ordered-data-stores/{}/scopes/{}/entries",
            self.universe_id,
            seg(&self.ordered_data_store_id),
            seg(self.scope.as_deref().unwrap_or(DEFAULT_SCOPE)),
        ))
    }

    fn entry_url(&self, entry_id: &str) -> String {
        format!("{}/{}", self.entries_url(), seg(entry_id))
    }
}

/// Sort direction for listing entries by value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum OrderedDataStoreSortOrder {
    Asc,
    Desc,
}

pub struct ListOrderedEntriesParams {
    pub location: OrderedDataStoreLocation,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub order: Option<OrderedDataStoreSortOrder>,
    /// e.g. `entry >= 10 && entry <= 50`
    pub filter: Option<String>,
}

pub struct OrderedEntryParams {
    pub location: OrderedDataStoreLocation,
    pub entry_id: String,
}

pub struct CreateOrderedEntryParams {
    pub location: OrderedDataStoreLocation,
    pub entry_id: String,
    pub value: i64,
}

pub struct UpdateOrderedEntryParams {
    pub location: OrderedDataStoreLocation,
    pub entry_id: String,
    pub value: i64,
    /// Create the entry if it does not exist.
    pub allow_missing: Option<bool>,
}

pub struct IncrementOrderedEntryParams {
    pub location: OrderedDataStoreLocation,
    pub entry_id: String,
    pub amount: i64,
}

#[derive(Serialize)]
struct ValueBody {
    value: i64,
}

#[derive(Serialize)]
struct AmountBody {
    amount: i64,
}

/// List entries, optionally sorted by value.
pub async fn list_entries(
    http: &HttpClient,
    params: &ListOrderedEntriesParams,
) -> Result<ListOrderedDataStoreEntriesResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(n) = params.max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = &params.page_token {
        query.push(("pageToken", token.clone()));
    }
    if let Some(order) = params.order {
        let order_by = match order {
            OrderedDataStoreSortOrder::Asc => "value",
            OrderedDataStoreSortOrder::Desc => "value desc",
        };
        query.push(("orderBy", order_by.to_string()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.clone()));
    }
    let res = http
        .get(params.location.entries_url())
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Create an entry.
pub async fn create_entry(
    http: &HttpClient,
    params: &CreateOrderedEntryParams,
) -> Result<OrderedDataStoreEntry, Error> {
    let res = http
        .post(params.location.entries_url())
        .query(&[("id", &params.entry_id)])
        .json(&ValueBody {
            value: params.value,
        })
        .send()
        .await?;
    http::json(res).await
}

/// Get an entry.
pub async fn get_entry(
    http: &HttpClient,
    params: &OrderedEntryParams,
) -> Result<OrderedDataStoreEntry, Error> {
    let res = http
        .get(params.location.entry_url(&params.entry_id))
        .send()
        .await?;
    http::json(res).await
}

/// Set an entry's value.
pub async fn update_entry(
    http: &HttpClient,
    params: &UpdateOrderedEntryParams,
) -> Result<OrderedDataStoreEntry, Error> {
    let mut query: QueryString = vec![];
    if let Some(allow_missing) = params.allow_missing {
        query.push(("allowMissing", allow_missing.to_string()));
    }
    let res = http
        .patch(params.location.entry_url(&params.entry_id))
        .query(&query)
        .json(&ValueBody {
            value: params.value,
        })
        .send()
        .await?;
    http::json(res).await
}

/// Increment an entry's value, creating it if needed.
pub async fn increment_entry(
    http: &HttpClient,
    params: &IncrementOrderedEntryParams,
) -> Result<OrderedDataStoreEntry, Error> {
    let url = format!("{}:increment", params.location.entry_url(&params.entry_id));
    let res = http
        .post(url)
        .json(&AmountBody {
            amount: params.amount,
        })
        .send()
        .await?;
    http::json(res).await
}

/// Delete an entry.
pub async fn delete_entry(http: &HttpClient, params: &OrderedEntryParams) -> Result<(), Error> {
    let res = http
        .delete(params.location.entry_url(&params.entry_id))
        .send()
        .await?;
    http::empty(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn default_scope_in_url() {
        let loc = OrderedDataStoreLocation {
            universe_id: UniverseId(1),
            ordered_data_store_id: "Leaderboard".to_string(),
            scope: None,
        };
        assert_eq!(
            loc.entry_url("Player1"),
            "https://apis.roblox.com/cloud/v2/universes/1/ordered-data-stores/Leaderboard/scopes/global/entries/Player1"
        );
    }

    #[test]
    fn decode_entries() {
        let res: ListOrderedDataStoreEntriesResponse = serde_json::from_value(json!({
            "orderedDataStoreEntries": [
                {"path": "p/1", "id": "a", "value": 10},
                {"path": "p/2", "id": "b", "value": 9.0},
                {"path": "p/3", "id": "c", "value": "8"}
            ],
            "nextPageToken": "abc"
        }))
        .unwrap();
        let values: Vec<i64> = res
            .ordered_data_store_entries
            .iter()
            .map(|e| e.value)
            .collect();
        assert_eq!(values, vec![10, 9, 8]);
        assert_eq!(res.next_page_token.as_deref(), Some("abc"));
    }
}
