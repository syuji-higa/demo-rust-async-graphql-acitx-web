use async_graphql::{SimpleObject, Object};

#[derive(Default)]
pub struct PostQuery;

#[derive(SimpleObject)]
struct Post {
    id: usize,
    title: String,
}

#[Object]
impl PostQuery {
    async fn posts(&self) -> Vec<Post> {
        vec![
            Post {
                id: 1,
                title: "Post 1".to_string(),
            },
            Post {
                id: 2,
                title: "Post 2".to_string(),
            },
        ]
    }
}
