use std::sync::Arc;

use crate::graphql::schemas::{
    inventory_schema::{Inventory, InventoryCurrencyChangeInput},
    paginated_response_schema::PaginatedResponse,
};
use neo4rs::{query, BoltNode, Graph, Query, Row};

pub struct InventoryModelManager {
    graph: Arc<Graph>,
}

impl InventoryModelManager {
    pub fn new(graph: Arc<Graph>) -> Self {
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

    pub async fn get_inventories(&self) -> PaginatedResponse<Inventory> {
        let query = "MATCH(inv:Inventory) return (inv)";
        let parameters = neo4rs::query(query);
        let mut result = self.graph.execute(parameters).await.unwrap();

        let mut inventories = Vec::<Inventory>::new();
        while let Ok(Some(row)) = result.next().await {
            inventories.push(self.parse_inventory(row).unwrap());
        }

        let total_entities = inventories.len();
        let total_pages = 1;

        return PaginatedResponse {
            entities: inventories,
            page_index: 0,
            page_size: total_entities as u32,
            total_entities: total_entities as u32,
            total_pages,
        };
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

    pub async fn get_inventory_by_owner_name(&self, name: String) -> Option<Inventory> {
        let query = "MATCH(onwer)-[:OWNS]->(inv:Inventory) WHERE toLower(onwer.name) CONTAINS toLower($name) return (inv)";
        let parameters = neo4rs::query(query).param("name", name);
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

    pub async fn update_inventory_currency(
        &self,
        inventory_id: String,
        params: InventoryCurrencyChangeInput,
    ) -> Option<Inventory> {
        let query = self.get_adjust_inventory_currency_query(
            inventory_id,
            (params.pp, params.gp, params.sp, params.cp),
        );
        let mut result = self.graph.execute(query).await.unwrap();
        if let Ok(Some(row)) = result.next().await {
            return self.parse_inventory(row);
        }
        None
    }

    fn get_adjust_inventory_currency_query(
        &self,
        inventory_uuid: String,
        currency: (i32, i32, i32, i32),
    ) -> Query {
        query(
            "MATCH (inv:Inventory {uuid: $inventory_uuid})
            SET inv.pp = inv.pp + $pp, inv.gp = inv.gp + $gp, inv.sp = inv.sp + $sp, inv.cp = inv.cp + $cp
            RETURN inv",
        )
        .param("inventory_uuid", inventory_uuid)
        .param("pp", currency.0)
        .param("gp", currency.1)
        .param("sp", currency.2)
        .param("cp", currency.3)
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
