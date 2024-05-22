use async_graphql::Object;
use async_graphql::ID;

#[derive(Clone)]
pub struct Inventory {
    pub uuid: ID,
    pub name: String,
    pub capacity: u16,
    pub cp: u32,
    pub sp: u32,
    pub gp: u32,
    pub pp: u32,
}

#[Object]
impl Inventory {
    async fn uuid(&self) -> &str {
        &self.uuid
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn capacity(&self) -> u16 {
        self.capacity
    }

    async fn cp(&self) -> u32 {
        self.cp
    }

    async fn sp(&self) -> u32 {
        self.sp
    }

    async fn gp(&self) -> u32 {
        self.gp
    }
}
