#[macro_use]
extern crate diesel;

#[macro_use]
extern crate serde_derive;

use actix_web::{middleware, App, HttpServer};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

mod errors;
mod models;
mod routes;
mod schema;

pub struct Blog {
  port: u16,
}
// NOTE: Bookmark on Page 112
impl Blog {
  pub fn new(port: u16) -> Self {
    Blog { port }
  }

  pub fn run(&self, database_url: String) -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = Pool::builder()
      .build(manager)
      .expect("Failed to create pool");

    println!("Listening on http://localhost:{}", self.port);
    HttpServer::new(move || {
      App::new()
        .data(pool.clone())
        .wrap(middleware::Logger::default())
        .configure(routes::users::configure)
    })
    .bind(("localhost", self.port))?
    .run()
  }
}
