use std::env;

use dotenv::dotenv;

pub struct DatabaseConfig{
    pub db_url: String
}

impl DatabaseConfig {
    pub fn inject_from_env() -> Self {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("Fail to retrive DATABASE_URL from env file, please reive");
        Self {
            db_url
        }
    }
}