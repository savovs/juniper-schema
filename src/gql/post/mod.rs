use super::Context;
use juniper;
use juniper_from_schema::graphql_schema_from_file;

graphql_schema_from_file!("src/gql/post/post_schema.gql");

pub struct Query;



impl QueryFields for Query {
  fn field_latest_posts(
    &self,
    _executor: &juniper::Executor<'_, Context>,
  ) -> Vec<Post> {
    // let post = Post {
    //   id: String::from("1"),
    //   title: String::from("Hello"),
    //   content: String::from("World"),
    // };

    Ok(vec![])
  }
}

#[cfg(test)]
mod post_test;


pub type Schema = RootNode<'static, Query, juniper::EmptyMutation>;