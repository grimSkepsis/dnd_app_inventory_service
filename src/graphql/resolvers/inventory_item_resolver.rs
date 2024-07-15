use crate::{
    graphql::schemas::{
        inventory_item_schema::{InventoryItem, InventoryItemQueryFilter},
        paginated_response_schema::PaginatedResponse,
    },
    models::inventory_item_model::InventoryItemModelManager,
};
use async_graphql::Object;

pub struct InventoryItemQuery {
    inventory_item_model_manager: InventoryItemModelManager,
}
impl InventoryItemQuery {
    pub fn new(inventory_item_model_manager: InventoryItemModelManager) -> Self {
        Self {
            inventory_item_model_manager,
        }
    }
}

#[Object]
impl InventoryItemQuery {
    pub async fn get_inventory_items(
        &self,
        inventory_id: String,
        page_index: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> Option<PaginatedResponse<InventoryItem>> {
        self.inventory_item_model_manager
            .get_inventory_items(
                inventory_id,
                page_index,
                page_size,
                order_by,
                order_direction,
                filter,
            )
            .await
    }
}
