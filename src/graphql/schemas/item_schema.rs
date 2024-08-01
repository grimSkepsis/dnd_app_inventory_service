use async_graphql::ID;
use async_graphql::{InputObject, Object};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Item {
    pub uuid: ID,
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

#[Object]
impl Item {
    async fn uuid(&self) -> &ID {
        &self.uuid
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn level(&self) -> u16 {
        self.level
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

    async fn display_bulk(&self) -> &str {
        &self.display_bulk
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

    async fn display_value(&self) -> &str {
        &self.display_value
    }

    async fn effect(&self) -> &str {
        &self.effect
    }

    pub async fn is_consumable(&self) -> bool {
        self.traits.contains(&"Consumable".to_string())
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
