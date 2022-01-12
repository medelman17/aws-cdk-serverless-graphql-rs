use async_graphql::{ID, Object, };
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Customer {
    pub id: ID,
    // created_at: String,
    // updated_at: String,
    pub(crate) given_name: String,
    // family_name: String,
    // preferred_name: Option<String>,
    // email: String,
    // telephone: String,
    // tax_id: Option<String>,
}

// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct PaginatedCustomerList {
//
// }
//
// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct Edge {
//     cursor: String
// }
//
// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct PageInfo {
//     end_cursor: Option<String>,
//     has_next_page: bool,
//     has_previous_page: bool,
//     start_cursor: Option<String>
// }
//
// #[derive(Debug, Default, Deserialize, Serialize)]
// pub struct ConnectionData<T> {
//     page_info: PageInfo,
//     edges
//
// }
//
//
//
// #[Object]
// impl PaginatedCustomerList {
//     page_info: PageInfo
// }

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct CustomerRange {
    pub customers: Vec<Customer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next: Option<String>,
}

#[Object]
impl Customer {
    pub(crate) async fn id(&self) -> &str {
        &self.id
    }

    async fn given_name(&self) -> &str {
        &self.given_name
    }
}

#[Object]
impl CustomerRange {
    async fn customers(&self) -> &Vec<Customer> { &self.customers }
    async fn next(&self) -> &Option<String> { &self.next }
}

