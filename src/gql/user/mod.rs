use super::Context;
use juniper;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/gql/user/user_schema.gql");

pub struct Query;
pub struct Mutation;

impl MutationFields for Mutation {
  fn field_noop(&self, _executor: &Executor<'_, Context>) -> FieldResult<&bool> {
      Ok(&true)
  }
}

impl QueryFields for Query {
  fn field_current_user(
    &self,
    _executor: &juniper::Executor<'_, Context>,
  ) -> juniper::FieldResult<String> {
    let user = User {
      name: String::from("Ferris"),
    };

    Ok(format!("The current user is {}.", user.name))
  }
}

#[cfg(test)]
mod user_test;

pub type Schema = juniper::RootNode<'static, Query, Mutation>;