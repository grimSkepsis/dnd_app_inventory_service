use crate::graphql::resolvers::{
    inventory_item_resolver::InventoryItemQuery, inventory_resolver::InventoryQuery,
    inventory_with_items_resolver::InventoryWithItemsQuery,
};
use crate::models::inventory_item_model::InventoryItemModelManager;
use crate::models::inventory_model::InventoryModelManager;
use crate::models::inventory_with_items_model::InventoryWithItemsModelManager;
use async_graphql::Object;

pub struct QueryRoot {
    inventory: InventoryQuery,
    inventory_items: InventoryItemQuery,
    inventory_with_items: InventoryWithItemsQuery,
}

impl QueryRoot {
    pub fn new(
        inventory_model_manager: InventoryModelManager,
        inventory_item_model_manager: InventoryItemModelManager,
        inventory_with_items_model_manager: InventoryWithItemsModelManager,
    ) -> Self {
        Self {
            inventory: InventoryQuery::new(inventory_model_manager),
            inventory_items: InventoryItemQuery::new(inventory_item_model_manager),
            inventory_with_items: InventoryWithItemsQuery::new(inventory_with_items_model_manager),
        }
    }
}
#[Object]
impl QueryRoot {
    async fn inventory(&self) -> &InventoryQuery {
        &self.inventory
    }

    async fn inventory_items(&self) -> &InventoryItemQuery {
        &self.inventory_items
    }

    async fn inventory_with_items(&self) -> &InventoryWithItemsQuery {
        &self.inventory_with_items
    }
}
