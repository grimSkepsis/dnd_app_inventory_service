use crate::db::DB;
use crate::graphql::resolvers::{
    inventory_item_resolver::InventoryItemQuery, inventory_resolver::InventoryQuery,
    inventory_with_items_resolver::InventoryWithItemsQuery,
};
use async_graphql::Object;
use std::sync::Arc;

pub struct QueryRoot {
    inventory: InventoryQuery,
    inventory_item: InventoryItemQuery,
    inventory_with_items: InventoryWithItemsQuery,
}

impl QueryRoot {
    pub fn new(db: Arc<DB>) -> Self {
        Self {
            inventory: InventoryQuery::new(db.clone()),
            inventory_item: InventoryItemQuery::new(db.clone()),
            inventory_with_items: InventoryWithItemsQuery::new(db.clone()),
        }
    }
}
#[Object]
impl QueryRoot {
    async fn user(&self) -> &InventoryQuery {
        &self.inventory
    }

    async fn product(&self) -> &InventoryItemQuery {
        &self.inventory_item
    }

    async fn inventory_with_items(&self) -> &InventoryWithItemsQuery {
        &self.inventory_with_items
    }
}
