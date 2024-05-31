use crate::db::DB;
use crate::inventory_item_service::{InventoryItem, InventoryItemQueryFilter};
use crate::inventory_service::Inventory;
use crate::inventory_with_items_service::InventoryWithItems;
use crate::pagination_service::PaginatedResponse;
use async_graphql::Object;

pub struct Query {
    pub db: DB,
}
