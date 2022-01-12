use async_trait::async_trait;
use aws_sdk_dynamodb::output::ScanOutput;

pub use dynamodb::DynamoDBStore;

use crate::error::Error;
use crate::models::customer::{Customer, CustomerRange};

pub mod dynamodb;
mod memory;

pub trait Store: StoreGetAll + StoreGet + StorePut + StoreDelete {}

#[async_trait]
pub trait StoreGetAll: Send + Sync {
    async fn all(&self, next: Option<&str>) -> Result<CustomerRange, Error>;
}

#[async_trait]
pub trait StoreGet: Send + Sync {
    async fn get(&self, id: &str) -> Result<Option<Customer>, Error>;
}

#[async_trait]
pub trait StorePut: Send + Sync {
    async fn put(&self, customer: &Customer) -> Result<(), Error>;
}

#[async_trait]
pub trait StoreDelete: Send + Sync {
    async fn delete(&self, id: &str) -> Result<(), Error>;
}