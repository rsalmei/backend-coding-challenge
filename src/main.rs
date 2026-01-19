mod connectivity;
mod utils;

use anyhow::Result;
use std::time::Duration;

/// The interval at which to update the nodes connectivity data.
const NODES_CONNECTIVITY_UPDATE_PERIOD: Duration = Duration::from_secs(30);

#[tokio::main]
async fn main() -> Result<()> {
    // use the endpoint specified in an environment variable, or default to `memory`.
    let endpoint = std::env::var("SURREALDB").unwrap_or_else(|_| "memory".to_owned());
    let db = surrealdb::engine::any::connect(endpoint).await?;
    db.use_ns("namespace").use_db("database").await?;

    // spawn the node connectivity update task.
    tokio::spawn(utils::periodic_task(NODES_CONNECTIVITY_UPDATE_PERIOD, {
        let db = db.clone(); // the surreal db handle is cheap to clone, just an Arc internally.
        move || connectivity::update_nodes_connectivity_task(db.clone())
    })); // ahh, much better! a generic periodic task spawner.

    // just for demo purposes (and keeping the main function alive), print the current total number
    // of nodes in the database, that is being updated in the background.
    loop {
        let mut nodes = db
            .query("select count() from ln_node_connectivity group all")
            .await?;
        let nodes: Option<u64> = nodes.take("count")?;
        println!("total nodes in db: {}", nodes.unwrap_or_default());
        tokio::time::sleep(NODES_CONNECTIVITY_UPDATE_PERIOD).await; // Duration is Copy.
    }
}
