use diesel::prelude::*;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub telegram_id: i32,
    pub step_id: i16
}