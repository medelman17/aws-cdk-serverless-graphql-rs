use async_graphql::{EmptySubscription, extensions, Schema};
use async_graphql_extension_apollo_tracing::ApolloTracing;

use crate::config::SchemaInitConfig;
use crate::graphql::resolvers::{
    mutation::MutationRoot,
    query::QueryRoot,
};
use crate::store;
use crate::utils;

// use async_graphql_extension_apollo_tracing::{ApolloTracing, ApolloTracingDataExt, HTTPMethod, register::register};

pub type GraphQLServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;


#[derive(Clone)]
pub struct SchemaOpts {
    pub apollo_tracing: ApolloTracingOpts,
}

#[derive(Clone)]
pub struct ApolloTracingOpts {
    pub authorization_token: String,
    pub hostname: String,
    pub graph_ref: String,
    pub release_name: String,
    pub batch_target: usize,
    pub server_id: String,
    pub variant: String,
    pub user_version: String,
    pub platform: String,
}


pub async fn get_schema() -> GraphQLServiceSchema {
    let config = SchemaInitConfig::new();
    let tracing_config = config.apollo_studio.clone();
    let apollo_tracing = ApolloTracing::new(
        tracing_config.authorization_token.clone(),
        tracing_config.hostname.clone(),
        tracing_config.graph_ref.clone(),
        tracing_config.release_name.clone(),
        tracing_config.batch_target.clone(),
    );

    let schema = Schema::build(
        QueryRoot,
        MutationRoot,
        EmptySubscription,
    ).extension(apollo_tracing)
        .finish();

    // register(
    //     &tracing_config.authorization_token,
    //     &schema,
    //     &tracing_config.server_id,
    //     &tracing_config.variant,
    //     &tracing_config.user_version,
    //     &tracing_config.platform,
    // )
    //     .await
    //     .unwrap();

    return schema;
}

// pub async fn init() -> GraphQLServiceSchema {
//     let store = utils::get_store().await;
//     Schema::build(
//         QueryRoot,
//         MutationRoot,
//         EmptySubscription)
//         .data(store)
//         .finish()
// }