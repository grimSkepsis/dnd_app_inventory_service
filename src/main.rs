// use async_graphql::parser::types::DirectiveLocation::Schema;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use async_graphql::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::routing::post;
use axum::Router;
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
mod query_engine;
mod user_service;

#[instrument(skip(graph_glrequest))]
async fn graphql_handler(graph_glrequest: GraphQLRequest) -> GraphQLResponse {
    // Log the incoming request
    info!("Received GraphQL request");
    let query = Query { db: DB };
    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(graph_glrequest.into_inner()).await;
    // Log the response
    info!("GraphQL response: {:?}", res);
    res.into()
}

#[tokio::main]
async fn main() {
    // Create a rolling file appender
    // let file_appender = tracing_appender::rolling::daily("logs", "app.log");

    // // Initialize logging with file appender
    // tracing_subscriber::fmt()
    //     .with_env_filter(EnvFilter::from_default_env().or_else("info")) // Default to `info` level
    //     .with_writer(file_appender.and(std::io::stdout)) // Log to file and stdout
    //     .with_format(tracing_subscriber::fmt::format().compact()) // Compact log format
    //     .with_target(false) // Disable module paths in logs
    //     .with_thread_ids(false) // Disable thread IDs in logs
    //     .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE) // Only log span close events
    //     .init();

    let file_appender = daily("logs", "app.log");

    // Initialize logging
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = tracing_subscriber::fmt::layer()
        .with_writer(file_appender.and(std::io::stdout))
        .with_timer(tracing_subscriber::fmt::time::SystemTime) // Use SystemTime for simplicity
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(false);

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatting_layer)
        .try_init()
        .expect("Failed to initialize logging");

    let app = Router::new().route("/graphql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Playground: http://localhost:3000/graphql");
    axum::serve(listener, app).await.unwrap()
}
