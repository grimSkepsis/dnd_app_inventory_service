use crate::graphql::schemas::{
    inventory_item_schema::InventoryItem, inventory_schema::Inventory, item_schema::Item,
};
use async_graphql::{OutputType, SimpleObject};

#[derive(Debug, Clone, SimpleObject)]
#[graphql(concrete(name = "PaginatedItemResponse", params(Item)))]
#[graphql(concrete(name = "PaginatedInventoryItemResponse", params(InventoryItem)))]
#[graphql(concrete(name = "PaginatedInventoryResponse", params(Inventory)))]
pub struct PaginatedResponse<T: OutputType> {
    pub entities: Vec<T>,
    pub total_entities: u32,
    pub page_index: u32,
    pub page_size: u32,
    pub total_pages: u32,
}
