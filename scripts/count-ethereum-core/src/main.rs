use futures::StreamExt;
use indexmap::IndexMap;
use proof_of_sql::base::database::OwnedColumn;
use std::{env, fs::File, io::BufReader, sync::Arc};
use sxt_proof_of_sql_sdk::SxTClient;

#[allow(dead_code)]
const ETHEREUM_CORE_COUNTS_FILE: &str = "ethereum-core-counts.json";

const ETHEREUM_CORE_TABLES: [&str; 21] = [
    "ETHEREUM.BLOCKS",
    "ETHEREUM.BLOCK_DETAILS",
    "ETHEREUM.TRANSACTIONS",
    "ETHEREUM.TRANSACTION_DETAILS",
    "ETHEREUM.CONTRACTS",
    "ETHEREUM.TOKENS",
    "ETHEREUM.NFT_COLLECTIONS",
    "ETHEREUM.NFTS",
    "ETHEREUM.NATIVETOKEN_TRANSFERS",
    "ETHEREUM.ERC20_EVT_TRANSFER",
    "ETHEREUM.ERC20_EVT_APPROVAL",
    "ETHEREUM.ERC721_EVT_TRANSFER",
    "ETHEREUM.ERC721_EVT_APPROVAL",
    "ETHEREUM.ERC1155_EVT_TRANSFER",
    "ETHEREUM.CONTRACT_EVT_APPROVALFORALL",
    "ETHEREUM.CONTRACT_EVT_OWNERSHIPTRANSFERRED",
    "ETHEREUM.ERC1155_EVT_TRANSFERBATCH",
    "ETHEREUM.NATIVE_WALLETS",
    "ETHEREUM.FUNGIBLETOKEN_WALLETS",
    "ETHEREUM.ERC721_OWNERS",
    "ETHEREUM.ERC1155_OWNERS",
];

async fn count_table(
    client: &SxTClient,
    table_ref: &str,
) -> Result<i64, Box<dyn core::error::Error>> {
    let uppercased_table_ref = table_ref.to_uppercase();
    let query = format!("SELECT * FROM {uppercased_table_ref}");
    let table = client
        .query_and_verify(&query, &uppercased_table_ref)
        .await?;
    assert_eq!(table.num_columns(), 1);
    assert_eq!(table.num_rows(), 1);

    let column = table.into_inner().swap_remove_index(0).unwrap().1;
    let OwnedColumn::BigInt(int_column) = column else {
        panic!("count query should return an int64 column")
    };

    Ok(int_column[0])
}

/// Load the previous counts file
#[allow(dead_code)]
async fn load_from_file() -> IndexMap<String, i64> {
    let file = File::open(ETHEREUM_CORE_COUNTS_FILE).expect("failed to open file");
    let mut reader = BufReader::new(&file);
    serde_json::from_reader(&mut reader).expect("failed to parse file")
}

/// Save the current counts to a file
#[allow(dead_code)]
async fn save_to_file(counts: IndexMap<String, i64>) {
    let file = File::create(ETHEREUM_CORE_COUNTS_FILE).expect("failed to create file");
    serde_json::to_writer(&file, &counts).expect("failed to write file");
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let client = Arc::new(SxTClient::new(
        env::var("PROVER_ROOT_URL").unwrap_or("api.spaceandtime.dev".to_string()),
        env::var("AUTH_ROOT_URL").unwrap_or("api.spaceandtime.dev".to_string()),
        env::var("SUBSTRATE_NODE_URL").unwrap_or("foo.bar.spaceandtime.dev".to_string()),
        env::var("SXT_API_KEY").expect("SXT_API_KEY is required"),
        env::var("VERIFIER_SETUP").unwrap_or("verifier_setup.bin".to_string()),
    ));

    let current_counts: IndexMap<String, i64> = futures::stream::iter(ETHEREUM_CORE_TABLES)
        .filter_map(|table_ref| {
            let client = client.clone();
            async move {
                log::info!("querying count of {table_ref}");
                let count = count_table(client.as_ref(), table_ref)
                    .await
                    .inspect_err(|e| log::error!("failed to query count for {table_ref}: {e}"))
                    .ok()?;

                Some((table_ref.to_string(), count))
            }
        })
        .collect()
        .await;
    println!("current_counts: {current_counts:?}");
}
