use crate::user_service::User;
use async_graphql::ID;

pub struct DB;

impl DB {
    pub fn get_data(&self) -> Vec<User> {
        vec![
            User {
                id: ID::from(1),
                name: String::from("Alice"),
                email: String::from("alice@example.com"),
            },
            User {
                id: ID::from(2),
                name: String::from("Bob"),
                email: String::from("bob@example.com"),
            },
        ]
    }
}
