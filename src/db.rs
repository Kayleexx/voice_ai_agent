use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use crate::models::*;
use crate::schema::logs;

pub type DbPool = Pool<ConnectionManager<SqliteConnection>>;

pub fn establish_connection() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool")
}

pub fn insert_log(pool: &DbPool, user_input: &str, bot_response: &str) {
    let conn = &mut pool.get().unwrap();
    let new_log = NewLog {
        user_input,
        bot_response,
    };

    diesel::insert_into(logs::table)
        .values(&new_log)
        .execute(conn)
        .expect("Failed to insert log");
}
