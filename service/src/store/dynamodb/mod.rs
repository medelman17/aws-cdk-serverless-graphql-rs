use std::collections::HashMap;

use async_graphql::ID;
use async_trait::async_trait;
use aws_config;
use aws_sdk_dynamodb::{Client, SdkError};
use aws_sdk_dynamodb::error::{DeleteItemError, GetItemError, PutItemError, ScanError};
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::output::{DeleteItemOutput, GetItemOutput, PutItemOutput, ScanOutput};
use tracing::{error, info, instrument, warn};

use ext::AttributeValueExt;

use crate::{Customer, CustomerRange, Error};

use super::{Store, StoreDelete, StoreGet, StoreGetAll, StorePut};

mod ext;

pub struct DynamoDBStore<C> {
    client: Client<C>,
    table_name: String,
}

impl<C> DynamoDBStore<C>
    where
        C: aws_smithy_client::bounds::SmithyConnector,
{
    pub fn new(client: Client<C>, table_name: String) -> DynamoDBStore<C> {
        DynamoDBStore { client, table_name }
    }
}

impl<C> Store for DynamoDBStore<C> where C: aws_smithy_client::bounds::SmithyConnector {}

#[async_trait]
impl<C> StoreGetAll for DynamoDBStore<C>
    where
        C: aws_smithy_client::bounds::SmithyConnector {
    #[instrument(skip(self))]
    async fn all(&self, next: Option<&str>) -> Result<CustomerRange, Error> {
        info!("Scanning DynamoDB table");
        let mut req = self.client.scan().table_name(&self.table_name).limit(20);
        req = if let Some(next) = next {
            req.exclusive_start_key("id", AttributeValue::S(next.to_owned()))
        } else {
            req
        };

        let res = req.send().await?;

        let customers = match res.items {
            Some(items) => items
                .into_iter()
                .map(|v| v.try_into())
                .collect::<Result<Vec<Customer>, Error>>()?,
            None => Vec::default(),
        };
        let next = res
            .last_evaluated_key
            .map(|m| m.get_s("PK").unwrap());
        Ok(CustomerRange { customers, next })
    }
}


#[async_trait]
impl<C> StoreGet for DynamoDBStore<C>
    where
        C: aws_smithy_client::bounds::SmithyConnector, {
    /// Get item
    #[instrument(skip(self))]
    async fn get(&self, id: &str) -> Result<Option<Customer>, Error> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert("PK".to_string(), AttributeValue::S(id.to_string()));
        key.insert("SK".to_string(), AttributeValue::S(id.to_string()));

        let res = self
            .client
            .get_item()
            .table_name(&self.table_name)
            .set_key(Some(key))
            .send()
            .await?;

        Ok(match res.item {
            Some(item) => Some(item.try_into()?),
            None => None
        })
    }
}

#[async_trait]
impl<C> StorePut for DynamoDBStore<C>
    where
        C: aws_smithy_client::bounds::SmithyConnector, {
    /// Get item
    #[instrument(skip(self))]
    async fn put(&self, customer: &Customer) -> Result<(), Error> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert("PK".to_string(), AttributeValue::S(customer.id.to_string()));
        key.insert("SK".to_string(), AttributeValue::S(customer.id.to_string()));

        self.client
            .put_item()
            .table_name(&self.table_name)
            .set_item(Some(customer.into()))
            .send()
            .await?;

        Ok(())
    }
}

#[async_trait]
impl<C> StoreDelete for DynamoDBStore<C>
    where
        C: aws_smithy_client::bounds::SmithyConnector, {
    /// Get item
    #[instrument(skip(self))]
    async fn delete(&self, id: &str) -> Result<(), Error> {
        let mut key: HashMap<String, AttributeValue> = HashMap::new();
        key.insert("PK".to_string(), AttributeValue::S(id.to_string()));
        key.insert("SK".to_string(), AttributeValue::S(id.to_string()));

        self.client
            .delete_item()
            .table_name(&self.table_name)
            .set_key(Some(key))
            .send()
            .await?;

        Ok(())
    }
}

impl From<&Customer> for HashMap<String, AttributeValue> {
    fn from(value: &Customer) -> HashMap<String, AttributeValue> {
        let value = value.clone();
        let mut retval = HashMap::new();
        retval.insert("PK".to_string(), AttributeValue::S((value.id).clone().to_string()));
        retval.insert("SK".to_string(), AttributeValue::S((value.id).clone().to_string()));
        retval.insert("GivenName".to_string(), AttributeValue::S(value.given_name.clone().parse().unwrap()));

        retval
    }
}

impl TryFrom<HashMap<String, AttributeValue>> for Customer {
    type Error = Error;
    fn try_from(value: HashMap<String, AttributeValue>) -> Result<Self, Self::Error> {
        Ok(Customer {
            id: ID::from(value.get_s("PK").ok_or(Error::InternalError("Missing PK"))?),
            given_name: value.get_s("GivenName").ok_or(Error::InternalError("Missing Name"))?,
        })
    }
}

pub async fn load_config() -> aws_config::Config {
    let config = aws_config::load_from_env().await;
    config
}

pub fn get_table_name() -> String {
    match std::env::var("TABLE_NAME") {
        Ok(res) => res,
        Err(e) => {
            warn!("'TABLE_NAME' environment variable not found; using default");
            "ServiceStack-ServiceDatabase8F93061E-1ETX76DJGQEK4".to_string()
        }
    }
}

pub fn init_client(cfg: &aws_config::Config) -> Client {
    Client::new(cfg)
}

pub async fn list_tables(client: &Client) {
    let resp = client.list_tables().send().await.unwrap();
    println!("Tables: ");

    let names = resp.table_names().unwrap_or_default();

    for name in names {
        println!("    {}", name);
    }
}

// pub async fn create_item(
//     client: &Client,
//     table_name: &str,
//     item: HashMap<String, AttributeValue>,
// ) -> Result<PutItemOutput, SdkError<PutItemError>> {
//     let res = client
//         .put_item()
//         .table_name(table_name)
//         .set_item(Some(item))
//         .send()
//         .await;
//
//     match res {
//         Ok(output) => {
//             info!("Item created!");
//             Ok(output)
//         }
//         Err(e) => {
//             error!("Failed to create item! {}",e);
//             Err(e)
//         }
//     }
// }
//
// pub async fn list_items(
//     client: &Client,
//     table_name: &str,
// ) -> Result<ScanOutput, SdkError<ScanError>> {
//     let req = client
//         .scan()
//         .table_name(table_name);
//     let resp = req.send().await?;
//     info!("Current items: {:?}", resp.items);
//     Ok(resp)
// }
//
// pub async fn delete_item(
//     client: &Client,
//     table_name: &str,
//     item: HashMap<String, AttributeValue>,
// ) -> Result<DeleteItemOutput, SdkError<DeleteItemError>> {
//     let req = client
//         .delete_item()
//         .table_name(table_name)
//         .set_key(Some(item));
//     let resp = req.send().await?;
//     Ok(resp)
// }
//
// pub async fn get_item(
//     client: &Client,
//     table_name: &str,
//     item: HashMap<String, AttributeValue>,
// ) -> Result<GetItemOutput, SdkError<GetItemError>> {
//     let req = client
//         .get_item()
//         .table_name(table_name)
//         .set_key(Some(item));
//     let resp = req.send().await?;
//     Ok(resp)
// }