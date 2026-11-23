use std::time::Duration;

use rbxcloud::{
    buffer,
    error::Error,
    http::HttpClient,
    types::UniverseId,
    v2::{Client, EntryMetadata},
};
use serde_json::json;

async fn run() -> Result<(), Error> {
    // Inputs:
    let api_key = "MY_API_KEY";
    let universe_id = UniverseId(9876543210);

    // Configure the HTTP client (timeouts, proxy) once and share it.
    let http = HttpClient::builder(api_key)
        .timeout(Duration::from_secs(30))
        .build()?;
    let client = Client::with_http(http);
    let data_stores = client.data_store(universe_id);

    // Write an entry, including a Luau buffer value:
    let value = json!({
        "coins": 100,
        "map": buffer::encode(&[1, 2, 3, 4]),
    });
    data_stores
        .set_entry(
            "PlayerData",
            None,
            "user_1",
            value,
            None,
            EntryMetadata::default(),
        )
        .await?;

    // Read it back and decode the buffer:
    let entry = data_stores.get_entry("PlayerData", None, "user_1").await?;
    if let Some(value) = &entry.value {
        let bytes = buffer::decode(&value["map"])?;
        println!("coins = {}, map = {:?}", value["coins"], bytes);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("{e}");
    }
}
