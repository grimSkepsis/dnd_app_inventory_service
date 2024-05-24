use async_graphql::{Object, OutputType};

#[derive(Debug, Clone)]
pub struct PaginatedResponse<T: OutputType + Sync> {
    pub entities: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
}

#[Object]
impl<T: OutputType + Sync> PaginatedResponse<T> {
    async fn entities(&self) -> &Vec<T> {
        &self.entities
    }

    async fn total(&self) -> u32 {
        self.total
    }

    async fn page(&self) -> u32 {
        self.page
    }

    async fn page_size(&self) -> u32 {
        self.page_size
    }

    async fn total_pages(&self) -> u32 {
        self.total_pages
    }
}
