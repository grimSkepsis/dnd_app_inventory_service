use crate::{
    graphql::schemas::{
        inventory_item_schema::{InventoryItem, InventoryItemQuantityAdjustmentParams},
        item_schema::ItemQueryFilter,
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
        filter: ItemQueryFilter,
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

pub struct InventoryItemMutation {
    inventory_item_model_manager: InventoryItemModelManager,
}
impl InventoryItemMutation {
    pub fn new(inventory_item_model_manager: InventoryItemModelManager) -> Self {
        Self {
            inventory_item_model_manager,
        }
    }
}

#[Object]
impl InventoryItemMutation {
    pub async fn add_or_remove_items_from_inventory(
        &self,
        inventory_id: String,
        items: Vec<InventoryItemQuantityAdjustmentParams>,
    ) -> bool {
        let res = self
            .inventory_item_model_manager
            .add_or_remove_items_from_inventory(inventory_id, items)
            .await;
        res
    }

    pub async fn sell_items(
        &self,
        inventory_id: String,
        items: Vec<InventoryItemQuantityAdjustmentParams>,
    ) -> bool {
        let res = self
            .inventory_item_model_manager
            .sell_items(inventory_id, items)
            .await;
        res
    }
}
