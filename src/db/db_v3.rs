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

    pub async fn get_inventory_items(
        &self,
        inventory_uuid: String,
        page: u32,
        page_size: u32,
    ) -> Option<PaginatedResponse<InventoryItem>> {
        let skip = (page - 1) * page_size;
        let query =
            "MATCH(inv:Inventory{uuid: $uuid}) Match(inv)-[c:CONTAINS]->(item:Item)
                        OPTIONAL MATCH (item)-[:HAS_BASE]->(base:ItemBase) return
                        item.uuid as uuid,
                        c.quantity as quantity,
                        COALESCE(item.effect, 'No effect') as effect,
                        COALESCE(item.level, 0) as level,
                        item.value as value,
                        [] as traits,
                        toFloat(COALESCE(item.bulk, base.bulk)) as bulk,
                        COALESCE(base.name +' ('+item.name+')', item.name) as name,
                        COALESCE(item.description, base.description, 'No description') as description,
                        COALESCE(item.activation_cost, base.activation_cost, 'Not activatible') as activation_cost,
                        COALESCE(item.usage_requirements, base.usage_requirements) as usage_requirements SKIP $skip LIMIT $limit";
        let parameters = neo4rs::query(query)
            .params([("uuid", inventory_uuid)])
            .params([("skip", skip), ("limit", page_size)]);

        let mut result = self.graph.execute(parameters).await.unwrap();
        let mut items = Vec::new();
        while let Ok(Some(row)) = result.next().await {
            items.push(self.parse_inventory_item(&row).unwrap());
        }
        Some(PaginatedResponse {
            entities: items,
            page: 1,
            page_size: 10,
            total: 10,
            total_pages: 1,
        })
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
