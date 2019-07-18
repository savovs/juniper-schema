use juniper;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/graphql/users/schema.gql");

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;
impl QueryFields for Query {
  fn field_hello_world(
    &self,
    executor: &juniper::Executor<'_, Context>,
    name: String,
  ) -> juniper::FieldResult<String> {
    Ok(format!("Hello, {}!", name))
  }
}

pub struct Mutation;
impl MutationFields for Mutation {
  fn field_noop(&self, executor: &juniper::Executor<'_, Context>) -> juniper::FieldResult<&bool> {
    Ok(&true)
  }
}

pub fn hello() {
  println!("hello")
}

