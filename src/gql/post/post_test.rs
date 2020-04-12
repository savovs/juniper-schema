#[test]
fn current_user() {
  use super::{Query, Post, Mutation, Schema};

  let context = crate::gql::Context;
  let query = "query { latestPosts }";
  
  let (result, errors) = juniper::execute(
    query,
    None,
    &Schema::new(Query, juniper::EmptyMutation),
    &juniper::Variables::new(),
    &context,
  )
  .unwrap();

  assert_eq!(errors.len(), 0);

  let result = result.as_object_value().unwrap();
  let post = Post {
    id: String::from("1"),
    title: String::from("Hello"),
    content: String::from("World"),
  };

  assert_eq!(result.get_field_value("latestPosts").unwrap(), post);
}