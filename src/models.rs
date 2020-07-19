#[derive(Queryable)]
pub struct Chore {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
}
