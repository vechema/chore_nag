use super::schema::{chores, rooms};

#[derive(Queryable)]
pub struct Chore {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "chores"]
pub struct NewChore {
  pub name: String,
  pub description: Option<String>,
}

#[derive(Queryable)]
pub struct Room {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "rooms"]
pub struct NewRoom {
  pub name: String,
  pub description: Option<String>,
}
