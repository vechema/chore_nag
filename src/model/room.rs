use crate::schema::rooms;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

#[derive(Queryable, Identifiable)]
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

pub fn create_room(
  connection: &SqliteConnection,
  name: String,
  description: Option<String>,
) -> usize {
  let new_room = NewRoom { name, description };

  diesel::insert_into(rooms::table)
    .values(&new_room)
    .execute(connection)
    .expect("Error saving new room")
}

pub fn get_rooms(connection: &SqliteConnection) -> Vec<Room> {
  rooms::table
    .limit(5)
    .load::<Room>(connection)
    .expect("Error loading rooms")
}

pub fn delete_room(connection: &SqliteConnection, name: String) -> () {
  diesel::delete(rooms::table.filter(rooms::name.eq(name)))
    .execute(connection)
    .expect("Error deleting room");
}

pub fn update_room(connection: &SqliteConnection, name: String, description: Option<String>) -> () {
  diesel::update(rooms::table.filter(rooms::name.eq(name)))
    .set(rooms::description.eq(description))
    .execute(connection)
    .expect("Error updating room");
}
