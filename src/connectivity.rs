use anyhow::Result;
use serde::{Deserialize, Serialize};
use surrealdb::Surreal;
use surrealdb::engine::any::Any;
use tracing::info;

/// The Mempool Space API endpoint for fetching Lightning Network nodes connectivity data.
const NODES_CONNECTIVITY_API: &str =
    "https://mempool.space/api/v1/lightning/nodes/rankings/connectivity";

/// Represents the connectivity information of a Lightning Network node (a subset of the full data).
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all(deserialize = "camelCase"))]
struct MempoolNodeConnectivity {
    public_key: String,
    alias: String,
    capacity: u64,
    first_seen: i64,
    updated_at: i64,
}

/// Represents the connectivity information of a Lightning Network node (a subset of the full data).
#[derive(Debug, Deserialize, Serialize)]
pub struct NodeConnectivity {
    pub public_key: String,
    pub alias: String,
    pub capacity: u64,
    pub first_seen: i64,
    pub updated_at: i64,
}

impl From<MempoolNodeConnectivity> for NodeConnectivity {
    fn from(mempool_node: MempoolNodeConnectivity) -> Self {
        Self {
            public_key: mempool_node.public_key,
            alias: mempool_node.alias,
            capacity: mempool_node.capacity,
            first_seen: mempool_node.first_seen,
            updated_at: mempool_node.updated_at,
        }
    }
}

/// Updates the local database with the latest connectivity data of Lightning Network nodes.
pub async fn update_nodes_connectivity_task(db: Surreal<Any>) -> Result<()> {
    let nodes = fetch_nodes_connectivity().await?;
    if nodes.is_empty() {
        anyhow::bail!("fetched zero nodes connectivity data from API");
    }

    // yes, I do use unwrap in production code, but very consciously. I even always comment why it's
    // safe to do so like it was an unsafe block. This reassures code reviewers and future me that
    // it's not an oversight but a deliberate choice, that has been thought through and is
    // guaranteed to be safe. I do this for any functions that may panic, like unwraps, arrays
    // indexing, Vec::swap_remove, etc.

    // I consider panics to be embarrassing, so no software of mine will ever panic, at most they
    // will gracefully shut down or degrade functionality.
    // I also use unwraps a lot at compile time, like for LazyLock initializers with Regexes using
    // fixed patterns, that must be correct or are a programming error that can only be fixed by the
    // developer (me :-). But I wouldn't ever use any of those in runtime code, that depends on the
    // environment or user input. It seems easy to discern between the two at first, but it requires
    // a good deal of experience to always get it right.

    // also notice that this field is being used here only for logging purposes, it won't even be
    // returned by the API; but I consider very important to store updated_at in the database for a
    // plethora of reasons, like debugging, data staleness checks, auditing, etc.
    let max_updated_at = nodes.iter().map(|node| node.updated_at).max().unwrap(); // SAFETY: nodes is not empty.
    info!("fetched new nodes connectivity data, max_updated_at: {max_updated_at}");

    // upsert each node's connectivity data into the database.
    // this is necessary because they are ranked by connectivity quality (number of open channels),
    // and the API clips at a fixed number of 100 nodes; this means the returned nodes are likely to
    // change over time, and thus we need to update already seen ones, as well as insert the others.
    for node in nodes {
        // the query notation does not suffer from the mandatory return type of the upsert method.
        db.query("UPSERT ln_node_connectivity CONTENT $node RETURN NONE")
            .bind(("node", node))
            .await?;
    }

    Ok(())
}

/// Fetches the connectivity data of Lightning Network nodes from the Mempool API.
async fn fetch_nodes_connectivity() -> Result<Vec<NodeConnectivity>> {
    reqwest::get(NODES_CONNECTIVITY_API)
        .await?
        .json::<Vec<MempoolNodeConnectivity>>()
        .await
        .map(|nodes| nodes.into_iter().map(Into::into).collect())
        .map_err(Into::into)
}
