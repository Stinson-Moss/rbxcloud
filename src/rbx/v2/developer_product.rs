//! Developer products (`/developer-products/v2`).

use serde::{Deserialize, Serialize};

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::UniverseId,
    util::QueryString,
};

use super::{
    form::{FormBuilder, ImageUpload},
    game_pass::PriceInformation,
};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeveloperProduct {
    pub product_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_image_asset_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub universe_id: Option<u64>,
    #[serde(default)]
    pub is_for_sale: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub store_page_enabled: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_information: Option<PriceInformation>,
    #[serde(default)]
    pub is_immutable: bool,
    #[serde(default)]
    pub is_managed_pricing_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListDeveloperProductsResponse {
    #[serde(default)]
    pub developer_products: Vec<DeveloperProduct>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Fields to set when creating or updating a developer product. `None`
/// leaves a field unchanged (or at Roblox's default when creating).
#[derive(Debug, Clone, Default)]
pub struct DeveloperProductFields {
    /// Required when creating.
    pub name: Option<String>,
    pub description: Option<String>,
    /// Price in Robux.
    pub price: Option<u64>,
    pub is_for_sale: Option<bool>,
    pub is_regional_pricing_enabled: Option<bool>,
    pub is_managed_pricing_enabled: Option<bool>,
    /// Only applies to updates.
    pub store_page_enabled: Option<bool>,
    pub image: Option<ImageUpload>,
}

impl DeveloperProductFields {
    fn form(&self) -> FormBuilder {
        FormBuilder::default()
            .text("name", self.name.as_ref())
            .text("description", self.description.as_ref())
            .text("price", self.price)
            .text("isForSale", self.is_for_sale)
            .text("isRegionalPricingEnabled", self.is_regional_pricing_enabled)
            .text("isManagedPricingEnabled", self.is_managed_pricing_enabled)
            .text("storePageEnabled", self.store_page_enabled)
    }
}

fn base_url(universe_id: UniverseId) -> String {
    http::url(&format!(
        "/developer-products/v2/universes/{universe_id}/developer-products"
    ))
}

/// List a universe's developer products with their configuration.
pub async fn list_developer_products(
    http: &HttpClient,
    universe_id: UniverseId,
    page_size: Option<u32>,
    page_token: Option<String>,
) -> Result<ListDeveloperProductsResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(n) = page_size {
        query.push(("pageSize", n.to_string()));
    }
    if let Some(token) = page_token {
        query.push(("pageToken", token));
    }
    let res = http
        .get(format!("{}/creator", base_url(universe_id)))
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get a developer product's configuration.
pub async fn get_developer_product(
    http: &HttpClient,
    universe_id: UniverseId,
    product_id: u64,
) -> Result<DeveloperProduct, Error> {
    let res = http
        .get(format!("{}/{product_id}/creator", base_url(universe_id)))
        .send()
        .await?;
    http::json(res).await
}

/// Create a developer product. `fields.name` is required.
pub async fn create_developer_product(
    http: &HttpClient,
    universe_id: UniverseId,
    fields: &DeveloperProductFields,
) -> Result<DeveloperProduct, Error> {
    if fields.name.is_none() {
        return Err(Error::EndpointError(
            "a name is required to create a developer product".to_string(),
        ));
    }
    let mut create_fields = fields.clone();
    // Not accepted on create.
    create_fields.store_page_enabled = None;
    let form = create_fields
        .form()
        .build("imageFile", fields.image.as_ref())?;
    let res = http
        .post(base_url(universe_id))
        .multipart(form)
        .send()
        .await?;
    http::json(res).await
}

/// Update a developer product. Only fields that are `Some` are changed.
pub async fn update_developer_product(
    http: &HttpClient,
    universe_id: UniverseId,
    product_id: u64,
    fields: &DeveloperProductFields,
) -> Result<(), Error> {
    let form = fields.form().build("imageFile", fields.image.as_ref())?;
    let res = http
        .patch(format!("{}/{product_id}", base_url(universe_id)))
        .multipart(form)
        .send()
        .await?;
    http::empty(res).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn decode_developer_product() {
        let product: DeveloperProduct = serde_json::from_value(json!({
            "productId": 9,
            "name": "100 Coins",
            "description": "",
            "iconImageAssetId": 0,
            "universeId": 1,
            "isForSale": true,
            "storePageEnabled": false,
            "priceInformation": {"defaultPriceInRobux": 25, "enabledFeatures": ["RegionalPricing"]},
            "isImmutable": false,
            "createdTimestamp": "2024-01-01T00:00:00Z",
            "updatedTimestamp": "2024-01-01T00:00:00Z",
            "isManagedPricingEnabled": false
        }))
        .unwrap();
        assert_eq!(product.product_id, 9);
        assert_eq!(
            product.price_information.unwrap().default_price_in_robux,
            Some(25)
        );
    }
}
