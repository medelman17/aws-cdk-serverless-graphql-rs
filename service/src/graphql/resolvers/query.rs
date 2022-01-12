use async_graphql::{Context, ID, Object};
use axum::response::IntoResponse;
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::{Customer, CustomerRange, domain, store, utils};
use crate::graphql::schema::ServiceContext;
use crate::store::Store;

type E = Box<dyn std::error::Error + Sync + Send + 'static>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn get_customer_by_id<'a>(&self, _ctx: &'a Context<'_>) -> Result<Option<Customer>, E> {
        let store = utils::get_store().await;
        let customer = domain::get_customer(&store, "12345").await.unwrap();
        Ok(customer)
    }
    // #[instrument(skip(ctx))]
    // async fn list_customers<'a>(&self, ctx: &'a Context<'_>) -> Result<CustomerRange, E> {
    //     Ok(())
    // }
}
