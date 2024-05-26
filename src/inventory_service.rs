use async_graphql::ID;
use async_graphql::{InputObject, Object};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Inventory {
    pub uuid: ID,
    pub name: String,
    pub capacity: u16,
    pub cp: u32,
    pub sp: u32,
    pub gp: u32,
    pub pp: u32,
}

#[Object]
impl Inventory {
    async fn uuid(&self) -> &str {
        &self.uuid
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn capacity(&self) -> u16 {
        self.capacity
    }

    async fn cp(&self) -> u32 {
        self.cp
    }

    async fn sp(&self) -> u32 {
        self.sp
    }

    async fn gp(&self) -> u32 {
        self.gp
    }

    async fn pp(&self) -> u32 {
        self.pp
    }
}

#[derive(Debug, Clone, InputObject)]
pub struct InventoryItemQueryFilter {
    pub search_value: Option<String>,
    pub include_traits: Option<Vec<String>>,
    pub exclude_traits: Option<Vec<String>>,
}

impl InventoryItemQueryFilter {
    // Simple method to generate Cypher query from the filter
    pub fn to_cypher_query(&self, base_query: &str) -> (String, HashMap<String, String>) {
        let mut query_conditions = Vec::new();
        let mut params = HashMap::new();

        if let Some(ref search_value) = self.search_value {
            query_conditions.push("toLower(combined_name) CONTAINS toLower($search_value)");
            params.insert("search_value".to_string(), search_value.clone());
        }

        if let Some(ref include_traits) = self.include_traits {
            query_conditions.push("ANY(tag IN n.tags WHERE tag IN $include_traits)");
            params.insert("include_tags".to_string(), include_traits.join(","));
        }

        if let Some(ref exclude_traits) = self.exclude_traits {
            query_conditions.push("NONE(tag IN n.tags WHERE tag IN $exclude_traits)");
            params.insert("exclude_tags".to_string(), exclude_traits.join(","));
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
