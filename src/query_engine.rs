use crate::db::DB;
use crate::inventory_service::Inventory;
use crate::user_service::User;
use async_graphql::Object;

pub struct Query {
    pub db: DB,
}

#[Object]
impl Query {
    pub async fn get_inventory(&self, id: String) -> Option<Inventory> {
        self.db.get_inventory_by_uuid(id).await
    }

    pub async fn get_chracter_inventory(&self, id: String) -> Option<Inventory> {
        self.db.get_inventory_by_character_uuid(id).await
    }

    pub async fn get_inventory_by_owner(&self, id: String) -> Option<Inventory> {
        self.db.get_inventory_by_owner_uuid(id).await
    }
}
