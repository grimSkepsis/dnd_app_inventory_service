// use async_graphql::parser::types::DirectiveLocation::Schema;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::post;
use axum::Router;
use dotenv::dotenv;
use neo4rs::Graph;
use std::env;
use std::sync::Arc;
use tracing::instrument;
use tracing_appender::rolling::daily;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

use crate::graphql::resolvers::root_resolver::QueryRoot;
use crate::models::{
    inventory_item_model::InventoryItemModelManager, inventory_model::InventoryModelManager,
    inventory_with_items_model::InventoryWithItemsModelManager, item_model::ItemModelManager,
};

use tower_http::cors::{Any, CorsLayer};
mod graphql;
mod models;

#[instrument(skip(schema, graph_glrequest))]
async fn graphql_handler(
    schema: Schema<QueryRoot, EmptyMutation, EmptySubscription>,
    graph_glrequest: GraphQLRequest,
) -> GraphQLResponse {
    let inner_request = graph_glrequest.into_inner();
    let res = schema.execute(inner_request).await;

    res.into()
}

#[tokio::main]
async fn main() {
    // Load .env file
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow any origin
        .allow_methods(Any) // Allow any method
        .allow_headers(Any);
    dotenv().ok();
    println!("RUST_LOG: {:?}", env::var("RUST_LOG"));
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

    let create_db_connection_pool = || async {
        println!("Creating connection pool for Neo4j");
        let uri = env::var("NEO4J_URI").unwrap_or_else(|_| "neo4j://localhost:7687".to_string());
        let user = env::var("NEO4J_USER").unwrap_or_else(|_| "neo4j".to_string());
        let pass = env::var("NEO4J_PASSWORD").unwrap_or_else(|_| "neo4j".to_string());
        let graph = Graph::new(uri, user, pass).await.unwrap();
        graph
    };

    let graph = Arc::new(create_db_connection_pool().await);

    let schema = Schema::build(
        QueryRoot::new(
            InventoryModelManager::new(graph.clone()),
            InventoryItemModelManager::new(graph.clone()),
            InventoryWithItemsModelManager::new(
                InventoryItemModelManager::new(graph.clone()),
                InventoryModelManager::new(graph.clone()),
            ),
            ItemModelManager::new(graph),
        ),
        EmptyMutation,
        EmptySubscription,
    )
    .finish();

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .try_init()
        .expect("Failed to initialize logging");
    let app = Router::new()
        .route(
            "/graphql",
            post(|req: GraphQLRequest| graphql_handler(schema, req)),
        )
        .layer(cors);

    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let listener = tokio::net::TcpListener::bind(&host).await.unwrap();

    println!("GQL on: {host}");

    axum::serve(listener, app).await.unwrap()
}
