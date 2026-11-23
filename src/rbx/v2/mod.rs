//! Access into Roblox v2 APIs.
//!
//! Most usage should go through the `Client` struct.

use inventory::{InventoryItems, ListInventoryItemsParams};
use luau_execution::{
    CreateLuauExecutionTaskParams, GetLuauExecutionSessionTaskLogsParams,
    GetLuauExecutionSessionTaskParams, LuauExecutionSessionTask, LuauExecutionSessionTaskLogPage,
    LuauExecutionTaskLogView, NewLuauExecutionSessionTask,
};
use place::{GetPlaceParams, PlaceInfo, UpdatePlaceInfo, UpdatePlaceParams};
use rand::{distr::Alphanumeric, Rng};
use universe::{
    GetUniverseParams, PublishUniverseMessageParams, RestartUniverseServersParams, UniverseInfo,
    UpdateUniverseInfo, UpdateUniverseParams,
};
use user::{
    GenerateUserThumbnailOperationResponse, GenerateUserThumbnailParams, GetUserParams,
    GetUserResponse, UserThumbnailFormat, UserThumbnailShape, UserThumbnailSize,
};
use user_restriction::{
    GetUserRestrictionParams, ListUserRestrictionLogsParams, ListUserRestrictionsParams,
    UpdateUserRestrictionParams, UserRestriction, UserRestrictionList, UserRestrictionLogsList,
};

use self::{
    group::{
        GetGroupParams, GetGroupResponse, GetGroupShoutParams, GetGroupShoutResponse,
        ListGroupMembershipsParams, ListGroupMembershipsResponse, ListGroupRolesParams,
        ListGroupRolesResponse,
    },
    notification::{Notification, NotificationParams, NotificationResponse},
    subscription::{GetSubscriptionParams, GetSubscriptionResponse, SubscriptionView},
};
pub mod creator_store;
pub mod data_store;
pub mod developer_product;
mod form;
pub mod game_pass;
pub mod group;
pub mod instance;
pub mod inventory;
pub mod luau_execution;
pub mod memory_store;
pub mod notification;
pub mod operation;
pub mod ordered_data_store;
pub mod place;
pub mod secret;
pub mod subscription;
pub mod universe;
pub mod user;
pub mod user_restriction;

use crate::rbx::{error::Error, http::HttpClient};
pub use form::ImageUpload;

use creator_store::{CreatorStoreAsset, CreatorStoreProduct, SearchCreatorStoreAssetsResponse};
use developer_product::{DeveloperProduct, DeveloperProductFields, ListDeveloperProductsResponse};
use game_pass::{GamePass, GamePassFields, ListGamePassesResponse};
use group::{
    GroupListParams, GroupMembership, GroupRole, ListGroupForumCategoriesResponse,
    ListGroupForumCommentsResponse, ListGroupForumPostsResponse, ListGroupJoinRequestsResponse,
};
use instance::{Instance, InstanceDetails, ListInstanceChildrenResponse};
use operation::PollOptions;
use secret::{ListSecretsResponse, Secret, UniversePublicKey};
use serde_json::{Map, Value};
use user::ListAssetQuotasResponse;

use data_store::{
    DataStore, DataStoreEntry, DataStoreLocation, ListDataStoreEntriesResponse,
    ListDataStoresResponse, SnapshotDataStoresResponse,
};
use memory_store::{
    ListMemoryStoreSortedMapItemsResponse, MemoryStoreFlushOperation, MemoryStoreQueueItem,
    MemoryStoreScope, MemoryStoreSortedMapItem, ReadMemoryStoreQueueItemsResponse, SortKey,
    SortedMapOrder,
};
use ordered_data_store::{
    ListOrderedDataStoreEntriesResponse, OrderedDataStoreEntry, OrderedDataStoreLocation,
    OrderedDataStoreSortOrder,
};

use super::types::{GroupId, PlaceId, RobloxUserId, UniverseId};

/// Access into the Roblox Open Cloud APIs.
///
/// ```rust,no_run
/// use rbxcloud::rbx::v2::Client;
///
/// let client = Client::new("API_KEY");
/// ```
///
/// To configure timeouts or a proxy, build an [`HttpClient`] first:
///
/// ```rust,no_run
/// use std::time::Duration;
/// use rbxcloud::rbx::{http::HttpClient, v2::Client};
///
/// # fn main() -> Result<(), rbxcloud::rbx::error::Error> {
/// let http = HttpClient::builder("API_KEY")
///     .timeout(Duration::from_secs(30))
///     .proxy("http://localhost:8080")
///     .build()?;
/// let client = Client::with_http(http);
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Client {
    http: HttpClient,
}

pub struct GroupClient {
    http: HttpClient,
    pub group_id: GroupId,
}

pub struct InventoryClient {
    http: HttpClient,
}

pub struct LuauExecutionClient {
    http: HttpClient,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub version_id: Option<String>,
}

pub struct SubscriptionClient {
    http: HttpClient,
}

pub struct NotificationClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

pub struct PlaceClient {
    http: HttpClient,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
}

pub struct UniverseClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

pub struct UserClient {
    http: HttpClient,
}

pub struct UserRestrictionClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

pub struct UserRestrictionParams {
    pub user_id: RobloxUserId,
    pub place_id: Option<PlaceId>,
    pub active: Option<bool>,
    pub duration: Option<u64>,
    pub private_reason: Option<String>,
    pub display_reason: Option<String>,
    pub exclude_alt_accounts: Option<bool>,
}

/// Options for listing pages of resources.
#[derive(Debug, Clone, Default)]
pub struct PageOptions {
    pub max_page_size: Option<u32>,
    pub page_token: Option<String>,
    pub filter: Option<String>,
    pub show_deleted: Option<bool>,
}

/// Optional metadata stored alongside a data store entry.
#[derive(Debug, Clone, Default)]
pub struct EntryMetadata {
    pub users: Option<Vec<RobloxUserId>>,
    pub attributes: Option<Map<String, Value>>,
}

/// Access to the data stores of a universe (Open Cloud v2).
#[derive(Debug, Clone)]
pub struct DataStoreClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

impl DataStoreClient {
    fn location(&self, data_store_id: &str, scope: Option<&str>) -> DataStoreLocation {
        DataStoreLocation {
            universe_id: self.universe_id,
            data_store_id: data_store_id.to_string(),
            scope: scope.map(str::to_string),
        }
    }

    /// List data stores in the universe.
    pub async fn list_data_stores(
        &self,
        options: PageOptions,
    ) -> Result<ListDataStoresResponse, Error> {
        data_store::list_data_stores(
            &self.http,
            &data_store::ListDataStoresParams {
                universe_id: self.universe_id,
                max_page_size: options.max_page_size,
                page_token: options.page_token,
                filter: options.filter,
                show_deleted: options.show_deleted,
            },
        )
        .await
    }

    /// Schedule a data store for deletion.
    pub async fn delete_data_store(&self, data_store_id: &str) -> Result<DataStore, Error> {
        data_store::delete_data_store(&self.http, self.universe_id, data_store_id).await
    }

    /// Restore a data store scheduled for deletion.
    pub async fn undelete_data_store(&self, data_store_id: &str) -> Result<DataStore, Error> {
        data_store::undelete_data_store(&self.http, self.universe_id, data_store_id).await
    }

    /// Snapshot all data stores in the universe (at most once per UTC day).
    pub async fn snapshot(&self) -> Result<SnapshotDataStoresResponse, Error> {
        data_store::snapshot_data_stores(&self.http, self.universe_id).await
    }

    /// List entries in a data store.
    pub async fn list_entries(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        options: PageOptions,
    ) -> Result<ListDataStoreEntriesResponse, Error> {
        data_store::list_entries(
            &self.http,
            &data_store::ListEntriesParams {
                location: self.location(data_store_id, scope),
                max_page_size: options.max_page_size,
                page_token: options.page_token,
                filter: options.filter,
                show_deleted: options.show_deleted,
            },
        )
        .await
    }

    /// Get an entry. Use `key@revision_id` as `entry_id` for a past revision.
    pub async fn get_entry(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
    ) -> Result<DataStoreEntry, Error> {
        data_store::get_entry(
            &self.http,
            &data_store::EntryParams {
                location: self.location(data_store_id, scope),
                entry_id: entry_id.to_string(),
            },
        )
        .await
    }

    /// Create an entry. Fails if it already exists.
    pub async fn create_entry(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        value: Value,
        metadata: EntryMetadata,
    ) -> Result<DataStoreEntry, Error> {
        data_store::create_entry(
            &self.http,
            &data_store::CreateEntryParams {
                location: self.location(data_store_id, scope),
                entry_id: entry_id.to_string(),
                value,
                users: metadata.users,
                attributes: metadata.attributes,
            },
        )
        .await
    }

    /// Set an entry's value, creating it if it does not exist. Pass `etag` to
    /// only write when the entry is unchanged since it was read.
    pub async fn set_entry(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        value: Value,
        etag: Option<String>,
        metadata: EntryMetadata,
    ) -> Result<DataStoreEntry, Error> {
        data_store::update_entry(
            &self.http,
            &data_store::UpdateEntryParams {
                location: self.location(data_store_id, scope),
                entry_id: entry_id.to_string(),
                value,
                etag,
                users: metadata.users,
                attributes: metadata.attributes,
                allow_missing: Some(true),
            },
        )
        .await
    }

    /// Increment a numeric entry, creating it if needed.
    pub async fn increment_entry(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        amount: f64,
        metadata: EntryMetadata,
    ) -> Result<DataStoreEntry, Error> {
        data_store::increment_entry(
            &self.http,
            &data_store::IncrementEntryParams {
                location: self.location(data_store_id, scope),
                entry_id: entry_id.to_string(),
                amount,
                users: metadata.users,
                attributes: metadata.attributes,
            },
        )
        .await
    }

    /// Delete an entry.
    pub async fn delete_entry(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
    ) -> Result<(), Error> {
        data_store::delete_entry(
            &self.http,
            &data_store::EntryParams {
                location: self.location(data_store_id, scope),
                entry_id: entry_id.to_string(),
            },
        )
        .await
    }

    /// List an entry's revisions.
    pub async fn list_entry_revisions(
        &self,
        data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        options: PageOptions,
    ) -> Result<ListDataStoreEntriesResponse, Error> {
        data_store::list_entry_revisions(
            &self.http,
            &data_store::ListEntryRevisionsParams {
                location: self.location(data_store_id, scope),
                entry_id: entry_id.to_string(),
                max_page_size: options.max_page_size,
                page_token: options.page_token,
                filter: options.filter,
            },
        )
        .await
    }
}

/// Access to the ordered data stores of a universe (Open Cloud v2).
#[derive(Debug, Clone)]
pub struct OrderedDataStoreClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

impl OrderedDataStoreClient {
    fn location(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
    ) -> OrderedDataStoreLocation {
        OrderedDataStoreLocation {
            universe_id: self.universe_id,
            ordered_data_store_id: ordered_data_store_id.to_string(),
            scope: scope.map(str::to_string),
        }
    }

    /// List entries, optionally sorted by value.
    pub async fn list_entries(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
        order: Option<OrderedDataStoreSortOrder>,
        options: PageOptions,
    ) -> Result<ListOrderedDataStoreEntriesResponse, Error> {
        ordered_data_store::list_entries(
            &self.http,
            &ordered_data_store::ListOrderedEntriesParams {
                location: self.location(ordered_data_store_id, scope),
                max_page_size: options.max_page_size,
                page_token: options.page_token,
                order,
                filter: options.filter,
            },
        )
        .await
    }

    pub async fn get_entry(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
    ) -> Result<OrderedDataStoreEntry, Error> {
        ordered_data_store::get_entry(
            &self.http,
            &ordered_data_store::OrderedEntryParams {
                location: self.location(ordered_data_store_id, scope),
                entry_id: entry_id.to_string(),
            },
        )
        .await
    }

    pub async fn create_entry(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        value: i64,
    ) -> Result<OrderedDataStoreEntry, Error> {
        ordered_data_store::create_entry(
            &self.http,
            &ordered_data_store::CreateOrderedEntryParams {
                location: self.location(ordered_data_store_id, scope),
                entry_id: entry_id.to_string(),
                value,
            },
        )
        .await
    }

    pub async fn update_entry(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        value: i64,
        allow_missing: Option<bool>,
    ) -> Result<OrderedDataStoreEntry, Error> {
        ordered_data_store::update_entry(
            &self.http,
            &ordered_data_store::UpdateOrderedEntryParams {
                location: self.location(ordered_data_store_id, scope),
                entry_id: entry_id.to_string(),
                value,
                allow_missing,
            },
        )
        .await
    }

    pub async fn increment_entry(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
        amount: i64,
    ) -> Result<OrderedDataStoreEntry, Error> {
        ordered_data_store::increment_entry(
            &self.http,
            &ordered_data_store::IncrementOrderedEntryParams {
                location: self.location(ordered_data_store_id, scope),
                entry_id: entry_id.to_string(),
                amount,
            },
        )
        .await
    }

    pub async fn delete_entry(
        &self,
        ordered_data_store_id: &str,
        scope: Option<&str>,
        entry_id: &str,
    ) -> Result<(), Error> {
        ordered_data_store::delete_entry(
            &self.http,
            &ordered_data_store::OrderedEntryParams {
                location: self.location(ordered_data_store_id, scope),
                entry_id: entry_id.to_string(),
            },
        )
        .await
    }
}

/// Access to the memory stores of a universe (Open Cloud v2).
#[derive(Debug, Clone)]
pub struct MemoryStoreClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

impl MemoryStoreClient {
    /// Add an item to a queue.
    pub async fn queue_add(
        &self,
        queue_id: &str,
        data: Value,
        priority: Option<f64>,
        ttl_seconds: Option<u64>,
    ) -> Result<MemoryStoreQueueItem, Error> {
        memory_store::create_queue_item(
            &self.http,
            &memory_store::CreateQueueItemParams {
                universe_id: self.universe_id,
                queue_id: queue_id.to_string(),
                data,
                priority,
                ttl_seconds,
            },
        )
        .await
    }

    /// Read items from a queue.
    pub async fn queue_read(
        &self,
        queue_id: &str,
        count: Option<u32>,
        all_or_nothing: Option<bool>,
        invisibility_window_seconds: Option<u64>,
    ) -> Result<ReadMemoryStoreQueueItemsResponse, Error> {
        memory_store::read_queue_items(
            &self.http,
            &memory_store::ReadQueueItemsParams {
                universe_id: self.universe_id,
                queue_id: queue_id.to_string(),
                count,
                all_or_nothing,
                invisibility_window_seconds,
            },
        )
        .await
    }

    /// Remove items returned by a previous read.
    pub async fn queue_discard(&self, queue_id: &str, read_id: &str) -> Result<(), Error> {
        memory_store::discard_queue_items(
            &self.http,
            &memory_store::DiscardQueueItemsParams {
                universe_id: self.universe_id,
                queue_id: queue_id.to_string(),
                read_id: read_id.to_string(),
            },
        )
        .await
    }

    /// List items in a sorted map.
    pub async fn sorted_map_list(
        &self,
        sorted_map_id: &str,
        order: Option<SortedMapOrder>,
        options: PageOptions,
    ) -> Result<ListMemoryStoreSortedMapItemsResponse, Error> {
        memory_store::list_sorted_map_items(
            &self.http,
            &memory_store::ListSortedMapItemsParams {
                universe_id: self.universe_id,
                sorted_map_id: sorted_map_id.to_string(),
                max_page_size: options.max_page_size,
                page_token: options.page_token,
                order,
                filter: options.filter,
            },
        )
        .await
    }

    /// Get a sorted map item.
    pub async fn sorted_map_get(
        &self,
        sorted_map_id: &str,
        item_id: &str,
    ) -> Result<MemoryStoreSortedMapItem, Error> {
        memory_store::get_sorted_map_item(
            &self.http,
            &memory_store::SortedMapItemParams {
                universe_id: self.universe_id,
                sorted_map_id: sorted_map_id.to_string(),
                item_id: item_id.to_string(),
            },
        )
        .await
    }

    /// Create or overwrite a sorted map item.
    pub async fn sorted_map_set(
        &self,
        sorted_map_id: &str,
        item_id: &str,
        value: Value,
        ttl_seconds: u64,
        sort_key: Option<SortKey>,
        etag: Option<String>,
    ) -> Result<MemoryStoreSortedMapItem, Error> {
        memory_store::update_sorted_map_item(
            &self.http,
            &memory_store::SetSortedMapItemParams {
                universe_id: self.universe_id,
                sorted_map_id: sorted_map_id.to_string(),
                item_id: item_id.to_string(),
                value,
                ttl_seconds,
                sort_key,
                etag,
                allow_missing: Some(true),
            },
        )
        .await
    }

    /// Delete a sorted map item.
    pub async fn sorted_map_delete(&self, sorted_map_id: &str, item_id: &str) -> Result<(), Error> {
        memory_store::delete_sorted_map_item(
            &self.http,
            &memory_store::SortedMapItemParams {
                universe_id: self.universe_id,
                sorted_map_id: sorted_map_id.to_string(),
                item_id: item_id.to_string(),
            },
        )
        .await
    }

    /// Start flushing all memory store data in the universe.
    pub async fn flush(
        &self,
        scope: Option<MemoryStoreScope>,
    ) -> Result<MemoryStoreFlushOperation, Error> {
        memory_store::flush(&self.http, self.universe_id, scope).await
    }

    /// Check a flush operation's status.
    pub async fn get_flush_operation(
        &self,
        operation_id: &str,
        scope: Option<MemoryStoreScope>,
    ) -> Result<MemoryStoreFlushOperation, Error> {
        memory_store::get_flush_operation(&self.http, self.universe_id, operation_id, scope).await
    }
}

/// Access to the data model of a published place (Instance API).
///
/// Methods wait for Roblox's long-running operations to finish, polling
/// according to `poll`.
#[derive(Debug, Clone)]
pub struct InstanceClient {
    http: HttpClient,
    pub universe_id: UniverseId,
    pub place_id: PlaceId,
    pub poll: PollOptions,
}

impl InstanceClient {
    fn params(&self, instance_id: &str) -> instance::InstanceParams {
        instance::InstanceParams {
            universe_id: self.universe_id,
            place_id: self.place_id,
            instance_id: instance_id.to_string(),
        }
    }

    /// Get an instance and its properties. Use [`instance::ROOT`] for the
    /// data model root.
    pub async fn get(&self, instance_id: &str) -> Result<Instance, Error> {
        let op = instance::get_instance(&self.http, &self.params(instance_id)).await?;
        operation::wait(&self.http, op, self.poll).await
    }

    /// List an instance's direct children.
    pub async fn list_children(
        &self,
        instance_id: &str,
        max_page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<ListInstanceChildrenResponse, Error> {
        let op = instance::list_children(
            &self.http,
            &self.params(instance_id),
            max_page_size,
            page_token,
        )
        .await?;
        operation::wait(&self.http, op, self.poll).await
    }

    /// Update an instance's class-specific properties.
    pub async fn update(
        &self,
        instance_id: &str,
        details: &InstanceDetails,
        update_mask: Option<String>,
    ) -> Result<Instance, Error> {
        let op =
            instance::update_instance(&self.http, &self.params(instance_id), details, update_mask)
                .await?;
        operation::wait(&self.http, op, self.poll).await
    }

    /// Replace the source of a `Script`, `LocalScript`, or `ModuleScript`.
    ///
    /// The instance is fetched first to determine its script class.
    pub async fn set_script_source(
        &self,
        instance_id: &str,
        source: String,
    ) -> Result<Instance, Error> {
        let current = self.get(instance_id).await?;
        let kind = current
            .engine_instance
            .details
            .as_ref()
            .and_then(|d| d.script_kind())
            .ok_or_else(|| {
                Error::EndpointError(format!(
                    "instance {instance_id} is not a Script, LocalScript, or ModuleScript"
                ))
            })?;
        self.update(
            instance_id,
            &InstanceDetails::with_source(kind, source),
            None,
        )
        .await
    }
}

/// Access to Creator Store products and search.
#[derive(Debug, Clone)]
pub struct CreatorStoreClient {
    http: HttpClient,
}

impl CreatorStoreClient {
    pub async fn get_product(&self, product_id: &str) -> Result<CreatorStoreProduct, Error> {
        creator_store::get_product(&self.http, product_id).await
    }

    /// Distribute an asset on the Creator Store for the first time.
    pub async fn create_product(
        &self,
        product: &CreatorStoreProduct,
    ) -> Result<CreatorStoreProduct, Error> {
        creator_store::create_product(&self.http, product).await
    }

    /// Update a product. See [`creator_store::update_product`].
    pub async fn update_product(
        &self,
        product_id: &str,
        product: &CreatorStoreProduct,
        update_mask: Option<String>,
        allow_missing: Option<bool>,
    ) -> Result<CreatorStoreProduct, Error> {
        creator_store::update_product(&self.http, product_id, product, update_mask, allow_missing)
            .await
    }

    pub async fn search(
        &self,
        params: &creator_store::SearchParams,
    ) -> Result<SearchCreatorStoreAssetsResponse, Error> {
        creator_store::search(&self.http, params).await
    }

    pub async fn get_asset(&self, asset_id: u64) -> Result<CreatorStoreAsset, Error> {
        creator_store::get_asset(&self.http, asset_id).await
    }
}

/// Access to a universe's secrets store.
#[derive(Debug, Clone)]
pub struct SecretClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

impl SecretClient {
    pub async fn list(
        &self,
        limit: Option<u32>,
        cursor: Option<String>,
    ) -> Result<ListSecretsResponse, Error> {
        secret::list_secrets(&self.http, self.universe_id, limit, cursor).await
    }

    /// The universe's public key, used to encrypt secret values.
    pub async fn public_key(&self) -> Result<UniversePublicKey, Error> {
        secret::get_public_key(&self.http, self.universe_id).await
    }

    /// Create a secret; the value is encrypted locally before sending.
    pub async fn create(
        &self,
        secret_id: &str,
        value: &[u8],
        domain: Option<&str>,
    ) -> Result<Secret, Error> {
        secret::create_secret(&self.http, self.universe_id, secret_id, value, domain).await
    }

    /// Replace a secret's value; encrypted locally before sending.
    pub async fn update(
        &self,
        secret_id: &str,
        value: &[u8],
        domain: Option<&str>,
    ) -> Result<Secret, Error> {
        secret::update_secret(&self.http, self.universe_id, secret_id, value, domain).await
    }

    pub async fn delete(&self, secret_id: &str) -> Result<(), Error> {
        secret::delete_secret(&self.http, self.universe_id, secret_id).await
    }
}

/// Access to a universe's game passes.
#[derive(Debug, Clone)]
pub struct GamePassClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

impl GamePassClient {
    pub async fn list(
        &self,
        page_size: Option<u32>,
        page_token: Option<String>,
        for_sale_only: Option<bool>,
    ) -> Result<ListGamePassesResponse, Error> {
        game_pass::list_game_passes(
            &self.http,
            self.universe_id,
            page_size,
            page_token,
            for_sale_only,
        )
        .await
    }

    pub async fn get(&self, game_pass_id: u64) -> Result<GamePass, Error> {
        game_pass::get_game_pass(&self.http, self.universe_id, game_pass_id).await
    }

    pub async fn create(&self, fields: &GamePassFields) -> Result<GamePass, Error> {
        game_pass::create_game_pass(&self.http, self.universe_id, fields).await
    }

    pub async fn update(&self, game_pass_id: u64, fields: &GamePassFields) -> Result<(), Error> {
        game_pass::update_game_pass(&self.http, self.universe_id, game_pass_id, fields).await
    }
}

/// Access to a universe's developer products.
#[derive(Debug, Clone)]
pub struct DeveloperProductClient {
    http: HttpClient,
    pub universe_id: UniverseId,
}

impl DeveloperProductClient {
    pub async fn list(
        &self,
        page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<ListDeveloperProductsResponse, Error> {
        developer_product::list_developer_products(
            &self.http,
            self.universe_id,
            page_size,
            page_token,
        )
        .await
    }

    pub async fn get(&self, product_id: u64) -> Result<DeveloperProduct, Error> {
        developer_product::get_developer_product(&self.http, self.universe_id, product_id).await
    }

    pub async fn create(&self, fields: &DeveloperProductFields) -> Result<DeveloperProduct, Error> {
        developer_product::create_developer_product(&self.http, self.universe_id, fields).await
    }

    pub async fn update(
        &self,
        product_id: u64,
        fields: &DeveloperProductFields,
    ) -> Result<(), Error> {
        developer_product::update_developer_product(
            &self.http,
            self.universe_id,
            product_id,
            fields,
        )
        .await
    }
}

impl GroupClient {
    pub async fn get_info(&self) -> Result<GetGroupResponse, Error> {
        group::get_group(
            &self.http,
            &GetGroupParams {
                group_id: self.group_id,
            },
        )
        .await
    }

    pub async fn get_shout(&self) -> Result<GetGroupShoutResponse, Error> {
        group::get_group_shout(
            &self.http,
            &GetGroupShoutParams {
                group_id: self.group_id,
            },
        )
        .await
    }

    pub async fn list_roles(
        &self,
        max_page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<ListGroupRolesResponse, Error> {
        group::list_group_roles(
            &self.http,
            &ListGroupRolesParams {
                group_id: self.group_id,
                max_page_size,
                page_token,
            },
        )
        .await
    }

    pub async fn list_memberships(
        &self,
        max_page_size: Option<u32>,
        filter: Option<String>,
        page_token: Option<String>,
    ) -> Result<ListGroupMembershipsResponse, Error> {
        group::list_group_memberships(
            &self.http,
            &ListGroupMembershipsParams {
                group_id: self.group_id,
                max_page_size,
                page_token,
                filter,
            },
        )
        .await
    }

    /// List pending join requests.
    pub async fn list_join_requests(
        &self,
        params: &GroupListParams,
    ) -> Result<ListGroupJoinRequestsResponse, Error> {
        group::list_join_requests(&self.http, self.group_id, params).await
    }

    pub async fn accept_join_request(&self, join_request_id: &str) -> Result<(), Error> {
        group::accept_join_request(&self.http, self.group_id, join_request_id).await
    }

    pub async fn decline_join_request(&self, join_request_id: &str) -> Result<(), Error> {
        group::decline_join_request(&self.http, self.group_id, join_request_id).await
    }

    pub async fn get_role(&self, role_id: u64) -> Result<GroupRole, Error> {
        group::get_role(&self.http, self.group_id, role_id).await
    }

    /// Give a member a role. `membership_id` is the last segment of the
    /// membership's `path`.
    pub async fn assign_role(
        &self,
        membership_id: &str,
        role_id: u64,
    ) -> Result<GroupMembership, Error> {
        group::assign_role(&self.http, self.group_id, membership_id, role_id).await
    }

    pub async fn unassign_role(
        &self,
        membership_id: &str,
        role_id: u64,
    ) -> Result<GroupMembership, Error> {
        group::unassign_role(&self.http, self.group_id, membership_id, role_id).await
    }

    /// Find a user's membership in this group, if they are a member.
    pub async fn find_membership(
        &self,
        user_id: RobloxUserId,
    ) -> Result<Option<GroupMembership>, Error> {
        let res = self
            .list_memberships(Some(1), Some(format!("user == 'users/{user_id}'")), None)
            .await?;
        Ok(res.group_memberships.into_iter().next())
    }

    pub async fn list_forum_categories(
        &self,
        params: &GroupListParams,
    ) -> Result<ListGroupForumCategoriesResponse, Error> {
        group::list_forum_categories(&self.http, self.group_id, params).await
    }

    pub async fn list_forum_posts(
        &self,
        category_id: &str,
        params: &GroupListParams,
    ) -> Result<ListGroupForumPostsResponse, Error> {
        group::list_forum_posts(&self.http, self.group_id, category_id, params).await
    }

    pub async fn list_forum_comments(
        &self,
        category_id: &str,
        post_id: &str,
        params: &GroupListParams,
    ) -> Result<ListGroupForumCommentsResponse, Error> {
        group::list_forum_comments(&self.http, self.group_id, category_id, post_id, params).await
    }
}

impl InventoryClient {
    pub async fn list_inventory_items(
        &self,
        user_id: RobloxUserId,
        max_page_size: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
    ) -> Result<InventoryItems, Error> {
        inventory::list_inventory_items(
            &self.http,
            &ListInventoryItemsParams {
                user_id,
                max_page_size,
                page_token,
                filter,
            },
        )
        .await
    }
}

impl LuauExecutionClient {
    pub async fn create_task(
        &self,
        script: String,
        timeout: Option<String>,
    ) -> Result<NewLuauExecutionSessionTask, Error> {
        luau_execution::create_luau_execution_task(
            &self.http,
            &CreateLuauExecutionTaskParams {
                universe_id: self.universe_id,
                place_id: self.place_id,
                version_id: self.version_id.clone(),
                script,
                timeout,
                binary_input: None,
                enable_binary_output: false,
            },
        )
        .await
    }

    /// Create a task with a binary input (uploaded first) and/or binary
    /// output enabled.
    pub async fn create_task_with_binary(
        &self,
        script: String,
        timeout: Option<String>,
        binary_input: Option<Vec<u8>>,
        enable_binary_output: bool,
    ) -> Result<NewLuauExecutionSessionTask, Error> {
        let binary_input = match binary_input {
            Some(data) => Some(
                luau_execution::upload_binary_input(&self.http, self.universe_id, data)
                    .await?
                    .path,
            ),
            None => None,
        };
        luau_execution::create_luau_execution_task(
            &self.http,
            &CreateLuauExecutionTaskParams {
                universe_id: self.universe_id,
                place_id: self.place_id,
                version_id: self.version_id.clone(),
                script,
                timeout,
                binary_input,
                enable_binary_output,
            },
        )
        .await
    }

    /// Wait for a task (e.g. one returned by `create_task`) to finish,
    /// polling every second for up to 5 minutes.
    pub async fn wait(
        &self,
        task: LuauExecutionSessionTask,
    ) -> Result<LuauExecutionSessionTask, Error> {
        luau_execution::wait_for_luau_execution_task(
            &self.http,
            task,
            PollOptions {
                interval: std::time::Duration::from_secs(1),
                max_attempts: 300,
            },
        )
        .await
    }

    /// Download a finished task's binary output.
    pub async fn download_binary_output(
        &self,
        task: &LuauExecutionSessionTask,
    ) -> Result<Vec<u8>, Error> {
        let uri = task
            .binary_output_uri
            .as_deref()
            .ok_or_else(|| Error::EndpointError("task has no binary output".to_string()))?;
        luau_execution::download_binary_output(&self.http, uri).await
    }

    pub async fn get_task(
        &self,
        session_id: String,
        task_id: String,
    ) -> Result<LuauExecutionSessionTask, Error> {
        luau_execution::get_luau_execution_task(
            &self.http,
            &GetLuauExecutionSessionTaskParams {
                universe_id: self.universe_id,
                place_id: self.place_id,
                version_id: self.version_id.clone(),
                session_id,
                task_id,
            },
        )
        .await
    }

    pub async fn get_logs(
        &self,
        session_id: String,
        task_id: String,
        view: LuauExecutionTaskLogView,
        max_page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<LuauExecutionSessionTaskLogPage, Error> {
        luau_execution::get_luau_execution_task_logs(
            &self.http,
            &GetLuauExecutionSessionTaskLogsParams {
                universe_id: self.universe_id,
                place_id: self.place_id,
                version_id: self.version_id.clone(),
                session_id,
                task_id,
                view,
                max_page_size,
                page_token,
            },
        )
        .await
    }
}

impl SubscriptionClient {
    pub async fn get(
        &self,
        universe_id: UniverseId,
        subscription_product: String,
        subscription: String,
        view: Option<SubscriptionView>,
    ) -> Result<GetSubscriptionResponse, Error> {
        subscription::get_subscription(
            &self.http,
            &GetSubscriptionParams {
                universe_id,
                subscription,
                subscription_product,
                view,
            },
        )
        .await
    }
}

impl NotificationClient {
    pub async fn send(
        &self,
        user_id: RobloxUserId,
        notification: Notification,
    ) -> Result<NotificationResponse, Error> {
        notification::send_notification(
            &self.http,
            &NotificationParams {
                user_id,
                notification,
            },
        )
        .await
    }
}

impl PlaceClient {
    pub async fn get(&self) -> Result<PlaceInfo, Error> {
        place::get_place(
            &self.http,
            &GetPlaceParams {
                universe_id: self.universe_id,
                place_id: self.place_id,
            },
        )
        .await
    }

    pub async fn update(
        &self,
        update_mask: String,
        info: UpdatePlaceInfo,
    ) -> Result<PlaceInfo, Error> {
        place::update_place(
            &self.http,
            &UpdatePlaceParams {
                universe_id: self.universe_id,
                place_id: self.place_id,
                update_mask,
                info,
            },
        )
        .await
    }
}

impl UniverseClient {
    pub async fn get(&self) -> Result<UniverseInfo, Error> {
        universe::get_universe(
            &self.http,
            &GetUniverseParams {
                universe_id: self.universe_id,
            },
        )
        .await
    }

    pub async fn update(
        &self,
        update_mask: String,
        info: UpdateUniverseInfo,
    ) -> Result<UniverseInfo, Error> {
        universe::update_universe(
            &self.http,
            &UpdateUniverseParams {
                universe_id: self.universe_id,
                update_mask,
                info,
            },
        )
        .await
    }

    /// Publish a message to live servers subscribed to `topic`.
    pub async fn publish_message(&self, topic: &str, message: &str) -> Result<(), Error> {
        universe::publish_message(
            &self.http,
            &PublishUniverseMessageParams {
                universe_id: self.universe_id,
                topic: topic.to_string(),
                message: message.to_string(),
            },
        )
        .await
    }

    pub async fn restart_servers(&self) -> Result<(), Error> {
        universe::restart_universe_servers(
            &self.http,
            &RestartUniverseServersParams {
                universe_id: self.universe_id,
            },
        )
        .await
    }
}

impl UserClient {
    /// List a user's asset quotas (e.g. monthly audio upload limits).
    pub async fn list_asset_quotas(
        &self,
        user_id: RobloxUserId,
        filter: Option<String>,
        max_page_size: Option<u32>,
        page_token: Option<String>,
    ) -> Result<ListAssetQuotasResponse, Error> {
        user::list_asset_quotas(&self.http, user_id, filter, max_page_size, page_token).await
    }

    pub async fn get_user(&self, user_id: RobloxUserId) -> Result<GetUserResponse, Error> {
        user::get_user(&self.http, &GetUserParams { user_id }).await
    }

    pub async fn generate_thumbnail(
        &self,
        user_id: RobloxUserId,
        size: Option<UserThumbnailSize>,
        format: Option<UserThumbnailFormat>,
        shape: Option<UserThumbnailShape>,
    ) -> Result<GenerateUserThumbnailOperationResponse, Error> {
        user::generate_thumbnail(
            &self.http,
            &GenerateUserThumbnailParams {
                user_id,
                size,
                shape,
                format,
            },
        )
        .await
    }
}

impl UserRestrictionClient {
    pub async fn list_user_restrictions(
        &self,
        place_id: Option<PlaceId>,
        max_page_size: Option<u32>,
        filter: Option<String>,
        page_token: Option<String>,
    ) -> Result<UserRestrictionList, Error> {
        user_restriction::list_user_restrictions(
            &self.http,
            &ListUserRestrictionsParams {
                universe_id: self.universe_id,
                place_id,
                max_page_size,
                page_token,
                filter,
            },
        )
        .await
    }

    pub async fn get_user_restriction(
        &self,
        user_id: RobloxUserId,
        place_id: Option<PlaceId>,
    ) -> Result<UserRestriction, Error> {
        user_restriction::get_user_restriction(
            &self.http,
            &GetUserRestrictionParams {
                universe_id: self.universe_id,
                place_id,
                user_id,
            },
        )
        .await
    }

    pub async fn update_user_restriction(
        &mut self,
        params: &UserRestrictionParams,
    ) -> Result<UserRestriction, Error> {
        let idempotency_key: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(32)
            .map(char::from)
            .collect();
        user_restriction::update_user_restriction(
            &self.http,
            &UpdateUserRestrictionParams {
                universe_id: self.universe_id,
                place_id: params.place_id,
                user_id: params.user_id,
                idempotency_key: Some(idempotency_key),
                active: params.active,
                duration: params.duration.map(|d| format!("{}s", d)),
                private_reason: params.private_reason.clone(),
                display_reason: params.display_reason.clone(),
                exclude_alt_accounts: params.exclude_alt_accounts,
            },
        )
        .await
    }

    pub async fn list_user_restriction_logs(
        &self,
        place_id: Option<PlaceId>,
        max_page_size: Option<u32>,
        page_token: Option<String>,
        filter: Option<String>,
    ) -> Result<UserRestrictionLogsList, Error> {
        user_restriction::list_user_restriction_logs(
            &self.http,
            &ListUserRestrictionLogsParams {
                universe_id: self.universe_id,
                place_id,
                max_page_size,
                page_token,
                filter,
            },
        )
        .await
    }
}

impl Client {
    /// Create a client with default HTTP settings.
    pub fn new(api_key: &str) -> Client {
        Client {
            http: HttpClient::new(api_key),
        }
    }

    /// Create a client from a pre-configured [`HttpClient`] (custom
    /// timeouts, proxy, etc).
    pub fn with_http(http: HttpClient) -> Client {
        Client { http }
    }

    /// The underlying HTTP client.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Data stores (v2) for a universe.
    pub fn data_store(&self, universe_id: UniverseId) -> DataStoreClient {
        DataStoreClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    /// Ordered data stores (v2) for a universe.
    pub fn ordered_data_store(&self, universe_id: UniverseId) -> OrderedDataStoreClient {
        OrderedDataStoreClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    /// Memory stores for a universe.
    pub fn memory_store(&self, universe_id: UniverseId) -> MemoryStoreClient {
        MemoryStoreClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    /// The data model of a published place (Instance API).
    pub fn instance(&self, universe_id: UniverseId, place_id: PlaceId) -> InstanceClient {
        InstanceClient {
            http: self.http.clone(),
            universe_id,
            place_id,
            poll: PollOptions::default(),
        }
    }

    /// Creator Store products and search.
    pub fn creator_store(&self) -> CreatorStoreClient {
        CreatorStoreClient {
            http: self.http.clone(),
        }
    }

    /// A universe's secrets store.
    pub fn secrets(&self, universe_id: UniverseId) -> SecretClient {
        SecretClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    /// A universe's game passes.
    pub fn game_passes(&self, universe_id: UniverseId) -> GamePassClient {
        GamePassClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    /// A universe's developer products.
    pub fn developer_products(&self, universe_id: UniverseId) -> DeveloperProductClient {
        DeveloperProductClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    pub fn group(&self, group_id: GroupId) -> GroupClient {
        GroupClient {
            http: self.http.clone(),
            group_id,
        }
    }

    pub fn inventory(&self) -> InventoryClient {
        InventoryClient {
            http: self.http.clone(),
        }
    }

    pub fn luau(
        &self,
        universe_id: UniverseId,
        place_id: PlaceId,
        version_id: Option<String>,
    ) -> LuauExecutionClient {
        LuauExecutionClient {
            http: self.http.clone(),
            universe_id,
            place_id,
            version_id,
        }
    }

    pub fn subscription(&self) -> SubscriptionClient {
        SubscriptionClient {
            http: self.http.clone(),
        }
    }

    pub fn notification(&self, universe_id: UniverseId) -> NotificationClient {
        NotificationClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    pub fn place(&self, universe_id: UniverseId, place_id: PlaceId) -> PlaceClient {
        PlaceClient {
            http: self.http.clone(),
            universe_id,
            place_id,
        }
    }

    pub fn universe(&self, universe_id: UniverseId) -> UniverseClient {
        UniverseClient {
            http: self.http.clone(),
            universe_id,
        }
    }

    pub fn user(&self) -> UserClient {
        UserClient {
            http: self.http.clone(),
        }
    }

    pub fn user_restriction(&self, universe_id: UniverseId) -> UserRestrictionClient {
        UserRestrictionClient {
            http: self.http.clone(),
            universe_id,
        }
    }
}
