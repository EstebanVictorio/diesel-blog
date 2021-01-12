use crate::errors::AppError;
use crate::schema::*;
use diesel::prelude::*;

type Result<T> = std::result::Result<T, AppError>;

#[derive(Queryable, Identifiable, Serialize, Debug, PartialEq)]
pub struct User {
  pub id: i32,
  pub username: String,
}
// NOTE: Add bookmark for last page: 124
pub fn create_user(conn: &SqliteConnection, username: &str) -> Result<User> {
  conn.transaction(|| {
    diesel::insert_into(users::table)
      .values((users::username.eq(username)))
      .execute(conn)?;

    users::table
      .order(users::id.desc())
      .select((users::id, users::username))
      .first(conn)
      .map_err(Into::into)
  })
}

enum UserKey<'a> {
  Username(&'a str), // NOTE: added lifetime, the Username value's lifetime lives as long as the struct's lifetime
  ID(i32),
}
