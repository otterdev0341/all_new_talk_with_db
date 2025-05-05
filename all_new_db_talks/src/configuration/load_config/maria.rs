use std::env;
use dotenv::dotenv;

use super::trait_get_uri::DbLoadConfigTrait;

pub struct MariaDbConfig {
    pub username: String,
    pub password: String,
    pub host: String,
    pub port: u16,
    pub db_name: String,
}

impl MariaDbConfig {
    pub fn inject_env() -> Self {
        dotenv().ok();

        let username = env::var("MARIADB_USERNAME")
            .expect("MARIADB_USERNAME in env not found!, please review");

        let password = env::var("MARIADB_PASSWORD")
            .expect("MARIADB_PASSWORD in env not found!, please review");

        let host = env::var("MARIADB_HOST")
            .expect("MARIADB_HOST in env not found!, please review");

        let port: u16 = env::var("MARIADB_PORT")
            .expect("MARIADB_PORT in env not found!, please review")
            .parse()
            .expect("MARIADB_PORT must be a valid u16 number");

        let db_name = env::var("MARIADB_DB_NAME")
            .expect("MARIADB_DB_NAME in env not found!, please review");

        Self {
            username,
            password,
            host,
            port,
            db_name,
        }
    }

    
}


impl DbLoadConfigTrait for MariaDbConfig {
    fn get_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username, self.password, self.host, self.port, self.db_name
        )
    }
}