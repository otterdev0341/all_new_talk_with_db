use std::env;
use dotenv::dotenv;

use super::trait_get_uri::DbLoadConfigTrait;

pub struct SqliteConfig {
    pub db_dir: String,
    pub db_name: String,
}

impl SqliteConfig {
    pub fn inject_env() -> Self {
        dotenv().ok();
        let db_dir = env::var("SQLITE_DB_DIR")
            .expect("SQLITE_DB_DIR in env not found!, please review");

        let db_name = env::var("SQLITE_DB_NAME")
            .expect("SQLITE_DB_NAME in env not found!, please review");

        Self { db_dir, db_name }
    }

    
}

impl DbLoadConfigTrait for SqliteConfig {
    fn get_url(&self) -> String {
        format!("sqlite://{}/{}", self.db_dir.trim_end_matches('/'), self.db_name)
    }
}
