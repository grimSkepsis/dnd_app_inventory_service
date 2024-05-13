use async_graphql::Object;
use async_graphql::ID;

#[derive(Clone)]
pub struct User {
    pub id: ID,
    pub name: String,
    pub email: String,
}

#[Object]
impl User {
    // pub fn new(id: ID, name: String, email: String) -> Self {
    //     User { id, name, email }
    // }

    async fn id(&self) -> &str {
        &self.id
    }

    async fn name(&self) -> &str {
        &self.name
    }

    async fn email(&self) -> &str {
        &self.email
    }
}
