use std::env;
use dotenv::dotenv;

use super::trait_get_uri::DbLoadConfigTrait;

pub struct PostgresConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

impl PostgresConfig {
    pub fn inject_env() -> Self {
        dotenv().ok();

        let username = env::var("POSTGRES_USERNAME")
            .expect("POSTGRES_USERNAME in env not found!, please review");

        let password = env::var("POSTGRES_PASSWORD")
            .expect("POSTGRES_PASSWORD in env not found!, please review");

        let host = env::var("POSTGRES_HOST")
            .expect("POSTGRES_HOST in env not found!, please review");

        let port: u16 = env::var("POSTGRES_PORT")
            .expect("POSTGRES_PORT in env not found!, please review")
            .parse()
            .expect("POSTGRES_PORT must be a valid u16 number");

        let db_name = env::var("POSTGRES_DB_NAME")
            .expect("POSTGRES_DB_NAME in env not found!, please review");

        Self {
            username,
            password,
            host,
            port,
            db_name,
        }
    }

   
}

impl DbLoadConfigTrait for PostgresConfig {
    fn get_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}