use std::{path::Path, sync::Arc};
use anyhow::{anyhow, Error};
use arrow::array::{Array, BooleanArray, Float64Array, Int64Array, StringArray};
use datafusion::{arrow::array::RecordBatch, prelude::SessionConfig};
use diesel::result;
use rust_csv::CsvFile;
use async_trait::async_trait;
use datafusion::prelude::*;
use crate::trait_req_impl::csv_trait::CsvImplTrait;

pub struct CsvUtill{
    file_path: String,
    table_name: String
}

impl CsvUtill{

    // new funtion that take file path and return self
    pub fn new(the_path: String) -> Self {
        Self { 
            file_path: the_path,
            table_name: "target".to_string()
         }
    }

    // verify is file exist
    pub fn verify_path(&self) -> bool {
        let is_exist = Path::new(&self.file_path).exists();
        is_exist
    }
    
     // Method to get the schema (first few rows) of the CSV file
     pub fn get_schema(&self)  {
        // Read the CSV file lazily
        let csv_file = CsvFile::read(self.file_path.to_string()).unwrap();
        let cols = csv_file.heads();
        print!("{:?}",cols);
    }
    
    pub fn record_batches_to_string(batches: Vec<RecordBatch>) -> String {
        let mut output = String::new();
    
        for batch in batches {
            let schema = batch.schema();
            let columns = batch.columns();
    
            // Header row
            let headers: Vec<String> = schema.fields().iter().map(|f| f.name().to_string()).collect();
            output.push_str(&headers.join(" | "));
            output.push('\n');
            output.push_str(&headers.iter().map(|_| "----".to_string()).collect::<Vec<_>>().join(" | "));
            output.push('\n');
    
            // Row values
            for row_index in 0..batch.num_rows() {
                let mut row = Vec::new();
    
                for column in columns {
                    let value = Self::array_value_to_string(column, row_index);
                    row.push(value);
                }
    
                output.push_str(&row.join(" | "));
                output.push('\n');
            }
    
            output.push('\n');
        }
    
        output
    }
    
    fn array_value_to_string(array: &Arc<dyn Array>, index: usize) -> String {
        if array.is_null(index) {
            return "NULL".to_string();
        }
    
        if let Some(arr) = array.as_any().downcast_ref::<StringArray>() {
            return arr.value(index).to_string();
        } else if let Some(arr) = array.as_any().downcast_ref::<Int64Array>() {
            return arr.value(index).to_string();
        } else if let Some(arr) = array.as_any().downcast_ref::<Float64Array>() {
            return arr.value(index).to_string();
        } else if let Some(arr) = array.as_any().downcast_ref::<BooleanArray>() {
            return arr.value(index).to_string();
        }
    
        "[unsupported type]".to_string()
    }
    // perform query

    
}

#[async_trait]
impl CsvImplTrait for CsvUtill {
    async fn execute_csv_query(&self, the_query: String) -> String {
    let mut ctx = SessionContext::new();

    ctx.register_csv(
        "products",
        self.file_path.to_string(),
        CsvReadOptions::new()
            .has_header(true)
            .delimiter(b',')
    ).await.unwrap();

    let df = ctx.sql(&the_query)
        .await
        .expect("Failed to execute query");

    let results = df.collect()
        .await
        .expect("Failed to collect results");

    // Use the helper to convert RecordBatches to string
    Self::record_batches_to_string(results)
} 
}


#[cfg(test)]
pub mod test {
    use crate::trait_req_impl::csv_trait::CsvImplTrait;
    use super::CsvUtill;

    #[tokio::test]
    pub async fn test_csv_read() {
        let path = "/home/otterdev_ball/BaseDiskProject/rust_project/all_new_talk_with_db/products-100.csv";
        let csv_utill = CsvUtill::new(path.to_string());

        let result_string = csv_utill.execute_csv_query("SELECT * FROM products WHERE price < 100".to_string()).await;

        // Just print the output to visually confirm
        println!("{}", result_string);

        // Optional: Add a simple assertion
        assert!(result_string.contains("price"), "Expected output to include header 'price'");
    }
}