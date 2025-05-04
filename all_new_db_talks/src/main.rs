use std::io::{self, Write};

use all_new_db_talks::{agent::text_to_sql::TextToSqlChain, datasource::db_utill::DbUtil, trait_req_impl::chain::Chain};


#[tokio::main]
async fn main() {
    let processor = TextToSqlChain::initialze().await.unwrap();
    let mut input = String::new();
    println!("How can i help you: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input)
        .expect("Failed to read line");
    let output = processor.run(input)
        .await
        .unwrap();
    println!("Response {:?}", output);
}
