use async_graphql::Object;

#[derive(Debug, Clone)]
pub struct Trait {
    pub name: String,
    pub description: Option<String>,
}

#[Object]
impl Trait {
    async fn name(&self) -> &String {
        &self.name
    }

    async fn description(&self) -> Option<&String> {
        self.description.as_ref()
    }
}
