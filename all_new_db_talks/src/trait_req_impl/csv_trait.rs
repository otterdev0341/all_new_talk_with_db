use anyhow::Error;
use async_trait::async_trait;
use datafusion::arrow::array::RecordBatch;

#[async_trait]
pub trait CsvImplTrait {
    async fn execute_csv_query(&self, the_query: String) -> String;
}