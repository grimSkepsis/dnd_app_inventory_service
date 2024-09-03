use std::sync::Arc;

use crate::graphql::schemas::{
    item_schema::{Item, ItemProperties, ItemQueryFilter},
    paginated_response_schema::PaginatedResponse,
};
use neo4rs::{Graph, Row};

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
                        item.uuid as uuid,
                        COALESCE(item.effect, 'No effect') as effect,
                        COALESCE(item.level, 0) as level,
                        item.value as value,
                        item_traits as traits,
                        toFloat(COALESCE(item.bulk, 0)) as bulk,
                        item.name as name,
                        COALESCE(item.description,  'No description') as description,
                        COALESCE(item.activation_cost,'Not activatible') as activation_cost,
                        COALESCE(item.usage_requirements, 'Not usable') as usage_requirements
                        ORDER BY <ORDER_FIELD> <ORDER_DIR>, uuid DESC
                        SKIP $skip LIMIT $limit"
                .replace("<ORDER_FIELD>", self.map_sort_field(&order_by))
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

    pub fn parse_item(&self, row: &Row) -> Option<Item> {
        let node_properties = row;

        Some(Item {
            uuid: node_properties.get("uuid").unwrap(),
            properties: ItemProperties {
                name: node_properties.get("name").unwrap(),
                value: node_properties.get("value").unwrap_or_default(),
                bulk: node_properties.get("bulk").unwrap_or_default(),
                display_bulk: Self::calc_display_bulk(
                    node_properties.get("bulk").unwrap_or_default(),
                ),
                description: node_properties.get("description").unwrap_or_default(),
                effect: node_properties.get("effect").unwrap_or_default(),
                level: node_properties.get("level").unwrap_or_default(),
                traits: node_properties.get("traits").unwrap_or_default(),
                activation_cost: node_properties.get("activation_cost").unwrap_or_default(),
                usage_requirements: node_properties
                    .get("usage_requirements")
                    .unwrap_or_default(),
                display_value: Self::calc_display_value(
                    node_properties.get("value").unwrap_or_default(),
                ),
            },
        })
    }

    fn calc_display_value(value: u64) -> Option<String> {
        let gp_value = value as f32 / 1000.0;
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
            "value" => "value",
            "level" => "level",
            "bulk" => "bulk",
            _ => "name", // Default field if input does not match
        }
    }
}
