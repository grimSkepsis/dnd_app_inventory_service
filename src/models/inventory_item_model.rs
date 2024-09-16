use std::sync::Arc;

use crate::graphql::schemas::inventory_item_schema::InventoryItemQuantityAdjustmentParams;
use crate::graphql::schemas::{
    inventory_item_schema::InventoryItem, item_schema::ItemQueryFilter,
    paginated_response_schema::PaginatedResponse,
};
use crate::models::item_model::ItemModelManager;
use neo4rs::{query, Graph, Query, Row};

pub struct InventoryItemModelManager {
    graph: Arc<Graph>,
    item_model_manager: ItemModelManager,
}

impl InventoryItemModelManager {
    pub fn new(graph: Arc<Graph>, item_model_manager: ItemModelManager) -> Self {
        Self {
            graph,
            item_model_manager,
        }
    }

    pub async fn get_inventory_items(
        &self,
        inventory_uuid: String,
        page_index: u32,
        page_size: u32,
        order_by: String,
        order_direction: String,
        filter: ItemQueryFilter,
    ) -> Option<PaginatedResponse<InventoryItem>> {
        let skip = page_index * page_size;
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
                        toInteger(COALESCE(item.value, '0')) AS numeric_value,
                        item_traits as traits,
                        toFloat(COALESCE(item.bulk, 0)) as bulk,
                        item.name as name,
                        COALESCE(item.description,  'No description') as description,
                        COALESCE(item.activation_cost,'n/a') as activation_cost,
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
                page_index,
                page_size,
                total_entities,
                total_pages,
            });
        }
        None
    }

    pub async fn add_or_remove_items_from_inventory(
        &self,
        inventory_uuid: String,
        items: Vec<InventoryItemQuantityAdjustmentParams>,
    ) -> bool {
        // Create a new session
        let mut txn = self.graph.start_txn().await.unwrap();

        //for all decrement operations, ensure we have enough quantity to decrement without going negative
        for item in &items {
            if item.quantity_change < 0 {
                let result =
                    self.graph
                        .execute(self.get_current_item_quantities_query(
                            inventory_uuid.clone(),
                            item.clone(),
                        ))
                        .await;
                if result.is_err() {
                    txn.rollback().await.unwrap();
                    return false;
                }
                let row = result.unwrap().next().await.unwrap().unwrap();
                let current_quantity: i32 = row.get("quantity").unwrap();
                if current_quantity < item.quantity_change.abs() {
                    txn.rollback().await.unwrap();
                    return false;
                }
            }
        }

        let result = txn
            .run_queries(
                items
                    .into_iter()
                    .map(|item| self.get_item_adjustment_query(inventory_uuid.clone(), item)),
            )
            .await;

        if result.is_ok() {
            txn.commit().await.unwrap();
            true
        } else {
            txn.rollback().await.unwrap();
            false
        }
    }

    pub async fn sell_items(
        &self,
        inventory_uuid: String,
        items: Vec<InventoryItemQuantityAdjustmentParams>,
    ) -> bool {
        // Create a new session
        let mut txn = self.graph.start_txn().await.unwrap();

        //for all sell operations, ensure we have enough quantity to decrement without going negative
        for item in &items {
            //if the quantity change is positive, error since we are selling
            if item.quantity_change >= 0 {
                txn.rollback().await.unwrap();
                return false;
            }
            let result = self
                .graph
                .execute(
                    self.get_current_item_quantities_query(inventory_uuid.clone(), item.clone()),
                )
                .await;
            if result.is_err() {
                txn.rollback().await.unwrap();
                return false;
            }
            let row = result.unwrap().next().await.unwrap().unwrap();
            let current_quantity: i32 = row.get("quantity").unwrap();
            if current_quantity < item.quantity_change.abs() {
                txn.rollback().await.unwrap();
                return false;
            }
        }

        let result = txn
            .run_queries(
                items
                    .into_iter()
                    .map(|item| self.get_item_adjustment_query(inventory_uuid.clone(), item)),
            )
            .await;

        if result.is_ok() {
            txn.commit().await.unwrap();
            true
        } else {
            txn.rollback().await.unwrap();
            false
        }
    }

    fn get_current_item_quantities_query(
        &self,
        inventory_uuid: String,
        item: InventoryItemQuantityAdjustmentParams,
    ) -> Query {
        // Execute the query and process the results
        return query(
            "MATCH (inv:Inventory {uuid: $inventory_uuid}), (item:Item {uuid: $item_uuid})
             OPTIONAL MATCH (inv)-[rel:CONTAINS]->(item:Item)
             RETURN item.uuid AS item_uuid, COALESCE(rel.quantity, 0) AS quantity",
        )
        .param("inventory_uuid", inventory_uuid.clone())
        .param("item_uuid", item.item_id.clone());
    }

    fn get_item_adjustment_query(
        &self,
        inventory_uuid: String,
        item: InventoryItemQuantityAdjustmentParams,
    ) -> Query {
        return query(
            "MATCH (inv:Inventory {uuid: $inventory_uuid}), (item:Item {uuid: $item_uuid})
            MERGE (inv)-[rel:CONTAINS]->(item)
            ON CREATE SET rel.quantity = $quantity_change
            ON MATCH SET rel.quantity = rel.quantity + $quantity_change
            WITH inv, item, rel
            // Check if the new quantity is 0
            FOREACH (ignoreMe IN CASE WHEN rel.quantity = 0 THEN [1] ELSE [] END |
              DELETE rel
            )
            RETURN item, inv, rel",
        )
        .param("inventory_uuid", inventory_uuid.clone())
        .param("item_uuid", item.item_id.clone())
        .param("quantity_change", item.quantity_change.clone());
    }

    fn parse_inventory_item(&self, row: &Row) -> Option<InventoryItem> {
        Some(InventoryItem {
            item: self.item_model_manager.parse_item(row).unwrap(),
            quantity: row.get("quantity").unwrap(),
        })
    }

    fn map_sort_field(&self, field: &str) -> &str {
        match field {
            "quantity" => "quantity",
            _ => self.item_model_manager.map_sort_field(field), // Default field if input does not match
        }
    }
}
