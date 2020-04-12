#[test]
fn current_user() {
  use super::{Query, Mutation, Schema};

  let context = crate::gql::Context;
  let query = "query { currentUser }";
  let (result, errors) = juniper::execute(
    query,
    None,
    &Schema::new(Query, juniper::EmptyMutation),
    &juniper::Variables::new(),
    &context,
  )
  .unwrap();

  assert_eq!(errors.len(), 0);

  let result_object = result.as_object_value().unwrap();

  assert_eq!(
    result_object
      .get_field_value("currentUser")
      .unwrap()
      .as_scalar_value::<String>()
      .unwrap(),
    "The current user is Ferris.",
  );
}