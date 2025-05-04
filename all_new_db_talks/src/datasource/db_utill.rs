use diesel::prelude::*;
use diesel::{Connection, MysqlConnection, QueryResult, RunQueryDsl};
use diesel::sql_query;
use diesel::sql_types::Text;
use anyhow::Result;
use serde::{Deserialize, Serialize};

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
}
