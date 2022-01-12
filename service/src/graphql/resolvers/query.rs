use async_graphql::*;
use async_graphql::{Context, Object};
// use async_graphql::connection::*;
// use axum::response::IntoResponse;
use tracing::{error, info, instrument, warn};

use crate::{Customer, domain, utils};

// use crate::graphql::schema::ServiceContext;


type E = Box<dyn std::error::Error + Sync + Send + 'static>;

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn customer_by_id<'a>(&self, id: String) -> Result<Option<Customer>, E> {
        let store = utils::get_store().await;
        let customer = domain::get_customer(&store, &id).await.unwrap();
        Ok(customer)
    }
}

//     async fn customers<'a>(
//         &self,
//         after: Option<String>,
//         before: Option<String>,
//         first: Option<i32>,
//         last: Option<i32>,
//     ) -> Result<Connection<usize, i32, EmptyFields, EmptyFields>> {
//         query(after, before, first, last, |after, before, first, last| async move {
//             let mut start = after.map(|after| after + 1).unwrap_or(0);
//             let mut end = before.unwrap_or(10000);
//             if let Some(first) = first {
//                 end = (start + first).min(end);
//             }
//             if let Some(last) = last {
//                 start = if last > end - start {
//                     end
//                 } else {
//                     end - last
//                 };
//             }
//             let mut connection = Connection::new(start > 0, end < 10000);
//             connection.append(
//                 (start..end).into_iter().map(|n|
//                                                  Ok(Edge::with_additional_fields(n, n as i32, EmptyFields)),
//                 ))?;
//             Ok(connection)
//         },
//         )
//     }
// }
