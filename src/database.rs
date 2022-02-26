use std::env;

use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::Pool;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn database_connection() -> DbPool {
    let url = env::var("DATABASE_URL").expect("DATABASE_URL env is not set.");
    let migration = ConnectionManager::<PgConnection>::new(url);
    r2d2::Pool::builder()
        .build(migration)
        .expect("Error during connection pool build")
}
