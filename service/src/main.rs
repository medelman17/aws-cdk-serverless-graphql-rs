use std::net::SocketAddr;

use axum::{routing::get, AddExtensionLayer, Router};
use lambda_web::{is_running_on_lambda, run_hyper_on_lambda, LambdaError};
use tracing::info;

use lib::{entrypoints, graphql, utils};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    utils::init_tracing();

    let schema = graphql::schema::init().await;
    let app = Router::new()
        .route(
            "/",
            get(entrypoints::graphql::playground).post(entrypoints::graphql::handler),
        )
        .layer(AddExtensionLayer::new(schema));

    if is_running_on_lambda() {
        run_hyper_on_lambda(app).await?;
    } else {
        let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
        info!("Playground: http://localhost:8000");
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
    Ok(())
}
