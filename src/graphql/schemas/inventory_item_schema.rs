use crate::graphql::schemas::item_schema::Item;
use async_graphql::{InputObject, Object, ID};

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub item: Item,
    pub quantity: u32,
}

#[Object]
impl InventoryItem {
    async fn uuid(&self) -> &ID {
        &self.item.uuid
    }

    async fn name(&self) -> Option<&String> {
        self.item.properties.name.as_ref()
    }

    async fn level(&self) -> Option<u16> {
        self.item.properties.level
    }

    async fn quantity(&self) -> u32 {
        self.quantity
    }

    async fn traits(&self) -> Option<&Vec<String>> {
        self.item.properties.traits.as_ref()
    }

    async fn activation_cost(&self) -> Option<&str> {
        self.item.properties.activation_cost.as_deref()
    }

    async fn bulk(&self) -> Option<f32> {
        self.item.properties.bulk
    }

    async fn display_bulk(&self) -> Option<&str> {
        self.item.display_bulk.as_deref()
    }

    async fn description(&self) -> Option<&str> {
        self.item.properties.description.as_deref()
    }

    async fn usage_requirements(&self) -> Option<&str> {
        self.item.properties.usage_requirements.as_deref()
    }

    async fn value(&self) -> Option<u64> {
        self.item.properties.value
    }

    async fn display_value(&self) -> Option<&str> {
        self.item.display_value.as_deref()
    }

    async fn effect(&self) -> Option<&str> {
        self.item.properties.effect.as_deref()
    }

    async fn is_consumable(&self) -> bool {
        match &self.item.properties.traits {
            Some(traits) => traits.contains(&"Consumable".to_string()),
            None => false,
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct InventoryItemQuantityAdjustmentParams {
    pub item_id: String,
    pub quantity_change: i32,
}
