use std::env;
mod other;

fn main() {
  let args: Vec<String> = env::args().collect();
  println!("{:?}", args);

  other::help();
}
