use crate::graphql::schemas::inventory_schema::Inventory;
use crate::models::inventory_model::InventoryModelManager;
use async_graphql::Object;

pub struct InventoryQuery {
    inventory_model_manager: InventoryModelManager,
}

impl InventoryQuery {
    pub fn new(inventory_model_manager: InventoryModelManager) -> Self {
        Self {
            inventory_model_manager,
        }
    }
}

#[Object]
impl InventoryQuery {
    pub async fn get_inventory(&self, id: String) -> Option<Inventory> {
        self.inventory_model_manager.get_inventory_by_uuid(id).await
    }

    pub async fn get_chracter_inventory(&self, id: String) -> Option<Inventory> {
        self.inventory_model_manager
            .get_inventory_by_character_uuid(id)
            .await
    }

    pub async fn get_inventory_by_owner(&self, id: String) -> Option<Inventory> {
        self.inventory_model_manager
            .get_inventory_by_owner_uuid(id)
            .await
    }

    pub async fn get_inventory_by_owner_name(&self, name_term: String) -> Option<Inventory> {
        self.inventory_model_manager
            .get_inventory_by_owner_name(name_term)
            .await
    }
}
