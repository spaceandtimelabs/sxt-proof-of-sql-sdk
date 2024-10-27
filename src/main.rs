use clap::Parser;
use dotenv::dotenv;
use sxt_proof_of_sql_sdk::{query_and_verify, SdkArgs};

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Load environment variables from .env file, if available
    dotenv().ok();

    // Parse command-line arguments
    let args = SdkArgs::parse();

    // Execute the query and verify the result
    let result = query_and_verify(&args).await?;

    // Print the result of the query
    println!("Query result: {:?}", result);

    Ok(())
}
