use async_graphql::{SimpleObject, Object};

#[derive(Default)]
pub struct UserQuery;

#[derive(SimpleObject)]
struct User {
    id: usize,
    name: String,
}

#[Object]
impl UserQuery {
    async fn users(&self) -> Vec<User> {
        vec![
            User {
                id: 1,
                name: "User 1".to_string(),
            },
            User {
                id: 2,
                name: "User 2".to_string(),
            },
        ]
    }
}
