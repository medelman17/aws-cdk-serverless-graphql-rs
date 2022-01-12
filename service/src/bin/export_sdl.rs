// use std::env;
// use std::fs;
//
// use async_graphql::*;
// use dotenv::dotenv;
//
// use lib::{graphql::resolvers, utils};
// use lib::graphql::schema::{ApolloTracingOpts, SchemaOpts};
//
// #[tokio::main]
// async fn main() {
//     dotenv::from_filename(".env.local").ok();
//
//     for v in dotenv::vars() {
//         println!(" {:?}", v);
//     }
//
//     utils::init_tracing();
//     let schema = utils::get_schema().await;
//     fs::write("schema.graphql", &schema.sdl());
//
//     println!("{}", &schema.sdl());
// }
