use async_graphql::{ID, Object};

#[derive(Clone)]
pub struct Customer {
    id: ID,
    name: String,
}

#[Object]
impl Customer {
    async fn id(&self) -> &str { &self.id }
    async fn name(&self) -> &str { &self.name }
}

pub struct Provider {}

impl Provider {
    pub fn new() -> Self {
        Self {}
    }

    pub fn list_customers(&self) -> Vec<&Customer> {
        let customers: Vec<&Customer> = vec![];
        customers
    }
}