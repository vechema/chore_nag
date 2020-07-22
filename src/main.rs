use structopt::StructOpt;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;
#[macro_use]
extern crate diesel;
use models::{Chore, NewChore};
use schema::chores;

#[derive(StructOpt, Debug)]
#[structopt(about = "the stupid content tracker")]
enum ChoreNag {
  #[structopt(about = "Show all chores")]
  List {},
  #[structopt(about = "Add a chore")]
  Add {
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

  diesel::insert_into(schema::chores::table)
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
    .expect("Errer updating chore");
}

fn main() {
  let opt = ChoreNag::from_args();

  let connection = establish_connection();

  match opt {
    ChoreNag::List {} => {
      let chores = get_chores(&connection);
      println!("From database: ");
      for chore in chores {
        println!("\t{}: {:?}", chore.name, chore.description);
      }
    }
    ChoreNag::Add { name, description } => {
      create_chore(&connection, name, description);
    }
    ChoreNag::Remove { name } => {
      delete_chore(&connection, name);
    }
    ChoreNag::Update { name, description } => {
      update_chore(&connection, name, description);
    }
  }
}
