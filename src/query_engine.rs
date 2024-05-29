use crate::db::DB;
use crate::inventory_item_service::InventoryItem;
use crate::inventory_service::{Inventory, InventoryItemQueryFilter};
use crate::inventory_with_items_service::InventoryWithItems;
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

    pub async fn get_inventory_by_owner_name(&self, name_term: String) -> Option<Inventory> {
        self.db.get_inventory_by_owner_name(name_term).await
    }

    pub async fn get_inventory_with_items_by_owner_name(
        &self,
        name_term: String,
        page: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> (Option<InventoryWithItems>) {
        self.db
            .get_inventory_with_items_by_owner_name(
                name_term,
                page,
                page_size,
                order_by,
                order_direction,
                filter,
            )
            .await
    }

    pub async fn get_inventory_items(
        &self,
        inventory_id: String,
        page: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> Option<PaginatedResponse<InventoryItem>> {
        self.db
            .get_inventory_items(
                inventory_id,
                page,
                page_size,
                order_by,
                order_direction,
                filter,
            )
            .await
    }
}
