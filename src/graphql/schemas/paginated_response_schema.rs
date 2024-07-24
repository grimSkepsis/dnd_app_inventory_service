use async_graphql::{OutputType, SimpleObject};

#[derive(Debug, Clone, SimpleObject)]
pub struct PaginatedResponse<T: OutputType> {
    pub entities: Vec<T>,
    pub total_entities: u32,
    pub page_index: u32,
    pub page_size: u32,
    pub total_pages: u32,
}
