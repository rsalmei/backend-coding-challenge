use anyhow::Result;
use serde::Deserialize;

/// The Mempool Space API endpoint for fetching Lightning Network nodes connectivity data.
const MP_NODES_CONNECTIVITY_API: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

#[tokio::main]
async fn main() -> Result<()> {
    let nodes = fetch_nodes_connectivity().await?;

    nodes.iter().for_each(|node| {
        println!(
            "alias: {} ({}), capacity: {}, seen: {}, updated: {}",
            node.alias, node.public_key, node.capacity, node.first_seen, node.updated_at
        );
    });
    Ok(())
}

/// Fetches the connectivity data of Lightning Network nodes from the Mempool API.
async fn fetch_nodes_connectivity() -> Result<Vec<NodeConnectivity>> {
    reqwest::get(MP_NODES_CONNECTIVITY_API)
        .await?
        .json()
        .await
        .map_err(Into::into)
}

/// Represents the connectivity information of a Lightning Network node (a subset of the full data).
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NodeConnectivity {
    pub public_key: String,
    pub alias: String,
    pub capacity: u64,
    pub first_seen: u64,
    pub updated_at: i64,
}
