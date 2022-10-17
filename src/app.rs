mod users;
mod posts;

use async_graphql::{MergedObject, Schema, EmptySubscription};

pub use users::{UserQuery, UserMutation};
pub use posts::{PostQuery};

#[derive(MergedObject, Default)]
pub struct Query(UserQuery, PostQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserMutation);

pub type AppSchema = Schema<Query, Mutation, EmptySubscription>;
