use crate::graphql::schemas::{
    item_schema::{Item, ItemProperties, ItemQueryFilter},
    paginated_response_schema::PaginatedResponse,
    trait_schema::Trait,
};
use async_graphql::ID;
use neo4rs::{BoltType, Graph, Row};
use std::collections::HashMap;
use std::sync::Arc;

const ITEM_FIELD_PATTERN: &str = "item.uuid as uuid,
COALESCE(item.effect, 'No effect') as effect,
COALESCE(item.level, 0) as level,
item.value as value,
toInteger(COALESCE(item.value, '0')) AS numeric_value,
item_traits as traits,
toFloat(COALESCE(item.bulk, 0)) as bulk,
item.name as name,
COALESCE(item.description,  'No description') as description,
COALESCE(item.activation_cost,'n/a') as activation_cost,
COALESCE(item.usage_requirements, 'Not usable') as usage_requirements";

pub struct ItemModelManager {
    graph: Arc<Graph>,
}

impl ItemModelManager {
    pub fn new(graph: Arc<Graph>) -> Self {
        Self { graph }
    }

    pub async fn get_items(
        &self,
        page_index: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: ItemQueryFilter,
    ) -> Option<PaginatedResponse<Item>> {
        let skip = page_index * page_size;
        let (query, params) = filter.to_cypher_query(
            &"
                        MATCH (item:Item)
                        <FILTER>
                        OPTIONAL MATCH (item)-[:HAS_TRAIT]->(trait:Trait)
                        WITH item, COLLECT(trait.name) as item_traits
                        RETURN
                       <ITEM_FIELD_PATTERN>
                        ORDER BY <ORDER_FIELD> <ORDER_DIR>, uuid DESC
                        SKIP $skip LIMIT $limit"
                .replace("<ORDER_FIELD>", self.map_sort_field(&order_by))
                .replace("<ITEM_FIELD_PATTERN>", ITEM_FIELD_PATTERN)
                .replace(
                    "<ORDER_DIR>",
                    if order_direction == "ASC" {
                        "ASC"
                    } else {
                        "DESC"
                    },
                ),
        );
        print!("{}", query.clone());

        let (count_query, count_params) =
            filter.to_cypher_query("MATCH (item:Item) <FILTER> RETURN count(item) as total");

        let mut result = self
            .graph
            .execute(
                neo4rs::query(&query)
                    .params(params)
                    .params([("order_by", order_by)])
                    .params([("skip", skip), ("limit", page_size)]),
            )
            .await
            .unwrap();

        let mut count_result = self
            .graph
            .execute(neo4rs::query(&count_query).params(count_params))
            .await
            .unwrap();
        let mut items = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            items.push(self.parse_item(&row).unwrap());
        }
        if let Ok(Some(row)) = count_result.next().await {
            let total_entities = row.get("total").unwrap();
            let total_pages = (total_entities as f32 / page_size as f32).ceil() as u32;
            return Some(PaginatedResponse {
                entities: items,
                page_index,
                page_size,
                total_entities,
                total_pages,
            });
        }
        None
    }

    pub async fn get_item(&self, uuid: &str) -> Option<Item> {
        let query = format!(
            "MATCH (item:Item {{uuid: '{}'}}) OPTIONAL MATCH (item)-[:HAS_TRAIT]->(trait:Trait) WITH item, COLLECT(trait.name) as item_traits RETURN {}",
            uuid, ITEM_FIELD_PATTERN
        );
        let mut result = self.graph.execute(neo4rs::query(&query)).await.unwrap();
        if let Ok(Some(row)) = result.next().await {
            return self.parse_item(&row);
        }
        None
    }

    pub async fn get_traits(&self) -> Vec<Trait> {
        let query =
            "MATCH (trait:Trait) RETURN trait.name as name, trait.description as description ORDER BY name";
        let mut result = self.graph.execute(neo4rs::query(&query)).await.unwrap();
        let mut traits = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            traits.push(Trait {
                name: row.get("name").unwrap(),
                description: row.get("description").unwrap_or_default(),
            });
        }
        traits
    }

    pub async fn create_item(&self, properties: ItemProperties) -> Option<Item> {
        let mut params: HashMap<&str, BoltType> = HashMap::new();
        params.insert(
            "name",
            properties
                .name
                .unwrap_or("Unnamed Item".to_string())
                .to_string()
                .into(),
        );
        params.insert("level", properties.level.unwrap_or_default().into());
        params.insert(
            "activation_cost",
            properties.activation_cost.unwrap_or_default().into(),
        );
        params.insert("bulk", properties.bulk.unwrap_or_default().into());
        params.insert(
            "description",
            properties.description.unwrap_or_default().into(),
        );
        params.insert(
            "usage_requirements",
            properties.usage_requirements.unwrap_or_default().into(),
        );
        params.insert(
            "value",
            properties.value.unwrap_or_default().to_string().into(),
        );
        params.insert("effect", properties.effect.unwrap_or_default().into());

        // Build the Cypher query using parameterized placeholders
        let query_string = &"CREATE (item:Item {
            uuid: apoc.create.uuid(),
            name: $name,
            level: $level,
            activation_cost: $activation_cost,
            bulk: $bulk,
            description: $description,
            usage_requirements: $usage_requirements,
            value: $value,
            effect: $effect
        }) RETURN item.uuid as uuid";

        // Execute the query with parameters
        let mut result = self
            .graph
            .execute(neo4rs::query(query_string).params(params))
            .await
            .unwrap();

        if let Ok(Some(row)) = result.next().await {
            let uuid: ID = row.get("uuid").unwrap();
            return self.get_item(&uuid).await;
        }
        return None;
    }

    pub async fn update_item(&self, item_uuid: String, properties: ItemProperties) -> Option<Item> {
        let mut params: HashMap<&str, BoltType> = HashMap::new();

        if let Some(name) = properties.name {
            params.insert("name", name.into());
        }
        if let Some(level) = properties.level {
            params.insert("level", level.into());
        }
        if let Some(activation_cost) = properties.activation_cost {
            params.insert("activation_cost", activation_cost.into());
        }
        if let Some(bulk) = properties.bulk {
            params.insert("bulk", bulk.into());
        }
        if let Some(description) = properties.description {
            params.insert("description", description.into());
        }
        if let Some(usage_requirements) = properties.usage_requirements {
            params.insert("usage_requirements", usage_requirements.into());
        }
        if let Some(value) = properties.value {
            params.insert("value", value.to_string().into());
        }
        if let Some(effect) = properties.effect {
            params.insert("effect", effect.into());
        }

        // Build the SET clause for regular properties
        let set_clause: String = params
            .keys()
            .map(|key| format!("item.{} = ${}", key, key))
            .collect::<Vec<_>>()
            .join(", ");

        // Construct the final query string with trait handling
        let query_string = format!(
            "MATCH (item:Item {{uuid: $item_uuid}})
             SET {}
             WITH item
             // Remove all existing trait relationships
             OPTIONAL MATCH (item)-[r:HAS_TRAIT]->(:Trait)
             DELETE r
             WITH item
             // Create new trait relationships
             UNWIND $traits as trait_name
             MERGE (t:Trait {{name: trait_name}})
             MERGE (item)-[:HAS_TRAIT]->(t)
             RETURN item.uuid as uuid",
            set_clause
        );

        // Insert item UUID and traits into params
        params.insert("item_uuid", item_uuid.into());
        if let Some(traits) = properties.traits {
            params.insert("traits", traits.into());
        } else {
            params.insert("traits", Vec::<String>::new().into());
        }

        // Execute the query with parameters
        let mut result = self
            .graph
            .execute(neo4rs::query(&query_string).params(params))
            .await
            .unwrap();

        if let Ok(Some(row)) = result.next().await {
            let uuid: ID = row.get("uuid").unwrap();
            return self.get_item(&uuid).await;
        }
        None
    }

    pub fn parse_item(&self, row: &Row) -> Option<Item> {
        let node_properties = row;
        let value = Some(
            node_properties
                .get("value")
                .unwrap_or("0")
                .to_string()
                .parse::<u64>()
                .unwrap_or_default(),
        );

        Some(Item {
            uuid: node_properties.get("uuid").unwrap(),
            display_bulk: Self::calc_display_bulk(node_properties.get("bulk").unwrap_or_default()),
            display_value: Self::calc_display_value(value.unwrap()),
            properties: ItemProperties {
                name: node_properties.get("name").unwrap(),
                value,
                bulk: node_properties.get("bulk").unwrap_or_default(),
                description: node_properties.get("description").unwrap_or_default(),
                effect: node_properties.get("effect").unwrap_or_default(),
                level: node_properties.get("level").unwrap_or_default(),
                traits: node_properties.get("traits").unwrap_or_default(),
                activation_cost: node_properties.get("activation_cost").unwrap_or_default(),
                usage_requirements: node_properties
                    .get("usage_requirements")
                    .unwrap_or_default(),
            },
        })
    }

    fn calc_display_value(value: u64) -> Option<String> {
        let gp_value = value as f32 / 100.0;
        return Some(format!("{} gp", gp_value.to_string()));
    }

    fn calc_display_bulk(bulk_value: f32) -> Option<String> {
        match bulk_value {
            0.0 => return Some("Negligible".to_string()),
            0.1 => return Some("Light".to_string()),
            _ => return Some(format!("{} bulk", bulk_value.to_string())),
        }
    }

    pub fn map_sort_field(&self, field: &str) -> &str {
        match field {
            "name" => "name",
            "value" => "numeric_value",
            "level" => "level",
            "bulk" => "bulk",
            _ => "name", // Default field if input does not match
        }
    }
}
