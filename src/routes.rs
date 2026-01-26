use crate::connectivity::NodeConnectivity;
use axum::{Json, extract::State, http::StatusCode};
use chrono::DateTime;
use serde::Serialize;
use surrealdb::Surreal;
use surrealdb::engine::any::Any;

/// The friendly node connectivity structure that will be returned by the API.
#[derive(Debug, Serialize)]
pub struct FriendlyNodeConnectivity {
    pub public_key: String,
    pub alias: String,
    /// Capacity in BTC, converted from satoshi, divided by 100M.
    pub capacity: f64,
    /// First seen datetime in ISO 8601 format.
    pub first_seen: String,
}

/// Filter parameters for the GET /nodes endpoint.
#[derive(Debug, Deserialize)]
pub struct NodeConnectivityFilter {
    min_capacity: Option<f64>, // in BTC, as returned by the API.
}

/// Handler for the GET /nodes endpoint.
pub async fn get_nodes_connectivity_handler(
    State(db): State<Surreal<Any>>,
    Query(filter): Query<NodeConnectivityFilter>,
) -> Result<Json<Vec<FriendlyNodeConnectivity>>, AppError> {
    if let Some(min_capacity) = filter.min_capacity
        && min_capacity < 0.0
    {
        return Err(AppError::ValueError(
            "min_capacity must be non-negative".to_owned(),
        ));
    }

    // prepare the SQL query based on the filter, while avoiding memory allocations.
    let sql = match filter.min_capacity {
        Some(_) => {
            "SELECT * FROM (SELECT *, first_seen FROM ln_node_connectivity
            WHERE capacity >= $min_capacity)"
        }
        None => "SELECT * FROM ln_node_connectivity",
    };
    // fetch current nodes connectivity data from the database.
    let mut response = db
        .query(sql)
        .bind((
            "min_capacity", // no problem binding vars that are not used in the query.
            filter.min_capacity.map(|c| (c * 100_000_000.0) as u64),
        ))
        .await
        .map_err(AppError::Database)?;

    // extract the result from the first statement in the query.
    let nodes: Vec<NodeConnectivity> = response.take(0).map_err(AppError::Database)?;

    // transform data for the response.
    let response = nodes
        .into_iter()
        .map(|node| {
            let first_seen = DateTime::from_timestamp(node.first_seen, 0)
                .map(|dt| dt.to_rfc3339())
                .unwrap_or_else(|| "Invalid Date".to_string());

            FriendlyNodeConnectivity {
                public_key: node.public_key,
                alias: node.alias,
                capacity: node.capacity as f64 / 100_000_000.0,
                first_seen,
            }
        })
        .collect();

    Ok(Json(response))
}
