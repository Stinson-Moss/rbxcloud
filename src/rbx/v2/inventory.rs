use serde::{Deserialize, Serialize};

use crate::rbx::{error::Error, types::RobloxUserId, util::QueryString};

use crate::rbx::http::{self, HttpClient};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ListInventoryItemsParams {
    pub user_id: RobloxUserId,
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItems {
    #[serde(default)]
    pub inventory_items: Vec<InventoryItem>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_page_token: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItem {
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub asset_details: Option<InventoryItemAssetDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub badge_details: Option<InventoryItemBadgeDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub game_pass_details: Option<InventoryItemGamePassDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub private_server_details: Option<InventoryItemPrivateServerDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_time: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemAssetDetails {
    pub asset_id: String,
    pub instance_id: String,
    pub inventory_item_asset_type: InventoryItemAssetType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collectible_details: Option<InventoryItemCollectibleDetails>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemBadgeDetails {
    pub badge_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemGamePassDetails {
    pub game_pass_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemPrivateServerDetails {
    pub private_server_id: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct InventoryItemCollectibleDetails {
    pub item_id: String,
    pub instance_id: String,
    pub instance_state: InventoryItemInstanceState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<u64>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryItemInstanceState {
    CollectibleItemInstanceStateUnspecified,
    Available,
    Hold,
    /// A value added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum InventoryItemAssetType {
    ClassicTshirt,
    Audio,
    Hat,
    Model,
    ClassicShirt,
    ClassicPants,
    Decal,
    ClassicHead,
    Face,
    Gear,
    Animation,
    Torso,
    RightArm,
    LeftArm,
    LeftLeg,
    RightLeg,
    Package,
    Plugin,
    MeshPart,
    HairAccessory,
    FaceAccessory,
    NeckAccessory,
    ShoulderAccessory,
    FrontAccessory,
    BackAccessory,
    WaistAccessory,
    ClimbAnimation,
    DeathAnimation,
    FallAnimation,
    IdleAnimation,
    JumpAnimation,
    RunAnimation,
    SwimAnimation,
    WalkAnimation,
    PoseAnimation,
    EmoteAnimation,
    Video,
    TshirtAccessory,
    ShirtAccessory,
    PantsAccessory,
    JacketAccessory,
    SweaterAccessory,
    ShortsAccessory,
    LeftShoeAccessory,
    RightShoeAccessory,
    DressSkirtAccessory,
    EyebrowAccessory,
    EyelashAccessory,
    MoodAnimation,
    DynamicHead,
    CreatedPlace,
    PurchasedPlace,
    /// A value added by Roblox after this version of rbxcloud.
    #[serde(other)]
    Unknown,
}

pub async fn list_inventory_items(
    http: &HttpClient,
    params: &ListInventoryItemsParams,
) -> Result<InventoryItems, Error> {
    let url = format!(
        "https://apis.roblox.com/cloud/v2/users/{userId}/inventory-items",
        userId = params.user_id,
    );

    let mut query: QueryString = vec![];
    if let Some(max_page_size) = params.max_page_size {
        query.push(("maxPageSize", format!("{max_page_size}")));
    }
    if let Some(page_token) = &params.page_token {
        query.push(("pageToken", page_token.clone()));
    }
    if let Some(filter) = &params.filter {
        query.push(("filter", filter.clone()));
    }

    let res = http.get(url).query(&query).send().await?;

    http::json::<InventoryItems>(res).await
}
