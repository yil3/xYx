pub mod service;
pub mod repository;
pub mod controller;
pub mod application;


use std::env;

use lazy_static::lazy_static;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

lazy_static! {
    static ref POOL: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&env::var("DATABASE_URL").unwrap())
        .expect("could not initialize the database connection pool");
}
