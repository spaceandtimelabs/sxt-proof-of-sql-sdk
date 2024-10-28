use std::env;
use std::fs::File;
use std::io::BufReader;

use futures::StreamExt;
use indexmap::IndexMap;
use proof_of_sql::base::database::OwnedColumn;
use sxt_proof_of_sql_sdk::{query_and_verify, SdkArgs};

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
    table_ref: String,
    base_args: SdkArgs,
) -> Result<i64, Box<dyn core::error::Error>> {
    let query = format!("SELECT * FROM {table_ref}");
    let args = SdkArgs {
        table_ref,
        query,
        ..base_args
    };

    let table = query_and_verify(&args).await?;
    assert_eq!(table.num_columns(), 1);
    assert_eq!(table.num_rows(), 1);

    let column = table.into_inner().swap_remove_index(0).unwrap().1;
    let OwnedColumn::BigInt(int_column) = column else {
        panic!("count query should return an int64 column")
    };

    Ok(int_column[0])
}

/// Load the previous counts file
async fn load_from_file() -> IndexMap<String, i64> {
    let file = File::open(ETHEREUM_CORE_COUNTS_FILE).expect("failed to open file");
    let mut reader = BufReader::new(&file);
    serde_json::from_reader(&mut reader).expect("failed to parse file")
}

/// Save the current counts to a file
async fn save_to_file(counts: IndexMap<String, i64>) {
    let file = File::create(ETHEREUM_CORE_COUNTS_FILE).expect("failed to create file");
    serde_json::to_writer(&file, &counts).expect("failed to write file");
}

#[tokio::main]
async fn main() {
    env_logger::init();
    dotenv::dotenv().unwrap();

    let base_args = SdkArgs {
        prover_root_url: env::var("PROVER_ROOT_URL").unwrap_or("api.spaceandtime.dev".to_string()),
        auth_root_url: env::var("AUTH_ROOT_URL").unwrap_or("api.spaceandtime.dev".to_string()),
        substrate_node_url: env::var("SUBSTRATE_NODE_URL")
            .unwrap_or("foo.bar.spaceandtime.dev".to_string()),
        verifier_setup: env::var("VERIFIER_SETUP").unwrap_or("verifier_setup.bin".to_string()),
        sxt_api_key: env::var("SXT_API_KEY").expect("SXT_API_KEY is required"),
        query: "SELECT COUNT(*) FROM PLACEHOLDER".to_string(),
        table_ref: "PLACEHOLDER".to_string(),
    };

    let current_counts: IndexMap<String, i64> = futures::stream::iter(ETHEREUM_CORE_TABLES)
        .filter_map(|table_ref| {
            let base_args = base_args.clone();
            async move {
                log::info!("querying count of {table_ref}");
                let count = count_table(table_ref.to_string().to_uppercase(), base_args)
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
