use lazy_static::lazy_static;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

lazy_static! {
    pub static ref EXPONENTIAL_SECONDS: &'static [f64] = &[0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,];
    pub static ref POOL: Pool<Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect_lazy(&std::env::var("DATABASE_URL").unwrap())
        .expect("could not initialize the database connection pool");
}
