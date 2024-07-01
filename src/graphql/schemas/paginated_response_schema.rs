use async_graphql::{Object, OutputType};

#[derive(Debug, Clone)]
pub struct PaginatedResponse<T: OutputType + Sync> {
    pub entities: Vec<T>,
    pub total_entities: u32,
    pub page_index: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[Object]
impl<T: OutputType + Sync> PaginatedResponse<T> {
    async fn entities(&self) -> &Vec<T> {
        &self.entities
    }

    async fn total_entities(&self) -> u32 {
        self.total_entities
    }

    async fn page_index(&self) -> u32 {
        self.page_index
    }

    async fn page_size(&self) -> u32 {
        self.page_size
    }

    async fn total_pages(&self) -> u32 {
        self.total_pages
    }
}
