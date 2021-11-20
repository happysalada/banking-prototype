use async_graphql::SimpleObject;
use sqlx::FromRow;
use std::default::Default;

#[derive(Clone, SimpleObject, Default, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: Option<String>,
    pub insterted_at: i32,
}
