use juniper::{GraphQLObject};

#[derive(GraphQLObject)]
#[graphql(description = "Rooms Object that object stores the room information")]
pub struct Rooms {
    id: i32,
    name: String,
    description: String,
    number: String,
    password: String,
    owner: i32,
    created_at: String,
    updated_at: String,
}


pub fn get_room() -> Rooms {
    Rooms {
        id: 1,
        name: "test".to_string(),
        description: "test".to_string(),
        number: "test".to_string(),
        password: "test".to_string(),
        owner: 1,
        created_at: "test".to_string(),
        updated_at: "test".to_string(),
    }
}