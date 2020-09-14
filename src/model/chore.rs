use crate::schema::chores;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Identifiable)]
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

pub fn create_chore(
  connection: &SqliteConnection,
  name: String,
  description: Option<String>,
) -> usize {
  let new_chore = NewChore { name, description };

  diesel::insert_into(chores::table)
    .values(&new_chore)
    .execute(connection)
    .expect("Error saving new chore")
}

pub fn get_chores(connection: &SqliteConnection) -> Vec<Chore> {
  chores::table
    .limit(5)
    .load::<Chore>(connection)
    .expect("Error loading chores")
}

pub fn delete_chore(connection: &SqliteConnection, name: String) -> () {
  diesel::delete(chores::table.filter(chores::name.eq(name)))
    .execute(connection)
    .expect("Error deleting chore");
}

pub fn update_chore(
  connection: &SqliteConnection,
  name: String,
  description: Option<String>,
) -> () {
  diesel::update(chores::table.filter(chores::name.eq(name)))
    .set(chores::description.eq(description))
    .execute(connection)
    .expect("Error updating chore");
}
