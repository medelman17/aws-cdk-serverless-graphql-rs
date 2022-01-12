use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApolloTracingConfig {
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

impl ApolloTracingConfig {
    pub fn new() -> Self {
        Self {
            authorization_token: dotenv::var("APOLLO_KEY").unwrap_or("".to_string()),
            hostname: dotenv::var("SERVICE_HOSTNAME").unwrap_or("".to_string()),
            graph_ref: dotenv::var("APOLLO_GRAPH_REF").unwrap_or("".to_string()),
            release_name: dotenv::var("APOLLO_SUBGRAPH_NAME").unwrap_or("".to_string()),
            batch_target: 10,
            server_id: dotenv::var("SERVICE_ID").unwrap_or("".to_string()),
            variant: dotenv::var("APOLLO_SUPER_GRAPH_VARIANT").unwrap_or("".to_string()),
            user_version: dotenv::var("APOLLO_USER_VERSION").unwrap_or("".to_string()),
            platform: dotenv::var("APOLLO_PLATFORM").unwrap_or("".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchemaInitConfig {
    pub apollo_studio: ApolloTracingConfig,
}


impl SchemaInitConfig {
    pub fn new() -> Self {
        Self {
            apollo_studio: ApolloTracingConfig::new()
        }
    }
}