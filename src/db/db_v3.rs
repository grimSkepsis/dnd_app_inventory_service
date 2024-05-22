use crate::inventory_service::Inventory;
use crate::user_service::User;
use async_graphql::ID;

pub struct DB;

impl DB {
    pub fn get_inventory_by_uuid(&self, uuid: String) -> Inventory {
        Inventory {
            uuid: ID::from(uuid),
            name: "Test Inventory".to_string(),
            capacity: 100,
            cp: 100,
            sp: 100,
            gp: 100,
            pp: 100,
        }
    }

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
