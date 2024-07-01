use crate::graphql::schemas::{
    inventory_item_schema::InventoryItemQueryFilter,
    inventory_with_items_schema::InventoryWithItems,
};

use super::{
    inventory_item_model::InventoryItemModelManager, inventory_model::InventoryModelManager,
};

pub struct InventoryWithItemsModelManager {
    inventory_items_model_manager: InventoryItemModelManager,
    inventory_model_manager: InventoryModelManager,
}

impl InventoryWithItemsModelManager {
    pub fn new(
        inventory_items_model_manager: InventoryItemModelManager,
        inventory_model_manager: InventoryModelManager,
    ) -> Self {
        Self {
            inventory_items_model_manager,
            inventory_model_manager,
        }
    }

    pub async fn get_inventory_with_items_by_owner_name(
        &self,
        name_term: String,
        page_index: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> Option<InventoryWithItems> {
        let inventory = self
            .inventory_model_manager
            .get_inventory_by_owner_name(name_term)
            .await;
        if inventory.is_none() {
            return None;
        }
        let items = self
            .inventory_items_model_manager
            .get_inventory_items(
                inventory.as_ref().unwrap().uuid.clone().to_string(),
                page_index,
                page_size,
                order_by,
                order_direction,
                filter,
            )
            .await;
        return Some(InventoryWithItems {
            inventory: inventory.unwrap(),
            items: items.unwrap(),
        });
    }
}
