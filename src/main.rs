use anyhow::Result;
use serde::Deserialize;

const MEMPOOL_API_URL: &str = "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

#[tokio::main]
async fn main() -> Result<()> {
    let nodes = reqwest::get(MEMPOOL_API_URL)
        .await?
        .json::<Vec<NodeConnectivity>>()
        .await?;

    nodes.iter().for_each(|node| {
        println!(
            "alias: {} ({}), capacity: {}, seen: {}, updated: {}",
            node.alias, node.public_key, node.capacity, node.first_seen, node.updated_at
        );
    });
    Ok(())
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeConnectivity {
    pub public_key: String,
    pub alias: String,
    pub capacity: u64,
    pub first_seen: u64,
    pub updated_at: i64,
}
