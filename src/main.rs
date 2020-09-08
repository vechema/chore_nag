use structopt::StructOpt;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;
#[macro_use]
extern crate diesel;
use models::{Chore, NewChore, Room};
use schema::{chores, rooms};

#[derive(StructOpt, Debug)]
#[structopt(about = "Nags you to do all your chores")]
enum Command {
  #[structopt(about = "Show all")]
  List {
    #[structopt(subcommand)]
    noun: Noun,
  },
  #[structopt(about = "Create a chore")]
  Create {
    name: String,
    description: Option<String>,
  },
  #[structopt(about = "Delete a chore by name")]
  Remove { name: String },
  #[structopt(about = "Update a chore by name")]
  Update {
    name: String,
    description: Option<String>,
  },
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Noun")]
enum Noun {
  #[structopt(about = "CHORES")]
  Chores,
  #[structopt(about = "Rooms")]
  Rooms,
}

pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
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

pub fn get_rooms(connection: &SqliteConnection) -> Vec<Room> {
  rooms::table
    .limit(5)
    .load::<Room>(connection)
    .expect("Error loading rooms")
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

fn main() {
  let opt = Command::from_args();

  let connection = establish_connection();

  match opt {
    Command::List { noun } => {
      println!("From database: ");
      match noun {
        Noun::Chores => {
          for chore in get_chores(&connection) {
            println!("\t{}: {:?}", chore.name, chore.description);
          }
        }
        Noun::Rooms => {
          for room in get_rooms(&connection) {
            println!("\t{}: {:?}", room.name, room.description);
          }
        }
      };
    }
    Command::Create { name, description } => {
      create_chore(&connection, name, description);
    }
    Command::Remove { name } => {
      delete_chore(&connection, name);
    }
    Command::Update { name, description } => {
      update_chore(&connection, name, description);
    }
  }
}
