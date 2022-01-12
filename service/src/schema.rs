use async_graphql::{Context, EmptyMutation, EmptySubscription, ID, Object, Schema};

use crate::model::{Customer, Provider};

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn customers<'a>(&self, ctx: &'a Context<'_>) -> Vec<&'a Customer> {
        ctx.data_unchecked::<Provider>().list_customers()
    }
}

pub type ServiceSchema = Schema<QueryRoot, EmptyMutation, EmptySubscription>;