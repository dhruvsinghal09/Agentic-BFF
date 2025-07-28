use rusqlite::Connection;
use std::{
    collections::HashSet,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct AppState {
    pub api_tokens: Arc<Mutex<HashSet<String>>>,
    pub db_conn: Arc<Mutex<Connection>>,
    pub jwt_secret: Arc<String>,
}

impl AppState {
    pub fn new() -> Self {
        // In-memory SQLite database setup
        let conn = Connection::open("users.db").unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                username TEXT PRIMARY KEY,
                password_hash TEXT NOT NULL
            )",
            [],
        )
        .unwrap();

        AppState {
            api_tokens: Arc::new(Mutex::new(HashSet::new())),
            db_conn: Arc::new(Mutex::new(conn)),
            jwt_secret: Arc::new("super_secret_jwt_key".into()),
        }
    }
}
