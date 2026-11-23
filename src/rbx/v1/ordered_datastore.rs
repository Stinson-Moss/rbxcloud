//! Low-level OrderedDataStore API operations.
//!
//! Typically, these operations should be consumed through the `RbxExperience`
//! struct, obtained through the `RbxCloud` struct.
//!

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::rbx::v1::{PageSize, UniverseId};
use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    util::QueryString,
};

pub struct OrderedListEntriesParams {
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub max_page_size: Option<PageSize>,
    pub page_token: Option<String>,
    pub order_by: Option<String>,
    pub filter: Option<String>,
}

pub struct OrderedCreateEntryParams {
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
    pub value: i64,
}

pub struct OrderedUpdateEntryParams {
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
    pub value: i64,
    pub allow_missing: Option<bool>,
}

pub struct OrderedIncrementEntryParams {
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
    pub increment: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedEntry {
    pub path: String,
    pub id: String,
    pub value: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OrderedListEntriesResponse {
    pub entries: Vec<OrderedEntry>,
    pub next_page_token: Option<String>,
}

pub struct OrderedEntryParams {
    pub universe_id: UniverseId,
    pub ordered_datastore_name: String,
    pub scope: Option<String>,
    pub id: String,
}

fn build_url(
    endpoint: &str,
    universe_id: UniverseId,
    data_store: &str,
    scope: Option<&str>,
) -> String {
    let s = scope.unwrap_or("global");
    if endpoint.is_empty() {
        format!("https://apis.roblox.com/ordered-data-stores/v1/universes/{universe_id}/orderedDataStores/{data_store}/scopes/{s}")
    } else {
        format!(
			"https://apis.roblox.com/ordered-data-stores/v1/universes/{universe_id}/orderedDataStores/{data_store}/scopes/{s}{endpoint}",
		)
    }
}

/// List entries of an OrderedDataStore.
pub async fn list_entries(
    http: &HttpClient,
    params: &OrderedListEntriesParams,
) -> Result<OrderedListEntriesResponse, Error> {
    let url = build_url(
        "/entries",
        params.universe_id,
        &params.ordered_datastore_name,
        params.scope.as_deref(),
    );
    let mut query: QueryString = vec![];
    if let Some(max_page_size) = &params.max_page_size {
        query.push(("max_page_size", max_page_size.to_string()));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("page_token", page_token.to_string()));
    }
    if let Some(order_by) = &params.order_by {
        query.push(("order_by", order_by.to_string()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.to_string()));
    }
    let res = http.get(url).query(&query).send().await?;
    http::json::<OrderedListEntriesResponse>(res).await
}

/// Add a new entry to an OrderedDataStore.
pub async fn create_entry(
    http: &HttpClient,
    params: &OrderedCreateEntryParams,
) -> Result<OrderedEntry, Error> {
    let url = build_url(
        "/entries",
        params.universe_id,
        &params.ordered_datastore_name,
        params.scope.as_deref(),
    );
    let query: QueryString = vec![("id", params.id.to_string())];
    let body_json = json!({
        "value": &params.value,
    });
    let body = serde_json::to_string(&body_json)?;
    let res = http
        .post(url)
        .header("Content-Type", "application/json")
        .query(&query)
        .body(body.clone())
        .send()
        .await?;
    http::json::<OrderedEntry>(res).await
}

pub async fn get_entry(
    http: &HttpClient,
    params: &OrderedEntryParams,
) -> Result<OrderedEntry, Error> {
    let url = build_url(
        format!("/entries/{entry}", entry = params.id).as_str(),
        params.universe_id,
        &params.ordered_datastore_name,
        params.scope.as_deref(),
    );
    let res = http.get(url).send().await?;
    http::json::<OrderedEntry>(res).await
}

pub async fn delete_entry(http: &HttpClient, params: &OrderedEntryParams) -> Result<(), Error> {
    let url = build_url(
        format!("/entries/{entry}", entry = params.id).as_str(),
        params.universe_id,
        &params.ordered_datastore_name,
        params.scope.as_deref(),
    );
    let res = http.delete(url).send().await?;
    http::empty(res).await
}

pub async fn update_entry(
    http: &HttpClient,
    params: &OrderedUpdateEntryParams,
) -> Result<OrderedEntry, Error> {
    let url = build_url(
        format!("/entries/{entry}", entry = params.id).as_str(),
        params.universe_id,
        &params.ordered_datastore_name,
        params.scope.as_deref(),
    );
    let mut query: QueryString = vec![];
    if let Some(allow_missing) = &params.allow_missing {
        query.push(("allow_missing", allow_missing.to_string()));
    }
    let body_json = json!({
        "value": &params.value,
    });
    let body = serde_json::to_string(&body_json)?;
    let res = http
        .patch(url)
        .header("Content-Type", "application/json")
        .body(body)
        .query(&query)
        .send()
        .await?;
    http::json::<OrderedEntry>(res).await
}

pub async fn increment_entry(
    http: &HttpClient,
    params: &OrderedIncrementEntryParams,
) -> Result<OrderedEntry, Error> {
    let url = build_url(
        format!("/entries/{entry}:increment", entry = params.id).as_str(),
        params.universe_id,
        &params.ordered_datastore_name,
        params.scope.as_deref(),
    );
    let body_json = json!({
        "amount": &params.increment,
    });
    let body = serde_json::to_string(&body_json)?;
    let res = http
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?;
    http::json::<OrderedEntry>(res).await
}
