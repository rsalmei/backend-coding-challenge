mod connectivity;
mod utils;

use std::time::Duration;

/// The interval at which to update the nodes connectivity data.
const NODES_CONNECTIVITY_UPDATE_INTERVAL: Duration = Duration::from_secs(30);

#[tokio::main]
async fn main() {
    // spawn the node connectivity update task.
    tokio::spawn(utils::periodic_task(
        NODES_CONNECTIVITY_UPDATE_INTERVAL,
        connectivity::update_nodes_connectivity_task,
    )); // ahh, much better! a generic periodic task spawner.

    tokio::time::sleep(Duration::from_hours(1)).await;
}
