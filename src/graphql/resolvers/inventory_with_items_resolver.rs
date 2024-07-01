use crate::graphql::schemas::inventory_item_schema::InventoryItemQueryFilter;
use crate::graphql::schemas::inventory_with_items_schema::InventoryWithItems;
use crate::models::inventory_with_items_model::InventoryWithItemsModelManager;
use async_graphql::Object;

pub struct InventoryWithItemsQuery {
    inventory_with_items_model_manager: InventoryWithItemsModelManager,
}
impl InventoryWithItemsQuery {
    pub fn new(inventory_with_items_model_manager: InventoryWithItemsModelManager) -> Self {
        Self {
            inventory_with_items_model_manager,
        }
    }
}

#[Object]
impl InventoryWithItemsQuery {
    pub async fn get_inventory_with_items_by_owner_name(
        &self,
        name_term: String,
        page_index: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> Option<InventoryWithItems> {
        self.inventory_with_items_model_manager
            .get_inventory_with_items_by_owner_name(
                name_term,
                page_index,
                page_size,
                order_by,
                order_direction,
                filter,
            )
            .await
    }
}
