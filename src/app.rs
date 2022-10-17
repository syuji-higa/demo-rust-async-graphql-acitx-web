mod users;
mod posts;

use async_graphql::{MergedObject, Schema, EmptyMutation, EmptySubscription};

pub use users::UserQuery;
pub use posts::PostQuery;

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, PostQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;
