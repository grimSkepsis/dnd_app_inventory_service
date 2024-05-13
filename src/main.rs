// use async_graphql::parser::types::DirectiveLocation::Schema;
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_graphql::Schema;
use async_graphql::EmptyMutation;
use async_graphql::EmptySubscription;
use axum::Router;
use axum::routing::post;

use crate::db::DB;
use crate::query_engine::Query;

mod db;

mod user_service;
mod query_engine;

async fn graphql_handler(graph_glrequest: GraphQLRequest) -> GraphQLResponse {
    let query = Query { db: DB };
    let schema = Schema::new(query, EmptyMutation, EmptySubscription);

    let res = schema.execute(graph_glrequest.into_inner()).await;
    res.into()
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/graphql", post(graphql_handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Playground: http://localhost:3000/graphql");
    axum::serve(listener, app).await.unwrap()
}
