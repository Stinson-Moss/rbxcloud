//! Creator Store: distributing your assets as products, and searching the
//! store.
//!
//! Product IDs have the form `ProductNamespace-ProductType-123`; the
//! product's `path` (`creator-store-products/{id}`) in any response contains
//! it.

use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    util::{de_lenient_i64, encode_path_segment as seg, QueryString},
};

/// A decimal number: `significand * 10^exponent`.
#[derive(Deserialize, Serialize, Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Decimal {
    #[serde(default, deserialize_with = "de_lenient_i64")]
    pub significand: i64,
    #[serde(default)]
    pub exponent: i32,
}

impl Decimal {
    pub fn to_f64(self) -> f64 {
        self.significand as f64 * 10f64.powi(self.exponent)
    }
}

/// An amount of money.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct Money {
    pub currency_code: String,
    pub quantity: Decimal,
}

impl Money {
    /// A USD price in cents, in the exact form the Creator Store requires
    /// (exponent `-9`, or `0` for free).
    pub fn usd_cents(cents: u64) -> Self {
        let quantity = if cents == 0 {
            Decimal::default()
        } else {
            Decimal {
                significand: cents as i64 * 10_000_000,
                exponent: -9,
            }
        };
        Self {
            currency_code: "USD".to_string(),
            quantity,
        }
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:.2} {}", self.quantity.to_f64(), self.currency_code)
    }
}

/// A product on the Creator Store. Set exactly one `*_asset_id` when
/// creating a product.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct CreatorStoreProduct {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub base_price: Option<Money>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchase_price: Option<Money>,
    /// Whether the seller wants the product distributed on the store.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub published: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub restrictions: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub purchasable: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_seller: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub group_seller: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub plugin_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub audio_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub decal_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mesh_part_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub video_asset_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub font_family_asset_id: Option<String>,
}

/// Kinds of assets that can be sold on the Creator Store.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum CreatorStoreAssetKind {
    Model,
    Plugin,
    Audio,
    Decal,
    MeshPart,
    Video,
    FontFamily,
}

impl CreatorStoreProduct {
    /// A product for the given asset.
    pub fn for_asset(kind: CreatorStoreAssetKind, asset_id: u64) -> Self {
        let id = Some(asset_id.to_string());
        let mut product = Self::default();
        match kind {
            CreatorStoreAssetKind::Model => product.model_asset_id = id,
            CreatorStoreAssetKind::Plugin => product.plugin_asset_id = id,
            CreatorStoreAssetKind::Audio => product.audio_asset_id = id,
            CreatorStoreAssetKind::Decal => product.decal_asset_id = id,
            CreatorStoreAssetKind::MeshPart => product.mesh_part_asset_id = id,
            CreatorStoreAssetKind::Video => product.video_asset_id = id,
            CreatorStoreAssetKind::FontFamily => product.font_family_asset_id = id,
        }
        product
    }

    /// The product ID (last segment of `path`).
    pub fn product_id(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or_default()
    }
}

fn product_url(product_id: &str) -> String {
    http::url(&format!(
        "/cloud/v2/creator-store-products/{}",
        seg(product_id)
    ))
}

/// Add a product to the Creator Store. Only for products that have never
/// been distributed; otherwise use [`update_product`].
pub async fn create_product(
    http: &HttpClient,
    product: &CreatorStoreProduct,
) -> Result<CreatorStoreProduct, Error> {
    let res = http
        .post(http::url("/cloud/v2/creator-store-products"))
        .json(product)
        .send()
        .await?;
    http::json(res).await
}

/// Get a Creator Store product.
pub async fn get_product(
    http: &HttpClient,
    product_id: &str,
) -> Result<CreatorStoreProduct, Error> {
    let res = http.get(product_url(product_id)).send().await?;
    http::json(res).await
}

/// Update a Creator Store product.
///
/// `update_mask` is a comma-separated list of fields to update (e.g.
/// `basePrice,published`). With `allow_missing`, the product is created if
/// it doesn't exist (and the mask is ignored).
pub async fn update_product(
    http: &HttpClient,
    product_id: &str,
    product: &CreatorStoreProduct,
    update_mask: Option<String>,
    allow_missing: Option<bool>,
) -> Result<CreatorStoreProduct, Error> {
    let mut query: QueryString = vec![];
    if let Some(mask) = update_mask {
        query.push(("updateMask", mask));
    }
    if let Some(allow_missing) = allow_missing {
        query.push(("allowMissing", allow_missing.to_string()));
    }
    let res = http
        .patch(product_url(product_id))
        .query(&query)
        .json(product)
        .send()
        .await?;
    http::json(res).await
}

// ------------------------------------------------------------------ search

/// Store category to search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum SearchCategory {
    Audio,
    Place,
    Model,
    Decal,
    Plugin,
    MeshPart,
    Video,
    FontFamily,
}

impl SearchCategory {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Audio => "Audio",
            Self::Place => "Place",
            Self::Model => "Model",
            Self::Decal => "Decal",
            Self::Plugin => "Plugin",
            Self::MeshPart => "MeshPart",
            Self::Video => "Video",
            Self::FontFamily => "FontFamily",
        }
    }
}

/// How to sort search results.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum SearchSort {
    Relevance,
    Trending,
    Top,
    AudioDuration,
    CreateTime,
    UpdatedTime,
    Ratings,
}

impl SearchSort {
    fn as_str(&self) -> &'static str {
        match self {
            Self::Relevance => "Relevance",
            Self::Trending => "Trending",
            Self::Top => "Top",
            Self::AudioDuration => "AudioDuration",
            Self::CreateTime => "CreateTime",
            Self::UpdatedTime => "UpdatedTime",
            Self::Ratings => "Ratings",
        }
    }
}

/// Parameters for searching the Creator Store.
#[derive(Debug, Clone, Default)]
pub struct SearchParams {
    pub category: Option<SearchCategory>,
    pub query: Option<String>,
    pub sort: Option<SearchSort>,
    /// `true` for descending order.
    pub descending: Option<bool>,
    pub creator_user_id: Option<u64>,
    pub creator_group_id: Option<u64>,
    pub only_verified_creators: Option<bool>,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
}

/// An asset listed on the Creator Store. The nested objects vary by asset
/// type and are kept as JSON.
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CreatorStoreAsset {
    #[serde(default)]
    pub asset: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_store_product: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub voting: Option<Value>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SearchCreatorStoreAssetsResponse {
    #[serde(default)]
    pub creator_store_assets: Vec<CreatorStoreAsset>,
    #[serde(default)]
    pub total_results: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
    #[serde(flatten)]
    pub other: Map<String, Value>,
}

/// Search the Creator Store.
pub async fn search(
    http: &HttpClient,
    params: &SearchParams,
) -> Result<SearchCreatorStoreAssetsResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(category) = params.category {
        query.push(("searchCategoryType", category.as_str().to_string()));
    }
    if let Some(q) = &params.query {
        query.push(("query", q.clone()));
    }
    if let Some(sort) = params.sort {
        query.push(("sortCategory", sort.as_str().to_string()));
    }
    if let Some(descending) = params.descending {
        let direction = if descending {
            "Descending"
        } else {
            "Ascending"
        };
        query.push(("sortDirection", direction.to_string()));
    }
    if let Some(id) = params.creator_user_id {
        query.push(("userId", id.to_string()));
    }
    if let Some(id) = params.creator_group_id {
        query.push(("groupId", id.to_string()));
    }
    if let Some(verified) = params.only_verified_creators {
        query.push(("includeOnlyVerifiedCreators", verified.to_string()));
    }
    if let Some(n) = params.max_page_size {
        query.push(("maxPageSize", n.to_string()));
    }
    if let Some(token) = &params.page_token {
        query.push(("pageToken", token.clone()));
    }
    let res = http
        .get(http::url("/toolbox-service/v2/assets:search"))
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get the Creator Store details of an asset.
pub async fn get_asset(http: &HttpClient, asset_id: u64) -> Result<CreatorStoreAsset, Error> {
    let res = http
        .get(http::url(&format!("/toolbox-service/v2/assets/{asset_id}")))
        .send()
        .await?;
    http::json(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn usd_prices() {
        let price = Money::usd_cents(499);
        assert_eq!(
            serde_json::to_value(&price).unwrap(),
            json!({"currencyCode": "USD", "quantity": {"significand": 4990000000i64, "exponent": -9}})
        );
        assert_eq!(price.to_string(), "4.99 USD");
        assert_eq!(
            serde_json::to_value(Money::usd_cents(0)).unwrap(),
            json!({"currencyCode": "USD", "quantity": {"significand": 0, "exponent": 0}})
        );
    }

    #[test]
    fn product_for_asset() {
        let mut product = CreatorStoreProduct::for_asset(CreatorStoreAssetKind::Plugin, 123);
        product.base_price = Some(Money::usd_cents(0));
        product.published = Some(true);
        assert_eq!(
            serde_json::to_value(&product).unwrap(),
            json!({
                "basePrice": {"currencyCode": "USD", "quantity": {"significand": 0, "exponent": 0}},
                "published": true,
                "pluginAssetId": "123"
            })
        );
    }

    #[test]
    fn decode_product() {
        let product: CreatorStoreProduct = serde_json::from_value(json!({
            "path": "creator-store-products/CreatorMarketplaceAsset-Plugin-123",
            "basePrice": {"currencyCode": "USD", "quantity": {"significand": "4990000000", "exponent": -9}},
            "published": true,
            "restrictions": ["SOLD_ITEM_RESTRICTED"],
            "purchasable": false,
            "pluginAssetId": "123"
        }))
        .unwrap();
        assert_eq!(product.product_id(), "CreatorMarketplaceAsset-Plugin-123");
        assert_eq!(product.base_price.unwrap().to_string(), "4.99 USD");
    }
}
