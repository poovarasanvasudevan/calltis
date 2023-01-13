use juniper::{EmptySubscription, FieldResult, RootNode};
use crate::dep::extension::{Extension};
use crate::dep::graphql_schema::rooms::Rooms;

pub mod rooms;
pub mod system_performance;

pub struct QueryRoot;
pub struct MutationRoot;

#[juniper::graphql_object]
impl QueryRoot {
    fn rooms(_id: String) -> FieldResult<Rooms> {
        Ok(rooms::get_room())
    }
    fn system_performance() -> FieldResult<system_performance::SystemInfo> {
        Ok(system_performance::get_system_info())
    }
}

#[juniper::graphql_object]
impl MutationRoot {
    fn create_room(_id: String) -> FieldResult<Rooms> {
        Ok(rooms::get_room())
    }
}


pub type Schema = RootNode<'static, QueryRoot, MutationRoot,EmptySubscription>;

pub fn create_schema() -> Schema {
    Schema::new(
        QueryRoot {},
        MutationRoot {},
        EmptySubscription::new()
    )
}