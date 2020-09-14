use structopt::StructOpt;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

mod model;
use model::assignment::*;
use model::chore::*;
use model::room::*;
use model::user::*;
mod schema;
#[macro_use]
extern crate diesel;

#[derive(StructOpt, Debug)]
#[structopt(about = "Nags you to do all your chores")]
enum Command {
  #[structopt(about = "Show all")]
  List {
    #[structopt(subcommand)]
    noun: NounPlural,
  },
  #[structopt(about = "Create")]
  Create {
    #[structopt(subcommand)]
    noun: NounSingular,
  },
  #[structopt(about = "Delete by name")]
  Remove {
    #[structopt(subcommand)]
    noun: NounSingular,
  },
  #[structopt(about = "Update by name")]
  Update {
    #[structopt(subcommand)]
    noun: NounSingular,
  },
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Noun")]
enum NounPlural {
  #[structopt(about = "Chores")]
  Chores {
    name: Option<String>,
    description: Option<String>,
  },
  #[structopt(about = "Rooms")]
  Rooms {
    name: Option<String>,
    description: Option<String>,
  },
  #[structopt(about = "Users")]
  Users {
    name: Option<String>,
    description: Option<String>,
  },
  #[structopt(about = "Assignments")]
  Assignments {
    chore_name: Option<String>,
    room_name: Option<String>,
    user_name: Option<String>,
  },
}

#[derive(StructOpt, Debug)]
#[structopt(about = "Noun")]
enum NounSingular {
  #[structopt(about = "Chore")]
  Chore {
    name: String,
    description: Option<String>,
  },
  #[structopt(about = "Room")]
  Room {
    name: String,
    description: Option<String>,
  },
  #[structopt(about = "User")]
  User {
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

fn main() {
  let opt = Command::from_args();

  let connection = establish_connection();

  match opt {
    Command::List {
      noun: NounPlural::Chores { .. },
    } => {
      for chore in get_chores(&connection) {
        println!("{}: {:?}", chore.name, chore.description);
      }
    }
    Command::List {
      noun: NounPlural::Rooms { .. },
    } => {
      for room in get_rooms(&connection) {
        println!("{}: {:?}", room.name, room.description);
      }
    }
    Command::List {
      noun: NounPlural::Users { .. },
    } => {
      for user in get_users(&connection) {
        println!("{}: {:?}", user.name, user.description);
      }
    }
    Command::List {
      noun: NounPlural::Assignments { .. },
    } => {
      for assignment in get_assignments(&connection) {
        println!(
          "{}, {}, {:?}",
          assignment.chore_id, assignment.room_id, assignment.user_id
        );
      }
    }
    Command::Create {
      noun: NounSingular::Chore { name, description },
    } => {
      create_chore(&connection, name, description);
    }
    Command::Create {
      noun: NounSingular::Room { name, description },
    } => {
      create_room(&connection, name, description);
    }
    Command::Create {
      noun: NounSingular::User { name, description },
    } => {
      create_user(&connection, name, description);
    }
    Command::Remove {
      noun: NounSingular::Chore { name, .. },
    } => {
      delete_chore(&connection, name);
    }
    Command::Remove {
      noun: NounSingular::Room { name, .. },
    } => {
      delete_room(&connection, name);
    }
    Command::Remove {
      noun: NounSingular::User { name, .. },
    } => {
      delete_user(&connection, name);
    }
    Command::Update {
      noun: NounSingular::Chore { name, description },
    } => {
      update_chore(&connection, name, description);
    }
    Command::Update {
      noun: NounSingular::Room { name, description },
    } => {
      update_room(&connection, name, description);
    }
    Command::Update {
      noun: NounSingular::User { name, description },
    } => {
      update_user(&connection, name, description);
    }
  }
}
