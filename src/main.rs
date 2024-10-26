#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Load environment variables
    let table = "ETHEREUM.CONTRACT_EVT_APPROVALFORALL";
    let sql = format!("SELECT * FROM {table};");
    let owned_table_result = sxt_proof_of_sql_sdk::query_and_verify(&sql, table.parse()?).await?;
    println!("Query result: {:?}", owned_table_result);
    Ok(())
}
