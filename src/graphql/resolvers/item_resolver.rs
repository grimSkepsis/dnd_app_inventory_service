use crate::{
    graphql::schemas::{
        item_schema::{Item, ItemProperties, ItemQueryFilter},
        paginated_response_schema::PaginatedResponse,
        trait_schema::Trait,
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

    pub async fn get_item(&self, id: String) -> Option<Item> {
        self.item_model_manager.get_item(&id).await
    }

    pub async fn get_traits(&self) -> Vec<Trait> {
        self.item_model_manager.get_traits().await
    }
}
pub struct ItemMutation {
    item_model_manager: ItemModelManager,
}
impl ItemMutation {
    pub fn new(item_model_manager: ItemModelManager) -> Self {
        Self { item_model_manager }
    }
}

#[Object]
impl ItemMutation {
    pub async fn create_item(&self, params: ItemProperties) -> Option<Item> {
        let res = self.item_model_manager.create_item(params).await;
        res
    }

    pub async fn update_item(&self, item_uuid: String, params: ItemProperties) -> Option<Item> {
        let res = self.item_model_manager.update_item(item_uuid, params).await;
        res
    }
}
