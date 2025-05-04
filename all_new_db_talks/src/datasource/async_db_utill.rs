use std::collections::HashMap;

use anyhow::Error;
use mysql::{prelude::Queryable, Row};

use crate::configuration::db_config::DatabaseConfig;



pub struct AsyncDb {
    pool: mysql::Pool,
}

impl AsyncDb {
    pub fn new() -> Result<Self, Error> {
        let db_url = DatabaseConfig::inject_from_env().db_url;
        let pool = mysql::Pool::new(mysql::Opts::from_url(&db_url)?)?;
        Ok(Self { pool })
    }

    pub async fn query(&self, query: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>> {
        let mut conn = self.pool.get_conn()?;
        let result: Vec<Row> = conn.query(query)?;

        let mut rows = Vec::new();
        for row in result {
            let mut map = HashMap::new();
            for (i, col) in row.columns_ref().iter().enumerate() {
                let key = col.name_str().to_string();
                let value = row.as_ref(i).map(Self::value_to_string).unwrap_or_default();
                map.insert(key, value);
            }
            rows.push(map);
        }

        Ok(rows)
    }

    pub async fn query_as_string(&self, generated_query: String) -> Result<String, Error> {
        let async_db = AsyncDb::new()?;
        let result = async_db.query(&generated_query).await.unwrap();
    
        let mut output = String::new();
        for (i, row) in result.iter().enumerate() {
            output.push_str(&format!("Row {}:\n", i + 1));
            for (key, value) in row {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }
    
        Ok(output)
    }

    fn value_to_string(val: &mysql::Value) -> String {
        match val {
            mysql::Value::NULL => "NULL".to_string(),
            mysql::Value::Bytes(bytes) => String::from_utf8_lossy(bytes).to_string(),
            mysql::Value::Int(i) => i.to_string(),
            mysql::Value::UInt(u) => u.to_string(),
            mysql::Value::Float(f) => f.to_string(),
            mysql::Value::Double(d) => d.to_string(),
            mysql::Value::Date(y, m, d, h, min, s, micros) => {
                format!(
                    "{:04}-{:02}-{:02} {:02}:{:02}:{:02}.{:06}",
                    y, m, d, h, min, s, micros
                )
            }
            mysql::Value::Time(is_neg, d, h, m, s, micros) => {
                let sign = if *is_neg { "-" } else { "" };
                // Dereferencing `d` and `h` here:
                let total_hours = (*d as u32) * 24 + (*h as u32);
                format!("{sign}{:03}:{:02}:{:02}.{:06}", total_hours, m, s, micros)
            }
        }
    }
}
