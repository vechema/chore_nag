use clap::{load_yaml, App};
use std::env;
mod other;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  other::help();

  let yaml = load_yaml!("cli.yml");
  let matches = App::from_yaml(yaml).get_matches();

  let chores = vec!["clean sink", "clean up after dog"];
}
