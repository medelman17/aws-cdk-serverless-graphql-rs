#[derive(Debug, Clone)]
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
            authorization_token: env!("APOLLO_KEY").to_string(),
            hostname: env!("SERVICE_HOSTNAME").to_string(),
            graph_ref: "gah".to_string(),
            release_name: env!("APOLLO_SUBGRAPH_NAME").to_string(),
            batch_target: 10,
            server_id: env!("SERVICE_ID").to_string(),
            variant: env!("APOLLO_SUPER_GRAPH_VARIANT").to_string(),
            user_version: env!("APOLLO_USER_VERSION").to_string(),
            platform: env!("APOLLO_PLATFORM").to_string(),
        }
    }
}

#[derive(Debug, Clone)]
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