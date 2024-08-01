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

    async fn name(&self) -> &str {
        &self.item.name
    }

    async fn level(&self) -> u16 {
        self.item.level
    }

    async fn quantity(&self) -> u32 {
        self.quantity
    }

    async fn traits(&self) -> &Vec<String> {
        &self.item.traits
    }

    async fn activation_cost(&self) -> &str {
        &self.item.activation_cost
    }

    async fn bulk(&self) -> f32 {
        self.item.bulk
    }

    async fn display_bulk(&self) -> &str {
        &self.item.display_bulk
    }

    async fn description(&self) -> &str {
        &self.item.description
    }

    async fn usage_requirements(&self) -> &str {
        &self.item.usage_requirements
    }

    async fn value(&self) -> u64 {
        self.item.value
    }

    async fn display_value(&self) -> &str {
        &self.item.display_value
    }

    async fn effect(&self) -> &str {
        &self.item.effect
    }

    async fn is_consumable(&self) -> bool {
        self.item.traits.contains(&"Consumable".to_string())
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct InventoryItemQuantityAdjustmentParams {
    pub item_id: String,
    pub quantity_change: i32,
}
