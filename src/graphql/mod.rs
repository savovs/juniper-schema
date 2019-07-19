use juniper;

mod hello;
mod users;

pub struct Context;
impl juniper::Context for Context {}

pub struct Query;

#[juniper::object(
    Context = Context,
)]
impl Query {
    // ..hello::Query
    // ..users::Query
    // ^ How do I put the contents of these here at compile time?
    // The idea is to have a nice folder structure separated by concern.
}

pub struct Mutation;

#[juniper::object(
    Context = Context,
)]
impl Mutation {}

pub type Schema = juniper::RootNode<'static, Query, Mutation>;
