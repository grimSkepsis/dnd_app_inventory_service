use crate::db::DB;
use crate::graphql::schemas::inventory_item_schema::InventoryItemQueryFilter;
use crate::graphql::schemas::inventory_with_items_schema::InventoryWithItems;
use async_graphql::Object;
use std::sync::Arc;

pub struct InventoryWithItemsQuery {
    pub db: Arc<DB>,
}
impl InventoryWithItemsQuery {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[Object]
impl InventoryWithItemsQuery {
    pub async fn get_inventory_with_items_by_owner_name(
        &self,
        name_term: String,
        page: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> Option<InventoryWithItems> {
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
}
