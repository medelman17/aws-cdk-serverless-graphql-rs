use crate::{
    error::Error,
    models::customer::{Customer, CustomerRange},
    store::{StoreDelete, StoreGet, StoreGetAll, StorePut},
};

pub async fn get_customers(
    store: &dyn StoreGetAll,
    next: Option<&str>,
) -> Result<CustomerRange, Error> {
    store.all(next).await
}

pub async fn get_customer(
    store: &dyn StoreGet,
    id: &str,
) -> Result<Option<Customer>, Error> {
    store.get(id).await
}

pub async fn put_customer(
    store: &dyn StorePut,
    customer: &Customer
) -> Result<(), Error> {
    let mut customer = customer.clone();
    store.put(&customer).await
}

pub async fn delete_customer(
    store: &dyn StoreDelete,
    id: &str
) -> Result<(), Error> {
    store.delete(id).await
}