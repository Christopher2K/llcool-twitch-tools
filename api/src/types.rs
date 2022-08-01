use diesel::{r2d2, sqlite::SqliteConnection};

pub type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqliteConnection>>;
