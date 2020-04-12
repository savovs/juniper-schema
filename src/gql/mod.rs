use juniper;


pub mod post;
pub mod user;

/// merge each gql module with a macro
/// let root_schema = merge_schemas![post, user];





pub struct Context;
pub struct Mutation;

impl juniper::Context for Context {}
