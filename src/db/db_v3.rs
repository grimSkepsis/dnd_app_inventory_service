use crate::inventory_service::Inventory;
use crate::pagination_service::PaginatedResponse;
use crate::{inventory_item_service::InventoryItem, inventory_service::InventoryItemQueryFilter};
use neo4rs::{BoltNode, Graph, Row};

pub struct DB {
    graph: Graph,
}

impl DB {
    pub fn new(graph: Graph) -> Self {
        Self { graph }
    }
    pub async fn get_inventory_by_character_uuid(&self, uuid: String) -> Option<Inventory> {
        let query = "MATCH(char:Character{uuid: $uuid})-[:OWNS]->(inv:Inventory) return (inv)";
        let parameters = neo4rs::query(query).param("uuid", uuid);
        let mut result = self.graph.execute(parameters).await.unwrap();
        if let Ok(Some(row)) = result.next().await {
            return self.parse_inventory(row);
        }
        None
    }

    pub async fn get_inventory_by_owner_uuid(&self, uuid: String) -> Option<Inventory> {
        let query = "MATCH(char{uuid: $uuid})-[:OWNS]->(inv:Inventory) return (inv)";
        let parameters = neo4rs::query(query).param("uuid", uuid);
        let mut result = self.graph.execute(parameters).await.unwrap();
        if let Ok(Some(row)) = result.next().await {
            return self.parse_inventory(row);
        }
        None
    }

    pub async fn get_inventory_by_uuid(&self, uuid: String) -> Option<Inventory> {
        let query = "MATCH(inv:Inventory{uuid: $uuid}) return (inv)";
        let parameters = neo4rs::query(query).param("uuid", uuid);
        let mut result = self.graph.execute(parameters).await.unwrap();
        if let Ok(Some(row)) = result.next().await {
            return self.parse_inventory(row);
        }
        None
    }
    fn map_sort_field(field: &str) -> &str {
        match field {
            "name" => "name",
            "value" => "value",
            "level" => "level",
            "quantity" => "quantity",
            "bulk" => "bulk",
            _ => "name", // Default field if input does not match
        }
    }
    pub async fn get_inventory_items(
        &self,
        inventory_uuid: String,
        page: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: InventoryItemQueryFilter,
    ) -> Option<PaginatedResponse<InventoryItem>> {
        let skip = (page - 1) * page_size;
        let (query, params) = filter.to_cypher_query(
            &"
                        MATCH(inv:Inventory{uuid: $uuid})
                        Match(inv)-[c:CONTAINS]->(item:Item)
                        <FILTER>
                        OPTIONAL MATCH (item)-[:HAS_TRAIT]->(trait:Trait)
                        WITH item,  c, COLLECT(trait.name) as item_traits

                        RETURN
                        item.uuid as uuid,
                        c.quantity as quantity,
                        COALESCE(item.effect, 'No effect') as effect,
                        COALESCE(item.level, 0) as level,
                        item.value as value,
                        item_traits as traits,
                        toFloat(COALESCE(item.bulk, 0)) as bulk,
                        item.name as name,
                        COALESCE(item.description,  'No description') as description,
                        COALESCE(item.activation_cost,'Not activatible') as activation_cost,
                        COALESCE(item.usage_requirements, 'Not usable') as usage_requirements
                        ORDER BY <ORDER_FIELD> <ORDER_DIR>
                        SKIP $skip LIMIT $limit"
                .replace("<ORDER_FIELD>", Self::map_sort_field(&order_by))
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

        let (count_query, count_params) = filter.to_cypher_query("MATCH(inv:Inventory{uuid: $uuid}) Match(inv)-[c:CONTAINS]->(item:Item) <FILTER> RETURN count(item) as total");

        let mut result = self
            .graph
            .execute(
                neo4rs::query(&query)
                    .params(params)
                    .params([("uuid", inventory_uuid.clone()), ("order_by", order_by)])
                    .params([("skip", skip), ("limit", page_size)]),
            )
            .await
            .unwrap();

        let mut count_result = self
            .graph
            .execute(
                neo4rs::query(&count_query)
                    .params(count_params)
                    .param("uuid", inventory_uuid),
            )
            .await
            .unwrap();
        let mut items = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            items.push(self.parse_inventory_item(&row).unwrap());
        }
        if let Ok(Some(row)) = count_result.next().await {
            let total_entities = row.get("total").unwrap();
            let total_pages = (total_entities as f32 / page_size as f32).ceil() as u32;
            return Some(PaginatedResponse {
                entities: items,
                page,
                page_size,
                total_entities,
                total_pages,
            });
        }
        None
    }

    fn parse_inventory_item(&self, row: &Row) -> Option<InventoryItem> {
        let node_properties = row;

        Some(InventoryItem {
            uuid: node_properties.get("uuid").unwrap(),
            name: node_properties.get("name").unwrap(),
            value: node_properties.get("value").unwrap(),
            bulk: node_properties.get("bulk").unwrap(),
            quantity: node_properties.get("quantity").unwrap(),
            description: node_properties.get("description").unwrap(),
            effect: node_properties.get("effect").unwrap(),
            level: node_properties.get("level").unwrap(),
            traits: node_properties.get("traits").unwrap(),
            activation_cost: node_properties.get("activation_cost").unwrap(),
            usage_requirements: node_properties.get("usage_requirements").unwrap(),
        })
    }

    fn parse_inventory(&self, row: Row) -> Option<Inventory> {
        let node_properties = row.get::<BoltNode>("inv").unwrap().properties;
        Some(Inventory {
            uuid: node_properties.get("uuid").unwrap(),
            name: node_properties.get("name").unwrap(),
            capacity: node_properties.get("capacity").unwrap(),
            cp: node_properties.get("cp").unwrap(),
            sp: node_properties.get("sp").unwrap(),
            gp: node_properties.get("gp").unwrap(),
            pp: node_properties.get("pp").unwrap(),
        })
    }
}
