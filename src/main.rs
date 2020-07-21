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
  Add {
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

pub fn create_chore(conn: &SqliteConnection, name: String, description: Option<String>) -> usize {
  let new_chore = NewChore { name, description };

  diesel::insert_into(schema::chores::table)
    .values(&new_chore)
    .execute(conn)
    .expect("Error saving new chore")
}

fn main() {
  let opt = ChoreNag::from_args();

  let connection = establish_connection();

  match opt {
    ChoreNag::List {} => {
      let results = chores::table
        .limit(5)
        .load::<Chore>(&connection)
        .expect("Error loading chores");

      println!("From database: ");
      for chore in results {
        println!("\t{}: {:?}", chore.name, chore.description);
      }
    }
    ChoreNag::Add { name, description } => {
      create_chore(&connection, name, description);
    }
  }
}
