use std::collections::HashMap;

use async_graphql::{EmptySubscription, extensions, Schema};
use aws_sdk_dynamodb::model::AttributeValue;
use serde_json::Value;
use tracing::{info, instrument};
use tracing_subscriber;
use ulid::Ulid;

use crate::{
    graphql::resolvers::{
        mutation::MutationRoot, query::QueryRoot,
    }, graphql::schema::GraphQLServiceSchema,
    store};
pub use crate::graphql::schema::get_schema;

pub fn init_tracing() {
    tracing_subscriber::fmt::init();
    // let subscriber = tracing_subscriber::fmt()
    //     .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
    //     .json()
    //     .finish();
    // tracing::subscriber::set_global_default(subscriber).expect("failed to set tracing subscriber");
}

pub fn create_unique_id() -> String {
    let ulid = Ulid::new();
    ulid.to_string()
}

pub fn parse_item(value: Value) -> HashMap<String, AttributeValue> {
    match value_to_item(value) {
        AttributeValue::M(map) => map,
        other => panic!("can only insert top level values, got {:?}", other),
    }
}

pub fn value_to_item(value: Value) -> AttributeValue {
    match value {
        Value::Null => AttributeValue::Null(true),
        Value::Bool(b) => AttributeValue::Bool(b),
        Value::Number(n) => AttributeValue::N(n.to_string()),
        Value::String(s) => AttributeValue::S(s),
        Value::Array(a) => AttributeValue::L(a.into_iter().map(value_to_item).collect()),
        Value::Object(o) => {
            AttributeValue::M(o.into_iter().map(|(k, v)| (k, value_to_item(v))).collect())
        }
    }
}

pub async fn get_store() -> impl store::Store {
    let config = store::dynamodb::load_config().await;
    let table_name = store::dynamodb::get_table_name();
    let client = store::dynamodb::init_client(&config);
    store::DynamoDBStore::new(client, table_name)
}


