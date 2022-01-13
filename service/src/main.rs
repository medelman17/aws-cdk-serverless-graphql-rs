use std::net::SocketAddr;

use axum::{AddExtensionLayer, Router, routing::get};
use lambda_web::{is_running_on_lambda, LambdaError, run_hyper_on_lambda};
use tracing::info;

use lib::{entrypoints, graphql, utils};

#[tokio::main]
async fn main() -> Result<(), LambdaError> {
    // dotenv::from_filename(".env.local").ok();
    utils::init_tracing();

    // let output = process::Command::new("git").args(&["rev-parse", "HEAD"]).output().unwrap();
    // let git_hash = String::from_utf8(output.stdout).unwrap();
    //
    // std::env::set_var("APOLLO_USER_VERSION", git_hash);


    // if is_running_on_lambda() {
    //     std::env::set_var("APOLLO_PLATFORM", "aws-lambda");
    // } else {
    //     std::env::set_var("APOLLO_PLATFORM", "localhost");
    // }

    let schema = graphql::schema::get_schema().await;
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
