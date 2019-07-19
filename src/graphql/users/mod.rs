use super::Context;
use juniper;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/graphql/users/schema.gql");

pub struct Query;

pub struct User {
  name: String,
}

impl QueryFields for Query {
  fn field_current_user(
    &self,
    executor: &juniper::Executor<'_, Context>,
  ) -> juniper::FieldResult<String> {
    let user = User {
      name: String::from("Ferris"),
    };

    Ok(format!("The current user is {}.", user.name))
  }
}
