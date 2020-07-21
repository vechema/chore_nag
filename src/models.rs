use super::schema::chores;

#[derive(Queryable)]
pub struct Chore {
  pub id: i32,
  pub name: String,
  pub description: Option<String>,
}

#[derive(Insertable)]
#[table_name = "chores"]
pub struct NewChore {
  pub name: String,
  pub description: Option<String>,
}
