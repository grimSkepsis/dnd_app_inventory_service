use crate::{
    db::DB,
    graphql::schemas::{
        inventory_item_schema::{InventoryItem, InventoryItemQueryFilter},
        paginated_response_schema::PaginatedResponse,
    },
};
use async_graphql::Object;
use std::sync::Arc;

pub struct InventoryItemQuery {
    db: Arc<DB>,
}
impl InventoryItemQuery {
    pub fn new(db: Arc<DB>) -> Self {
        Self { db }
    }
}

#[Object]
impl InventoryItemQuery {
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
