use super::{maria::MariaDbConfig, mysql::MysqlConfig, postgrest::PostgresConfig, sqlite::SqliteConfig, trait_get_uri::DbLoadConfigTrait};

pub enum DbConfig {
    MYSQL,
    MARIADB,
    SQLITE,
    POSTGRES
}

pub struct DatabaseFactory{}

impl DatabaseFactory {
    pub fn get_database_config(db_type: DbConfig) -> Box<dyn DbLoadConfigTrait>  {
        match db_type {
            DbConfig::MYSQL => Box::new(MysqlConfig::inject_env()),
            DbConfig::MARIADB => Box::new(MariaDbConfig::inject_env()),
            DbConfig::POSTGRES => Box::new(PostgresConfig::inject_env()),
            DbConfig::SQLITE => Box::new(SqliteConfig::inject_env()),
        }
    }
}
