use async_graphql::SimpleObject;

use crate::{
    inventory_item_service::InventoryItem, inventory_service::Inventory,
    pagination_service::PaginatedResponse,
};
#[derive(SimpleObject)]
pub struct InventoryWithItems {
    pub inventory: Inventory,
    pub items: PaginatedResponse<InventoryItem>,
}
