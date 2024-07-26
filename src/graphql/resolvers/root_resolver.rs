use crate::graphql::resolvers::{
    inventory_item_resolver::InventoryItemQuery, inventory_resolver::InventoryQuery,
    inventory_with_items_resolver::InventoryWithItemsQuery, item_resolver::ItemQuery,
};
use crate::models::{
    inventory_item_model::InventoryItemModelManager, inventory_model::InventoryModelManager,
    inventory_with_items_model::InventoryWithItemsModelManager, item_model::ItemModelManager,
};
use async_graphql::Object;

use super::inventory_item_resolver::InventoryItemMutation;

pub struct QueryRoot {
    inventory: InventoryQuery,
    inventory_items: InventoryItemQuery,
    inventory_with_items: InventoryWithItemsQuery,
    items: ItemQuery,
}

impl QueryRoot {
    pub fn new(
        inventory_model_manager: InventoryModelManager,
        inventory_item_model_manager: InventoryItemModelManager,
        inventory_with_items_model_manager: InventoryWithItemsModelManager,
        item_model_manager: ItemModelManager,
    ) -> Self {
        Self {
            inventory: InventoryQuery::new(inventory_model_manager),
            inventory_items: InventoryItemQuery::new(inventory_item_model_manager),
            inventory_with_items: InventoryWithItemsQuery::new(inventory_with_items_model_manager),
            items: ItemQuery::new(item_model_manager),
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

    async fn items(&self) -> &ItemQuery {
        &self.items
    }

    async fn inventory_with_items(&self) -> &InventoryWithItemsQuery {
        &self.inventory_with_items
    }
}

pub struct MutationRoot {
    inventory_items: InventoryItemMutation,
}
impl MutationRoot {
    pub fn new(inventory_item_model_manager: InventoryItemModelManager) -> Self {
        Self {
            inventory_items: InventoryItemMutation::new(inventory_item_model_manager),
        }
    }
}

#[Object]
impl MutationRoot {
    async fn inventory_items(&self) -> &InventoryItemMutation {
        &self.inventory_items
    }
}
