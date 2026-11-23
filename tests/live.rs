//! Live tests against the real Roblox Open Cloud API.
//!
//! These are `#[ignore]`d so `cargo test` stays offline. Run them with
//! `mise run test:live` after configuring a test experience; see the "Live
//! tests" section of CONTRIBUTING.md.
//!
//! Configuration (environment variables):
//! - `RBXCLOUD_API_KEY` (required)
//! - `RBXCLOUD_TEST_UNIVERSE_ID`, `RBXCLOUD_TEST_PLACE_ID`,
//!   `RBXCLOUD_TEST_GROUP_ID`, `RBXCLOUD_TEST_USER_ID` (each test skips
//!   itself if an ID it needs is missing)
//! - `RBXCLOUD_TEST_WRITE=1` to also run tests that create and then delete
//!   data. They only touch keys named `rbxcloud-live-<run id>`.

use std::sync::OnceLock;

use rbxcloud::{
    buffer,
    error::Error,
    types::{GroupId, PlaceId, RobloxUserId, UniverseId},
    v2::{
        creator_store::{SearchCategory, SearchParams},
        instance::ROOT,
        luau_execution::LuauExecutionOutcome,
        memory_store::SortKey,
        Client, EntryMetadata, PageOptions,
    },
};
use serde_json::json;

struct Live {
    client: Client,
    universe: Option<UniverseId>,
    place: Option<PlaceId>,
    group: Option<GroupId>,
    user: Option<RobloxUserId>,
    write: bool,
    run_id: String,
}

fn env_id(name: &str) -> Option<u64> {
    let value = std::env::var(name).ok()?;
    Some(
        value
            .trim()
            .parse()
            .unwrap_or_else(|_| panic!("{name} must be a number, got {value:?}")),
    )
}

fn live() -> &'static Live {
    static LIVE: OnceLock<Live> = OnceLock::new();
    LIVE.get_or_init(|| {
        let api_key = std::env::var("RBXCLOUD_API_KEY")
            .expect("live tests need RBXCLOUD_API_KEY (see CONTRIBUTING.md, \"Live tests\")");
        Live {
            client: Client::new(&api_key),
            universe: env_id("RBXCLOUD_TEST_UNIVERSE_ID").map(UniverseId),
            place: env_id("RBXCLOUD_TEST_PLACE_ID").map(PlaceId),
            group: env_id("RBXCLOUD_TEST_GROUP_ID").map(GroupId),
            user: env_id("RBXCLOUD_TEST_USER_ID").map(RobloxUserId),
            write: std::env::var("RBXCLOUD_TEST_WRITE").is_ok_and(|v| v == "1"),
            run_id: format!(
                "{}-{:04x}",
                chrono::Utc::now().format("%Y%m%d%H%M%S"),
                rand::random::<u16>()
            ),
        }
    })
}

/// Skip the current test (with a message) unless `$value` is set.
macro_rules! need {
    ($value:expr, $var:literal) => {
        match $value {
            Some(v) => v,
            None => {
                eprintln!("skipped: set {}", $var);
                return;
            }
        }
    };
}

/// Skip the current test unless writes are enabled.
macro_rules! need_write {
    ($live:expr) => {
        if !$live.write {
            eprintln!("skipped: set RBXCLOUD_TEST_WRITE=1 to run tests that modify data");
            return;
        }
    };
}

fn is_not_found(err: &Error) -> bool {
    matches!(err, Error::HttpStatusError { code: 404, .. })
}

// ------------------------------------------------------------------ reads

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn universe_and_place() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let info = l.client.universe(universe).get().await.unwrap();
    assert!(
        info.path.ends_with(&universe.0.to_string()),
        "{}",
        info.path
    );

    let place = need!(l.place, "RBXCLOUD_TEST_PLACE_ID");
    let info = l.client.place(universe, place).get().await.unwrap();
    assert!(info.path.ends_with(&place.0.to_string()), "{}", info.path);
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn user_and_asset_quotas() {
    let l = live();
    let user = need!(l.user, "RBXCLOUD_TEST_USER_ID");
    let info = l.client.user().get_user(user).await.unwrap();
    assert_eq!(info.id, user.0.to_string());

    let quotas = l
        .client
        .user()
        .list_asset_quotas(user, None, None, None)
        .await
        .unwrap();
    eprintln!("{} asset quotas", quotas.asset_quotas.len());
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn group_reads() {
    let l = live();
    let group_id = need!(l.group, "RBXCLOUD_TEST_GROUP_ID");
    let group = l.client.group(group_id);

    let info = group.get_info().await.unwrap();
    assert_eq!(info.id, group_id.0.to_string());

    let roles = group.list_roles(Some(10), None).await.unwrap();
    let first = roles.group_roles.first().expect("group has no roles");
    let role = group.get_role(first.id.parse().unwrap()).await.unwrap();
    assert_eq!(role.id, first.id);

    group.list_memberships(Some(1), None, None).await.unwrap();
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn list_data_stores() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let res = l
        .client
        .data_store(universe)
        .list_data_stores(PageOptions {
            max_page_size: Some(5),
            ..Default::default()
        })
        .await
        .unwrap();
    eprintln!("{} data stores on first page", res.data_stores.len());
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn instance_tree() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let place = need!(l.place, "RBXCLOUD_TEST_PLACE_ID");
    let instances = l.client.instance(universe, place);

    instances.get(ROOT).await.unwrap();
    let children = instances.list_children(ROOT, None, None).await.unwrap();
    let names: Vec<_> = children
        .instances
        .iter()
        .filter_map(|i| i.engine_instance.name.as_deref())
        .collect();
    assert!(names.contains(&"Workspace"), "root children: {names:?}");
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn monetization_lists() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    l.client
        .game_passes(universe)
        .list(Some(5), None, None)
        .await
        .unwrap();
    l.client
        .developer_products(universe)
        .list(Some(5), None)
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn list_secrets() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    l.client
        .secrets(universe)
        .list(Some(10), None)
        .await
        .unwrap();
    // Also checks the public key decodes into a usable encryption key.
    l.client.secrets(universe).public_key().await.unwrap();
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn creator_store_search() {
    let l = live();
    let res = l
        .client
        .creator_store()
        .search(&SearchParams {
            query: Some("tree".into()),
            category: Some(SearchCategory::Model),
            max_page_size: Some(1),
            ..Default::default()
        })
        .await
        .unwrap();
    assert!(res.total_results > 0, "no results for a common search term");
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn luau_execution() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let place = need!(l.place, "RBXCLOUD_TEST_PLACE_ID");
    let luau = l.client.luau(universe, place, None);

    let task = luau
        .create_task("return 1 + 1".into(), Some("30s".into()))
        .await
        .unwrap();
    let task = luau.wait(task).await.unwrap();
    match task.outcome() {
        LuauExecutionOutcome::Complete(results) => assert_eq!(results, [json!(2)]),
        other => panic!("unexpected outcome: {other:?}"),
    }

    let task = luau
        .create_task("error('expected failure')".into(), Some("30s".into()))
        .await
        .unwrap();
    let task = luau.wait(task).await.unwrap();
    assert!(
        matches!(task.outcome(), LuauExecutionOutcome::Failed(_)),
        "a failing script should report Failed (issue #78)"
    );
}

#[tokio::test]
#[ignore = "live: calls the Roblox API"]
async fn luau_binary_input() {
    let l = live();
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let place = need!(l.place, "RBXCLOUD_TEST_PLACE_ID");
    let luau = l.client.luau(universe, place, None);

    let task = luau
        .create_task_with_binary(
            "return buffer.len(({...})[1].BinaryInput)".into(),
            Some("30s".into()),
            Some(b"hello".to_vec()),
            false,
        )
        .await
        .unwrap();
    let task = luau.wait(task).await.unwrap();
    match task.outcome() {
        LuauExecutionOutcome::Complete(results) => assert_eq!(results, [json!(5)]),
        other => panic!("unexpected outcome: {other:?}"),
    }
}

// ----------------------------------------------------------------- writes

#[tokio::test]
#[ignore = "live: modifies data"]
async fn data_store_roundtrip() {
    let l = live();
    need_write!(l);
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let ds = l.client.data_store(universe);
    let store = "rbxcloud-live";
    let key = format!("rbxcloud-live-{}", l.run_id);
    let counter = format!("{key}-counter");

    let value = json!({"n": 1, "blob": buffer::encode(b"hi")});
    ds.set_entry(store, None, &key, value, None, EntryMetadata::default())
        .await
        .unwrap();
    let entry = ds.get_entry(store, None, &key).await.unwrap();
    let value = entry.value.expect("entry has no value");
    assert_eq!(value["n"], 1);
    assert_eq!(
        buffer::decode(&value["blob"]).unwrap().as_deref(),
        Some(&b"hi"[..])
    );

    let revisions = ds
        .list_entry_revisions(store, None, &key, PageOptions::default())
        .await
        .unwrap();
    assert!(!revisions.data_store_entries.is_empty());

    for _ in 0..2 {
        ds.increment_entry(store, None, &counter, 2.0, EntryMetadata::default())
            .await
            .unwrap();
    }
    let total = ds.get_entry(store, None, &counter).await.unwrap().value;
    assert_eq!(total.and_then(|v| v.as_f64()), Some(4.0));

    ds.delete_entry(store, None, &key).await.unwrap();
    ds.delete_entry(store, None, &counter).await.unwrap();
    let err = ds.get_entry(store, None, &key).await.unwrap_err();
    assert!(is_not_found(&err), "expected 404 after delete, got {err}");
}

#[tokio::test]
#[ignore = "live: modifies data"]
async fn ordered_data_store_roundtrip() {
    let l = live();
    need_write!(l);
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let ods = l.client.ordered_data_store(universe);
    let store = "rbxcloud-live";
    let id = format!("rbxcloud-live-{}", l.run_id);

    ods.create_entry(store, None, &id, 10).await.unwrap();
    let entry = ods.increment_entry(store, None, &id, 5).await.unwrap();
    assert_eq!(entry.value, 15);
    let entry = ods.update_entry(store, None, &id, 3, None).await.unwrap();
    assert_eq!(entry.value, 3);
    assert_eq!(ods.get_entry(store, None, &id).await.unwrap().value, 3);
    ods.delete_entry(store, None, &id).await.unwrap();
}

#[tokio::test]
#[ignore = "live: modifies data"]
async fn memory_store_queue() {
    let l = live();
    need_write!(l);
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let ms = l.client.memory_store(universe);
    let queue = format!("rbxcloud-live-{}", l.run_id);

    ms.queue_add(&queue, json!({"job": 1}), None, Some(60))
        .await
        .unwrap();
    let read = ms
        .queue_read(&queue, Some(1), None, Some(30))
        .await
        .unwrap();
    assert_eq!(read.queue_items.len(), 1);
    assert_eq!(read.queue_items[0].data, Some(json!({"job": 1})));
    ms.queue_discard(&queue, &read.id).await.unwrap();
}

#[tokio::test]
#[ignore = "live: modifies data"]
async fn memory_store_sorted_map() {
    let l = live();
    need_write!(l);
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let ms = l.client.memory_store(universe);
    let map = format!("rbxcloud-live-{}", l.run_id);

    ms.sorted_map_set(
        &map,
        "a",
        json!({"x": 1}),
        60,
        Some(SortKey::Numeric(1.0)),
        None,
    )
    .await
    .unwrap();
    let item = ms.sorted_map_get(&map, "a").await.unwrap();
    assert_eq!(item.value, Some(json!({"x": 1})));
    let list = ms
        .sorted_map_list(
            &map,
            None,
            PageOptions {
                max_page_size: Some(10),
                ..Default::default()
            },
        )
        .await
        .unwrap();
    assert_eq!(list.items.len(), 1);
    ms.sorted_map_delete(&map, "a").await.unwrap();
}

#[tokio::test]
#[ignore = "live: modifies data"]
async fn publish_message() {
    let l = live();
    need_write!(l);
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    l.client
        .universe(universe)
        .publish_message("rbxcloud-live", "hello from rbxcloud live tests")
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "live: modifies data"]
async fn secret_roundtrip() {
    let l = live();
    need_write!(l);
    let universe = need!(l.universe, "RBXCLOUD_TEST_UNIVERSE_ID");
    let secrets = l.client.secrets(universe);
    let id = format!("rbxcloud_live_{}", l.run_id.replace('-', "_"));

    secrets.create(&id, b"first", None).await.unwrap();
    secrets.update(&id, b"second", None).await.unwrap();
    let listed = secrets.list(Some(100), None).await.unwrap();
    assert!(
        listed.secrets.iter().any(|s| s.id == id),
        "created secret not listed"
    );
    secrets.delete(&id).await.unwrap();
}
