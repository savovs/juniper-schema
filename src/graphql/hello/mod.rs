use juniper;
use juniper_from_schema::graphql_schema_from_file;
use super::Context;

graphql_schema_from_file!("src/graphql/hello/schema.gql");

pub struct Query;

impl QueryFields for Query {
  fn field_hello(
    &self,
    executor: &juniper::Executor<'_, Context>,
    name: String,
  ) -> juniper::FieldResult<String> {
    Ok(format!("Hello, {}!", name))
  }
}
