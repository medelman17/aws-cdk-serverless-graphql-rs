// use std::collections::HashMap;
// use std::fmt::Error;
// use std::net::SocketAddr;
//
// use async_graphql::Schema;
// use aws_sdk_dynamodb::model::AttributeValue;
// use aws_sdk_dynamodb::output::{DeleteItemOutput, ScanOutput};
// use axum::{AddExtensionLayer, Router};
// use axum::routing::get;
// use lambda_web::{is_running_on_lambda, run_hyper_on_lambda};
// use tracing::{error, info};
//
// use lib::domain;
// use lib::entrypoints::graphql::*;
// use lib::graphql;
// use lib::store::{dynamodb, Store};
// use lib::utils::*;
//
// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     init_tracing();
//     let store = get_store().await;
//     let schema = graphql::schema::init().await;
//     let app = Router::new()
//         .route("/", get(playground).post(handler))
//         .layer(AddExtensionLayer::new(schema));
//
//     if is_running_on_lambda() {
//         run_hyper_on_lambda(app).await.unwrap();
//     } else {
//         let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
//         println!("Playground: http://localhost:8000");
//         axum::Server::bind(&addr)
//             .serve(app.into_make_service())
//             .await
//             .unwrap();
//     }
//
//
//     Ok(())
// }