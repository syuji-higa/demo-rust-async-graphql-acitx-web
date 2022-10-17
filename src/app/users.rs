use async_graphql::{SimpleObject, Object};

#[derive(Default)]
pub struct UserQuery;

#[derive(Default)]
pub struct UserMutation;

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

#[Object]
impl UserMutation {
    async fn add_user(&self, name: String) -> Result<bool, &'static str> {
        match get_value_good(true) {
            Ok(result) => {
                println!("success: {}", name);
                Ok(result)
            }
            Err(msg) => {
                println!("failure: {}", msg);
                Err(msg)
            }
        }
    }
}

fn get_value_good(v: bool) -> Result<bool, &'static str> {
    if v {
        Ok(true)
    } else {
        Err("error message")
    }
}
