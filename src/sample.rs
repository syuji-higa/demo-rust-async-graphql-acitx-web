mod models;

use async_graphql::{Schema, EmptyMutation, EmptySubscription};

pub use models::Query;

pub type SampleSchema = Schema<Query, EmptyMutation, EmptySubscription>;
