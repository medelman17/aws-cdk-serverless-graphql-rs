use std::net::SocketAddr;

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
use async_graphql::http::{GraphQLPlaygroundConfig, playground_source};
use axum::{AddExtensionLayer, Router, routing::get};
use axum::response::{self, IntoResponse};
use lambda_web::{is_running_on_lambda, LambdaError, run_hyper_on_lambda};

use lib::graphql_handler;
use lib::model::Provider;
use lib::schema::QueryRoot;

async fn graphql_playground() -> impl IntoResponse {
    response::Html(playground_source(GraphQLPlaygroundConfig::new("/")))
}

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    let schema = Schema::build(QueryRoot, EmptyMutation, EmptySubscription)
        .data(Provider::new())
        .finish();

    let app = Router::new()
        .route("/", get(graphql_playground)
            .post(graphql_handler))
        .layer(AddExtensionLayer::new(schema));

    if is_running_on_lambda() {
        run_hyper_on_lambda(app).await?;
    } else {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

        println!("Playground: http://localhost:8000");

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await?;
    }
    Ok(())
}
