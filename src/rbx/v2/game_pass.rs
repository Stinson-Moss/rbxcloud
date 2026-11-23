//! Game passes (`/game-passes/v1`).

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::rbx::{
    error::Error,
    http::{self, HttpClient},
    types::UniverseId,
    util::QueryString,
};

use super::form::{FormBuilder, ImageUpload};

/// Price settings shared by game passes and developer products.
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct PriceInformation {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_price_in_robux: Option<i64>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub enabled_features: Vec<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GamePass {
    pub game_pass_id: u64,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub is_for_sale: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub icon_asset_id: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub price_information: Option<PriceInformation>,
    #[serde(default)]
    pub is_managed_pricing_enabled: bool,
    #[serde(default)]
    pub is_trial_enabled: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_timestamp: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_timestamp: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ListGamePassesResponse {
    #[serde(default)]
    pub game_passes: Vec<GamePass>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

/// Fields to set when creating or updating a game pass. `None` leaves a
/// field unchanged (or at Roblox's default when creating).
#[derive(Debug, Clone, Default)]
pub struct GamePassFields {
    /// Required when creating.
    pub name: Option<String>,
    pub description: Option<String>,
    /// Price in Robux.
    pub price: Option<u64>,
    pub is_for_sale: Option<bool>,
    pub is_managed_pricing_enabled: Option<bool>,
    pub is_trial_enabled: Option<bool>,
    pub image: Option<ImageUpload>,
}

impl GamePassFields {
    fn form(&self) -> FormBuilder {
        FormBuilder::default()
            .text("name", self.name.as_ref())
            .text("description", self.description.as_ref())
            .text("price", self.price)
            .text("isForSale", self.is_for_sale)
            .text("isManagedPricingEnabled", self.is_managed_pricing_enabled)
            .text("isTrialEnabled", self.is_trial_enabled)
    }
}

fn base_url(universe_id: UniverseId) -> String {
    http::url(&format!(
        "/game-passes/v1/universes/{universe_id}/game-passes"
    ))
}

/// List a universe's game passes with their configuration.
pub async fn list_game_passes(
    http: &HttpClient,
    universe_id: UniverseId,
    page_size: Option<u32>,
    page_token: Option<String>,
    for_sale_only: Option<bool>,
) -> Result<ListGamePassesResponse, Error> {
    let mut query: QueryString = vec![];
    if let Some(n) = page_size {
        query.push(("pageSize", n.to_string()));
    }
    if let Some(token) = page_token {
        query.push(("pageToken", token));
    }
    if let Some(for_sale) = for_sale_only {
        query.push(("isForSale", for_sale.to_string()));
    }
    let res = http
        .get(format!("{}/creator", base_url(universe_id)))
        .query(&query)
        .send()
        .await?;
    http::json(res).await
}

/// Get a game pass's configuration.
pub async fn get_game_pass(
    http: &HttpClient,
    universe_id: UniverseId,
    game_pass_id: u64,
) -> Result<GamePass, Error> {
    let res = http
        .get(format!("{}/{game_pass_id}/creator", base_url(universe_id)))
        .send()
        .await?;
    http::json(res).await
}

/// Create a game pass. `fields.name` is required.
pub async fn create_game_pass(
    http: &HttpClient,
    universe_id: UniverseId,
    fields: &GamePassFields,
) -> Result<GamePass, Error> {
    if fields.name.is_none() {
        return Err(Error::EndpointError(
            "a name is required to create a game pass".to_string(),
        ));
    }
    let form = fields.form().build("imageFile", fields.image.as_ref())?;
    let res = http
        .post(base_url(universe_id))
        .multipart(form)
        .send()
        .await?;
    http::json(res).await
}

/// Update a game pass. Only fields that are `Some` are changed.
pub async fn update_game_pass(
    http: &HttpClient,
    universe_id: UniverseId,
    game_pass_id: u64,
    fields: &GamePassFields,
) -> Result<(), Error> {
    let form = fields.form().build("imageFile", fields.image.as_ref())?;
    let res = http
        .patch(format!("{}/{game_pass_id}", base_url(universe_id)))
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
    fn decode_game_pass() {
        let res: ListGamePassesResponse = serde_json::from_value(json!({
            "gamePasses": [{
                "gamePassId": 123,
                "name": "VIP",
                "description": "",
                "isForSale": true,
                "iconAssetId": 456,
                "createdTimestamp": "2024-01-01T00:00:00Z",
                "updatedTimestamp": "2024-01-01T00:00:00Z",
                "priceInformation": {"defaultPriceInRobux": 100, "enabledFeatures": []},
                "isManagedPricingEnabled": false,
                "isTrialEnabled": false
            }],
            "nextPageToken": ""
        }))
        .unwrap();
        let pass = &res.game_passes[0];
        assert_eq!(pass.game_pass_id, 123);
        assert_eq!(
            pass.price_information
                .as_ref()
                .unwrap()
                .default_price_in_robux,
            Some(100)
        );
    }

    #[test]
    fn form_fields_use_api_names() {
        let fields = GamePassFields {
            name: Some("VIP".into()),
            price: Some(250),
            is_for_sale: Some(true),
            ..Default::default()
        };
        assert_eq!(
            fields.form().fields(),
            &[
                ("name", "VIP".to_string()),
                ("price", "250".to_string()),
                ("isForSale", "true".to_string())
            ]
        );
    }
}
