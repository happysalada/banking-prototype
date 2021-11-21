use async_graphql::SimpleObject;
use sqlx::FromRow;
use std::default::Default;

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub inserted_at: String,
}

#[derive(Clone, SimpleObject, Debug, Default, FromRow)]
pub struct Transaction {
    pub id: String,
    pub from_id: String,
    pub to_id: String,
    pub amount: i32,
    pub note: String,
    pub inserted_at: String,
}
