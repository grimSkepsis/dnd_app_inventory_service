use crate::inventory_item_service::InventoryItem;
use crate::inventory_service::Inventory;
use crate::pagination_service::PaginatedResponse;
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
    ) -> Option<PaginatedResponse<InventoryItem>> {
        let skip = (page - 1) * page_size;
        let query = "MATCH(inv:Inventory{uuid: $uuid}) Match(inv)-[c:CONTAINS]->(item:Item)
                        OPTIONAL MATCH (item)-[:HAS_BASE]->(base:ItemBase)
                        OPTIONAL MATCH (item)-[:HAS_TRAIT]->(trait:Trait)
                        OPTIONAL MATCH (base)-[:HAS_TRAIT]->(baseTrait:Trait)
                        WITH item, base, c, COLLECT(trait.name) as item_traits, COLLECT(baseTrait.name) as base_traits
                        RETURN
                        item.uuid as uuid,
                        c.quantity as quantity,
                        COALESCE(item.effect, 'No effect') as effect,
                        COALESCE(item.level, 0) as level,
                        item.value as value,
                        (item_traits + base_traits) as traits,
                        toFloat(COALESCE(item.bulk, base.bulk)) as bulk,
                        COALESCE(base.name +' ('+item.name+')', item.name) as name,
                        COALESCE(item.description, base.description, 'No description') as description,
                        COALESCE(item.activation_cost, base.activation_cost, 'Not activatible') as activation_cost,
                        COALESCE(item.usage_requirements, base.usage_requirements) as usage_requirements
                        ORDER BY $$ <>
                        SKIP $skip LIMIT $limit"
                        .replace("$$", Self::map_sort_field(&order_by))
                        .replace("<>", if order_direction == "ASC" { "ASC" } else { "DESC" });
        print!("{}", query.clone());
        let count_query =
            "MATCH(inv:Inventory{uuid: $uuid}) Match(inv)-[c:CONTAINS]->(item:Item) RETURN count(item) as total";
        let parameters = neo4rs::query(&query)
            .params([("uuid", inventory_uuid.clone()), ("order_by", order_by)])
            .params([("skip", skip), ("limit", page_size)]);

        let mut result = self.graph.execute(parameters).await.unwrap();
        let mut count_result = self
            .graph
            .execute(neo4rs::query(count_query).param("uuid", inventory_uuid))
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
