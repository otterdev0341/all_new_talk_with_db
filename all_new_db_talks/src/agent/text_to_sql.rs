use std::env;

use anyhow::Error;
use crate::{configuration::model_config::ModelSelect, datasource::async_db_utill::AsyncDb};
use ollama_rs::{generation::completion::request::GenerationRequest, Ollama};
use sqlx::mysql::MySqlPool;
use async_trait::async_trait;
use crate::{datasource::db_utill::{DatabaseSchema, DbUtil}, trait_req_impl::chain::Chain};
use dotenv::dotenv;
use anyhow::anyhow;
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
        let prompt = self.construct_prompt(input).await?;
        let request = GenerationRequest::new(
            ModelSelect::SqlOperate.as_str(),
            prompt
        );
        let sql = self.client.generate(request).await.unwrap();
        println!("SQL is {:?}", sql.response);
        let clean_query = sql.response
        .replace("```sql", "")
        .replace("```", "")
        .trim()
        .to_string();
        
        let asy_db = AsyncDb::new().unwrap();
        let query_data = asy_db.query_as_string(clean_query).await.unwrap();
        Ok(query_data)
    }
}


impl TextToSqlChain {
    pub async fn get_db_info(&self) -> Result<DatabaseSchema, Error> {
        let tool = match DbUtil::new() {
            Ok(mut data) => {
                let result = data.get_database_schema();
                Ok(result)
            },
            Err(_) => Err(anyhow!("fail to get database schema")),
        };

        tool
    }

    pub async fn construct_prompt(&self, input:String) -> Result<String, Error> {
        let mut db = match DbUtil::new() {
            Ok(tool) => tool,
            Err(_) => return Err(anyhow!("Failed to retive database schema"))
        };
        let db_schema = db.get_database_schema();

        let prompt = format!(
            "You are a database expert.
    
            Database Schema:
            {}
            
            Instructions:
            - Generate ONE correct SQL query for SQLite that answers the given user question.
            - Only output the SQL command.
            - No explanations, no examples, no prefixes (such as 'Example:', 'SQL:', 'Response:', 'Result:').
            - No formatting like markdown (no ```sql blocks).
            - Output ONLY the SQL query — no extra text.
            
            User Question:
            {}
            
            Remember: ONLY output a single valid SQL command.",
            db_schema,
            input.trim()
        );
    
        Ok(prompt)
    }

}


