use async_graphql::SimpleObject;

use crate::graphql::schemas::{
    inventory_item_schema::InventoryItem, inventory_schema::Inventory,
    paginated_response_schema::PaginatedResponse,
};
#[derive(SimpleObject)]
pub struct InventoryWithItems {
    pub inventory: Inventory,
    pub items: PaginatedResponse<InventoryItem>,
}
