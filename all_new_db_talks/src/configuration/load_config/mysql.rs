use dotenv::dotenv;
use super::trait_get_uri::DbLoadConfigTrait;

pub struct MysqlConfig {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub db_name: String,
    pub ip_address: String,
}

impl MysqlConfig {
    pub fn inject_env() -> Self {
        dotenv().ok();
        let retrive_username = std::env::var("MYSQL_USERNAME")
            .expect("MYSQL_USERNAME in env not found!, please review");
        let retrive_password = std::env::var("MYSQL_PASSWORD")
            .expect("MYSQL_PASSWORD in env not found!, please review");
        let retrive_port: u16 = std::env::var("MYSQL_PORT")
            .expect("MYSQL_PORT in env not found!, please review")
            .parse()
            .expect("MYSQL_PORT must be a valid u16 number");
        let retrive_db_name = std::env::var("MYSQL_DB_NAME")
            .expect("MYSQL_DB_NAME in env not found!, please review");
        let retrive_id_address = std::env::var("MYSQL_IP_ADDRESS")
            .expect("MYSQL_IP_ADDRESS in env not found!, please review");

        Self { 
            username: retrive_username, 
            password: retrive_password, 
            port: retrive_port, 
            db_name: retrive_db_name, 
            ip_address: retrive_id_address }
    }

    
}

impl DbLoadConfigTrait for MysqlConfig {
    fn get_url(&self) -> String {
        format!(
            "mysql://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.ip_address,
            self.port,
            self.db_name
        )
    }
}