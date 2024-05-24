use crate::db::DB;
use crate::inventory_item_service::InventoryItem;
use crate::inventory_service::Inventory;
use crate::pagination_service::PaginatedResponse;
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

    pub async fn get_inventory_items(
        &self,
        inventory_id: String,
        page: u32,
        page_size: u32,
    ) -> Option<PaginatedResponse<InventoryItem>> {
        self.db
            .get_inventory_items(inventory_id, page, page_size)
            .await
    }
}
