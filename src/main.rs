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

macro_rules! list_noun {
  ($noun:tt, $connection:expr) => {
    $noun::table
      .limit(5)
      .load::<Chore>($connection)
      .expect("Error loading $noun")
  };
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
  let opt = Command::from_args();

  let connection = establish_connection();

  match opt {
    Command::List { noun } => {
      let nouns = match noun {
        Noun::Chores => list_noun!(chores, &connection),
        Noun::Rooms => list_noun!(rooms, &connection),
      };
      println!("From database: ");
      for noun in nouns {
        println!("\t{}: {:?}", noun.name, noun.description);
      }
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
