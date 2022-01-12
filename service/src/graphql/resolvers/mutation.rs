use async_graphql::{Context, Object};
use axum::response::IntoResponse;
use serde_json::json;
use tracing::{error, info, instrument, warn};

use crate::{Customer, CustomerRange, domain, store};

type E = Box<dyn std::error::Error + Sync + Send + 'static>;


pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_customer<'a>(&self, ctx: &Context<'_>) -> String {
        "Hi".to_string()
    }
    //
    // #[instrument(skip(ctx))]
    // async fn delete_customer<'a>(&self, ctx: &Context<'_>) -> String {
    //     "Hi".to_string()
    // }
    //
    // #[instrument(skip(ctx))]
    // async fn update_customer<'a>(&self, ctx: &Context<'_>) -> String {
    //     "Hi".to_string()
    // }
}