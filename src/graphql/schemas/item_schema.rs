use async_graphql::ID;
use async_graphql::{InputObject, Object};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ItemProperties {
    pub name: String,
    pub level: Option<u16>,
    pub traits: Option<Vec<String>>,
    pub activation_cost: Option<String>,
    pub bulk: Option<f32>,
    pub display_bulk: Option<String>,
    pub description: Option<String>,
    pub usage_requirements: Option<String>,
    pub value: Option<u64>,
    pub display_value: Option<String>,
    pub effect: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub uuid: ID,
    pub properties: ItemProperties,
    // pub name: String,
    // pub level: u16,
    // pub traits: Vec<String>,
    // pub activation_cost: String,
    // pub bulk: f32,
    // pub display_bulk: String,
    // pub description: String,
    // pub usage_requirements: String,
    // pub value: u64,
    // pub display_value: String,
    // pub effect: String,
}

#[Object]
impl Item {
    async fn uuid(&self) -> &ID {
        &self.uuid
    }

    async fn name(&self) -> &str {
        &self.properties.name
    }

    async fn level(&self) -> Option<u16> {
        self.properties.level
    }

    async fn traits(&self) -> Option<&Vec<String>> {
        self.properties.traits.as_ref()
    }

    async fn activation_cost(&self) -> Option<&String> {
        self.properties.activation_cost.as_ref()
    }

    async fn bulk(&self) -> Option<f32> {
        self.properties.bulk
    }

    async fn display_bulk(&self) -> Option<&String> {
        self.properties.display_bulk.as_ref()
    }

    async fn description(&self) -> Option<&String> {
        self.properties.description.as_ref()
    }

    async fn usage_requirements(&self) -> Option<&String> {
        self.properties.usage_requirements.as_ref()
    }

    async fn value(&self) -> Option<u64> {
        self.properties.value
    }

    async fn display_value(&self) -> Option<&String> {
        self.properties.display_value.as_ref()
    }

    async fn effect(&self) -> Option<&String> {
        self.properties.effect.as_ref()
    }

    pub async fn is_consumable(&self) -> bool {
        if let Some(ref traits) = self.properties.traits {
            traits.contains(&"Consumable".to_string())
        } else {
            false
        }
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct ItemQueryFilter {
    pub search_value: Option<String>,
    pub included_traits: Option<Vec<String>>,
    pub excluded_traits: Option<Vec<String>>,
}

impl ItemQueryFilter {
    // Simple method to generate Cypher query from the filter
    pub fn to_cypher_query(&self, base_query: &str) -> (String, HashMap<String, String>) {
        let mut query_conditions = Vec::<String>::new();
        let mut params = HashMap::new();

        if let Some(ref search_value) = self.search_value {
            query_conditions.push("toLower(item.name) CONTAINS toLower($search_value)".to_string());
            params.insert("search_value".to_string(), search_value.clone());
        }

        if let Some(ref included_traits) = self.included_traits {
            for (idx, trait_name) in included_traits.iter().enumerate() {
                let condition =
                    "(item)-[:HAS_TRAIT]->(:Trait{name: $it<>})".replace("<>", &idx.to_string());
                query_conditions.push(condition);
                params.insert(format!("it{}", idx), trait_name.clone());
            }

            params.insert("included_traits".to_string(), included_traits.join(","));
        }

        if let Some(ref excluded_traits) = self.excluded_traits {
            for (idx, trait_name) in excluded_traits.iter().enumerate() {
                let condition = "NOT (item)-[:HAS_TRAIT]->(:Trait{name: $et<>})"
                    .replace("<>", &idx.to_string());
                query_conditions.push(condition);
                params.insert(format!("et{}", idx), trait_name.clone());
            }
        }

        let full_query = if query_conditions.is_empty() {
            base_query.to_string().replace("<FILTER>", "")
        } else {
            base_query.to_string().replace(
                "<FILTER>",
                &("WHERE ".to_string() + &query_conditions.join(" AND ")),
            )
        };

        (full_query, params)
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct ItemCreationParams {
    pub name: String,
    pub level: u16,
    pub traits: Vec<String>,
    pub activation_cost: String,
    pub bulk: f32,
    pub display_bulk: String,
    pub description: String,
    pub usage_requirements: String,
    pub value: u64,
    pub display_value: String,
    pub effect: String,
}
