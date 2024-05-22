// use async_graphql::parser::types::DirectiveLocation::Schema;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::post;
use axum::Router;
use dotenv::dotenv;
use neo4rs::{Graph, Node};
use serde_json::to_string;
use std::env;
use tracing::{info, instrument};
use tracing_appender::rolling::daily;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::db::DB;
use crate::query_engine::Query;

mod db;
// pub mod db_v1;
// pub use db_v1 as db;
mod inventory_service;
mod query_engine;
mod user_service;

#[instrument(skip(graph_glrequest))]
async fn graphql_handler(graph_glrequest: GraphQLRequest) -> GraphQLResponse {
    let inner_request = graph_glrequest.into_inner();
    // Serialize the incoming request to a string
    let request_string =
        to_string(&inner_request).unwrap_or_else(|e| format!("Failed to serialize request: {}", e));

    // Log the incoming request
    info!("Received GraphQL request: {}", request_string);
    let query = Query { db: DB };
    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(inner_request).await;
    // Log the response
    info!("GraphQL response: {:?}", res);
    res.into()
}

#[tokio::main]
async fn main() {
    // Load .env file
    dotenv().ok();
    println!("RUST_LOG: {:?}", env::var("RUST_LOG"));
    // log_dotenv_vars();
    let file_appender = daily("logs", "app.log");

    // Initialize logging
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let is_debug = env::var("RUST_LOG")
        .map(|val| val == "debug")
        .unwrap_or(false);
    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender.and(std::io::stdout))
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339()) // Use SystemTime for simplicity
        .with_span_events(FmtSpan::CLOSE)
        .with_target(is_debug)
        .with_thread_ids(is_debug);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .try_init()
        .expect("Failed to initialize logging");
    let app = Router::new().route("/graphql", post(graphql_handler));

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();

    let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "neo4j://localhost:7687".to_string());
    let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
    let pass = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "neo4j".to_string());
    let graph = Graph::new(uri, user, pass).await.unwrap();

    let query =
        "MATCH (p:Character {name: $name})-[:OWNS]->(inv:Inventory)-[:CONTAINS]->(item:Item)
        OPTIONAL MATCH (item)-[:HAS_BASE]->(baseItem:ItemBase)
        RETURN
        COALESCE(item.description, baseItem.description) as description,
        COALESCE(baseItem.name + ' (' + item.name + ')', item.name) AS name,
        item.effect as effect";

    // Set your parameters
    let parameters = neo4rs::query(query).param("name", "Thorrun");

    // Run the query
    let mut result = graph.execute(parameters).await.unwrap();

    // Iterate and process the results
    while let Ok(Some(row)) = result.next().await {
        // Log the raw row for debugging
        // println!("{:?}", row);

        // Safely extract the fields
        let name: String = row.get("name").unwrap();
        // let description: Result<String, _> = row.get("description");
        let description: String = row.get("description").unwrap_or_else(|_| "N/A".to_string());
        let effect: String = row.get("effect").unwrap_or_else(|_| "N/A".to_string());
        println!(
            "Item: {}, Effect: {}, Description: {}",
            name, effect, description
        );
    }

    println!("GQL on: {host}");

    axum::serve(listener, app).await.unwrap()
}
