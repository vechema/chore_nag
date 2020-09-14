use crate::schema::assignments;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Identifiable)]
pub struct Assignment {
  pub id: i32,
  pub chore_id: i32,
  pub room_id: i32,
  pub user_id: Option<i32>,
}

#[derive(Insertable)]
#[table_name = "assignments"]
pub struct NewAssignment {
  pub chore_id: i32,
  pub room_id: i32,
  pub user_id: Option<i32>,
}

pub fn get_assignments(connection: &SqliteConnection) -> Vec<Assignment> {
  assignments::table
    .limit(5)
    .load::<Assignment>(connection)
    .expect("Error loading chores")
}
