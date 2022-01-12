use async_graphql::{EmptySubscription, Schema};

use crate::graphql::resolvers::{
    mutation::MutationRoot,
    query::QueryRoot,
};
use crate::store;
use crate::utils;

pub type GraphQLServiceSchema = Schema<QueryRoot, MutationRoot, EmptySubscription>;

pub struct ServiceContext {
    pub store: dyn store::Store,
}

pub async fn init() -> GraphQLServiceSchema {
    let store = utils::get_store().await;
    Schema::build(
        QueryRoot,
        MutationRoot,
        EmptySubscription)
        .data(store)
        .finish()
}