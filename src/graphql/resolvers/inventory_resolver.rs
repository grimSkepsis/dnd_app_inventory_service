use crate::{db::DB, graphql::schemas::inventory_schema::Inventory};
use async_graphql::Object;
use std::sync::Arc;

pub struct InventoryQuery {
    db: Arc<DB>,
}

impl InventoryQuery {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[Object]
impl InventoryQuery {
    pub async fn get_inventory(&self, id: String) -> Option<Inventory> {
        self.db.get_inventory_by_uuid(id).await
    }

    pub async fn get_chracter_inventory(&self, id: String) -> Option<Inventory> {
        self.db.get_inventory_by_character_uuid(id).await
    }

    pub async fn get_inventory_by_owner(&self, id: String) -> Option<Inventory> {
        self.db.get_inventory_by_owner_uuid(id).await
    }

    pub async fn get_inventory_by_owner_name(&self, name_term: String) -> Option<Inventory> {
        self.db.get_inventory_by_owner_name(name_term).await
    }
}
