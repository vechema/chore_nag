use structopt::StructOpt;
mod other;

#[derive(StructOpt, Debug)]
#[structopt(about = "the stupid content tracker")]
enum ChoreNag {
  #[structopt(about = "Show all chores")]
  List {},
}

fn main() {
  let opt = ChoreNag::from_args();

  let chores = vec!["clean sink", "clean up after dog"];

  match opt {
    ChoreNag::List {} => println!("{:?}", chores),
  }
}
