use crate::db::DB;
use crate::inventory_service::Inventory;
use crate::user_service::User;
use async_graphql::Object;

pub struct Query {
    pub db: DB,
}

#[Object]
impl Query {
    // pub fn new(db: Db) -> Self {
    //     Query { db }
    // }

    pub async fn get_inventory(&self, id: String) -> Option<Inventory> {
        Some(self.db.get_inventory_by_uuid(id))
    }
    pub async fn get_users(&self) -> Vec<User> {
        self.db.get_data()
    }

    pub async fn get_user(&self, id: String) -> Option<User> {
        self.db
            .get_data()
            .iter()
            .find(|user| user.id.0 == id)
            .cloned()
    }
}
