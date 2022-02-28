use async_graphql::Object;

pub struct Query;

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
