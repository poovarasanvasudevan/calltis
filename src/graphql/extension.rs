use juniper::{GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "Stores extension information")]
pub struct Extension {
    pub id: i32,
    pub name: String,
    pub extension: String,
    pub description: String,
    pub is_active: bool,
    pub created_at: String,
    pub updated_at: String,
}

