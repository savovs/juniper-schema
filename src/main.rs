mod graphql;

fn main() {
    use graphql::{Context, Mutation, Query, Schema};

    let context = Context;
    let query = "query { hello(name: \"friend\") currentUser }";
    let (result, errors) = juniper::execute(
        query,
        None,
        &Schema::new(Query, Mutation),
        &juniper::Variables::new(),
        &context,
    )
    .unwrap();

    assert_eq!(errors.len(), 0);

    let result_object = result.as_object_value().unwrap();

    assert_eq!(
        result_object
            .get_field_value("hello")
            .unwrap()
            .as_scalar_value::<String>()
            .unwrap(),
        "Hello, friend!",
    );

    assert_eq!(
        result_object
            .get_field_value("currentUser")
            .unwrap()
            .as_scalar_value::<String>()
            .unwrap(),
        "The current user is Ferris.",
    );
}
