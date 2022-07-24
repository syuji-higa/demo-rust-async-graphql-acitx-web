use async_graphql::{Object, SimpleObject};

pub struct Query;

#[derive(SimpleObject)]
struct Posts {
    id: usize,
}

#[Object]
impl Query {
    async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    async fn posts(&self) -> Posts {
        Posts {
            id: 1,
        }
    }
}
