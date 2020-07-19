use structopt::StructOpt;
mod other;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod models;
mod schema;
#[macro_use]
extern crate diesel;
use diesel::prelude::*;
use models::Chore;
use schema::chores;

#[derive(StructOpt, Debug)]
#[structopt(about = "the stupid content tracker")]
enum ChoreNag {
  #[structopt(about = "Show all chores")]
  List {},
}

pub fn establish_connection() -> SqliteConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  SqliteConnection::establish(&database_url)
    .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
  let opt = ChoreNag::from_args();

  let chores_list = vec!["clean sink", "clean up after dog"];

  match opt {
    ChoreNag::List {} => println!("From command line: {:?}", chores_list),
  }

  let connection = establish_connection();

  let results = chores::table
    .limit(5)
    .load::<Chore>(&connection)
    .expect("Error loading posts");

  println!("From database: ");
  for chore in results {
    println!("\t{}: {:?}", chore.name, chore.description);
  }
}
