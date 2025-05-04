use std::collections::HashMap;
use std::fmt;

use diesel::prelude::*;
use diesel::{Connection, MysqlConnection, QueryResult, RunQueryDsl};
use diesel::sql_query;
use diesel::sql_types::Text;
use anyhow::{Error, Result};
use mysql::prelude::Queryable;
use mysql::{Opts, Pool, Row};
use serde::{Deserialize, Serialize};
use anyhow::anyhow;

use crate::configuration::db_config::DatabaseConfig;

pub struct DbUtil {
    pub db_con: MysqlConnection,
}

#[derive(Debug, QueryableByName, Deserialize, Serialize, Clone)]
pub struct TableName {
    #[diesel(sql_type = Text)]
    table_name: String,
}

#[derive(Debug, QueryableByName, Deserialize, Serialize, Clone)]
pub struct ColumnName {
    #[diesel(sql_type = Text)]
    column_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TableSchema {
    pub table_name: String,
    pub field_columns: Vec<ColumnName>,
}

impl Default for TableSchema {
    fn default() -> Self {
        Self {
            table_name: String::new(),
            field_columns: Vec::new(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseSchema {
    pub schemas: Vec<TableSchema>,
}

impl Default for DatabaseSchema {
    fn default() -> Self {
        Self {
            schemas: Vec::new(),
        }
    }
}

impl fmt::Display for DatabaseSchema {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for table in &self.schemas {
            writeln!(f, "Table: {}", table.table_name)?;
            for column in &table.field_columns {
                writeln!(f, "  - {}", column.column_name)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for ColumnName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.column_name)
    }
}

impl DbUtil {
    /// Construct a new DbUtil with a fresh MySQL connection
    pub fn new() -> Result<Self> {
        let db_config = crate::configuration::db_config::DatabaseConfig::inject_from_env();
        let connection = MysqlConnection::establish(&db_config.db_url)?;
        Ok(Self {
            db_con: connection,
        })
    }

    /// Collect full schema: tables + columns
    pub fn get_database_schema(&mut self) -> DatabaseSchema {
        let mut database_schema = DatabaseSchema::default();

        let all_tables = match self.get_all_tables() {
            Ok(tables) => tables,
            Err(err) => {
                eprintln!("Failed to retrieve tables: {}", err);
                return database_schema;
            }
        };

        for table_name in all_tables.iter() {
            let mut table_schema = TableSchema {
                table_name: table_name.to_string(),
                ..Default::default()
            };

            match self.get_columns_for_table(table_name) {
                Ok(columns) => {
                    table_schema.field_columns = columns
                        .into_iter()
                        .map(|col_name| ColumnName {
                            column_name: col_name,
                        })
                        .collect();
                }
                Err(err) => {
                    eprintln!("Failed to get columns for table '{}': {}", table_name, err);
                    continue;
                }
            }

            database_schema.schemas.push(table_schema);
        }

        database_schema
    }

    /// Get all table names in the current database schema
    fn get_all_tables(&mut self) -> QueryResult<Vec<String>> {
        let db_name = "Chinook";
        let query = format!(
            "SELECT table_name AS `table_name` FROM information_schema.tables WHERE table_schema = '{}'",
            db_name
        );

        let results: Vec<TableName> = sql_query(query).load(&mut self.db_con)?;
        Ok(results.into_iter().map(|r| r.table_name).collect())
    }

    /// Get all columns (fields) for a specific table
    fn get_columns_for_table(&mut self, table_name: &str) -> QueryResult<Vec<String>> {
        let db_name = "Chinook";
        let query = format!(
            "SELECT column_name AS `column_name` FROM information_schema.columns WHERE table_schema = '{}' AND table_name = '{}'",
            db_name, table_name
        );

        let results: Vec<ColumnName> = sql_query(query).load(&mut self.db_con)?;
        Ok(results.into_iter().map(|r| r.column_name).collect())
    }


    pub async fn query(&self, query: &str) -> Result<Vec<HashMap<String, String>>, Box<dyn std::error::Error>>{
        let url = DatabaseConfig::inject_from_env(); // adjust as needed
        let db_option = Opts::from_url(&url.db_url).unwrap();
        let pool = Pool::new(db_option)?;
        let mut conn = pool.get_conn()?;
    
        let result: Vec<Row> = conn.query(query)?;
    
        let mut rows: Vec<HashMap<String, String>> = Vec::new();
    
        for row in result {
            let mut map = HashMap::new();
            for (i, col) in row.columns_ref().iter().enumerate() {
                let key = col.name_str().to_string();
                let value = row.as_ref(i).map(|v| Self::value_to_string(v)).unwrap_or_default();

                map.insert(key, value);
            }
            rows.push(map);
        }
    
        Ok(rows)
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

    pub async fn query_as_string(&self, generated_query: String) -> Result<String, Error> {
        let db_util = match DbUtil::new() {
            Ok(db_util) => db_util,
            Err(_) => return Err(anyhow!("fail to connect to db")),
        };
    
        let result = db_util.query(&generated_query).await.unwrap();
    
        // Convert the Vec<HashMap<String, String>> into a formatted string
        let mut output = String::new();
        for (i, row) in result.iter().enumerate() {
            output.push_str(&format!("Row {}:\n", i + 1));
            for (key, value) in row {
                output.push_str(&format!("  {}: {}\n", key, value));
            }
        }
    
        Ok(output)
    }
    
}





#[cfg(test)]
mod test {
    use super::DbUtil;

    #[test]
    fn test_database_tables_and_columns() {
        let mut db_util = DbUtil::new().unwrap();
        let result_tables = db_util.get_all_tables().unwrap();

        for table in result_tables.iter() {
            println!("Table: {:?}", table);
            let columns = db_util.get_columns_for_table(table).unwrap();
            for column in columns.iter() {
                println!("  Column: {}", column);
            }
        }
    }

    #[test]
    fn test_database_schema_struct() {
        let mut db_util = DbUtil::new().unwrap();
        let schema = db_util.get_database_schema();
        println!("{:#?}", schema);
    }

    #[tokio::test]
    async fn test_query() {
        let db_util = DbUtil::new().unwrap();
        let sql = "SELECT table_name FROM information_schema.tables WHERE table_schema = 'Chinook';";
        let result = db_util.query(sql).await.unwrap();
        print!("{:?}", result);
    }


}
