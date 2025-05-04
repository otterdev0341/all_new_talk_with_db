use std::env;

use anyhow::Error;
use ollama_rs::Ollama;
use sqlx::mysql::{MySqlPool, MySqlRow};
use async_trait::async_trait;
use crate::trait_req_impl::chain::Chain;
use dotenv::dotenv;
use sqlx::Row;

pub struct TextToSqlChain{
    pub client: Ollama,
    pub db: MySqlPool
}


#[async_trait]
impl Chain for TextToSqlChain {
    async fn initialze() -> Result<Box<dyn Chain + Send>, Error>
        where
            Self: Sized
    {
        let _ = dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("Failed to load Database url, please review .env");
        let ollama_host = env::var("OLAMA_URL").expect("can't connect to ollama, please review .env file");
        let ollama_port = env::var("OLAMA_PORT").expect("Failed to retrive port information, please review .env");
        let pool = MySqlPool::connect(&db_url).await?;
        let ollama = Ollama::new(&ollama_host, ollama_port.parse().unwrap());
        Ok(
            Box::new(TextToSqlChain {
                client: ollama,
                db: pool
            })
        )
    }


    async fn run(&self, input: String) -> Result<String, Error>{
        let result = self.get_db_info(&self.db).await.unwrap();
        Ok(result)
    }
}


impl TextToSqlChain {
    pub async fn get_db_info(&self, pool: &MySqlPool) -> Result<String, Error> {
        let mut schema_description = String::new();
    
        // Step 1: Get current database name
        let current_db_row = sqlx::query("SELECT DATABASE() AS db")
            .fetch_one(pool)
            .await?;
    
        let current_db: String = current_db_row.try_get("db")?;
    
        // Step 2: Get all table names from current database
        let tables = sqlx::query(
            "SELECT TABLE_NAME FROM INFORMATION_SCHEMA.TABLES WHERE TABLE_SCHEMA = ?"
        )
        .bind(&current_db)
        .fetch_all(pool)
        .await?;
    
        for table in tables {
            let table_name: String = table.try_get("TABLE_NAME")?;
            schema_description.push_str(&format!("Table \"{}\":\n", table_name));
    
            // Step 3: Get all columns for this table (name and type)
            let columns = sqlx::query(
                "SELECT COLUMN_NAME, DATA_TYPE FROM INFORMATION_SCHEMA.COLUMNS \
                 WHERE TABLE_SCHEMA = ? AND TABLE_NAME = ?"
            )
            .bind(&current_db)
            .bind(&table_name)
            .fetch_all(pool)
            .await?;
    
            for col in columns {
                let column_name: String = col.try_get("COLUMN_NAME")?;
                let column_type: String = col.try_get("DATA_TYPE")?;
                schema_description.push_str(&format!("- {} ({})\n", column_name, column_type));
            }
    
            schema_description.push('\n');
        }
    
        Ok(schema_description)
    }
}


