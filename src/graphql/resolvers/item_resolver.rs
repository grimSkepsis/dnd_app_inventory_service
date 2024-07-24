use crate::{
    graphql::schemas::{
        item_schema::{Item, ItemQueryFilter},
        paginated_response_schema::PaginatedResponse,
    },
    models::item_model::ItemModelManager,
};
use async_graphql::Object;

pub struct ItemQuery {
    item_model_manager: ItemModelManager,
}

impl ItemQuery {
    pub fn new(item_model_manager: ItemModelManager) -> Self {
        Self { item_model_manager }
    }
}

#[Object]
impl ItemQuery {
    pub async fn get_items(
        &self,
        page_index: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: ItemQueryFilter,
    ) -> Option<PaginatedResponse<Item>> {
        self.item_model_manager
            .get_items(page_index, page_size, order_by, order_direction, filter)
            .await
    }
}
