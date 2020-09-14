use crate::schema::users;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Identifiable)]
pub struct User {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
  pub name: String,
  pub description: Option<String>,
}

pub fn create_user(
  connection: &SqliteConnection,
  name: String,
  description: Option<String>,
) -> usize {
  let new_user = NewUser { name, description };

  diesel::insert_into(users::table)
    .values(&new_user)
    .execute(connection)
    .expect("Error saving new user")
}

pub fn get_users(connection: &SqliteConnection) -> Vec<User> {
  users::table
    .limit(5)
    .load::<User>(connection)
    .expect("Error loading users")
}

pub fn delete_user(connection: &SqliteConnection, name: String) -> () {
  diesel::delete(users::table.filter(users::name.eq(name)))
    .execute(connection)
    .expect("Error deleting user");
}

pub fn update_user(connection: &SqliteConnection, name: String, description: Option<String>) -> () {
  diesel::update(users::table.filter(users::name.eq(name)))
    .set(users::description.eq(description))
    .execute(connection)
    .expect("Error updating user");
}
