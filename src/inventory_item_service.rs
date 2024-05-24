use async_graphql::Object;
use async_graphql::ID;

#[derive(Debug, Clone)]
pub struct InventoryItem {
    pub uuid: ID,
    pub name: String,
    pub level: u16,
    pub traits: Vec<String>,
    pub activation_cost: String,
    pub bulk: f32,
    pub description: String,
    pub usage_requirements: String,
    pub value: u64,
    pub effect: String,
    pub quantity: u32,
}

#[Object]
impl InventoryItem {
    async fn uuid(&self) -> &ID {
        &self.uuid
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn level(&self) -> u16 {
        self.level
    }

    async fn quantity(&self) -> u32 {
        self.quantity
    }

    async fn traits(&self) -> &Vec<String> {
        &self.traits
    }

    async fn activation_cost(&self) -> &str {
        &self.activation_cost
    }

    async fn bulk(&self) -> f32 {
        self.bulk
    }

    async fn description(&self) -> &str {
        &self.description
    }

    async fn usage_requirements(&self) -> &str {
        &self.usage_requirements
    }

    async fn value(&self) -> u64 {
        self.value
    }

    async fn effect(&self) -> &str {
        &self.effect
    }
}
