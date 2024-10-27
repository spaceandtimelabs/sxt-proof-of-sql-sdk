use std::env;
use std::fs::File;
use std::path::Path;

use clap::Parser;
use futures::StreamExt;
use indexmap::IndexMap;
use proof_of_sql::base::database::{OwnedColumn, OwnedTable};
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
    let query = format!("SELECT COUNT(*) FROM {table_ref}");
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

#[tokio::test]
async fn count_ethereum_tables() {
    env::set_var("QUERY", "SELECT COUNT(*) FROM ETHEREUM.PLACEHOLDER");
    env::set_var("TABLE_REF", "ETHEREUM.PLACEHOLDER");

    dotenv::dotenv().unwrap();

    let base_args = SdkArgs::parse();

    let current_counts: IndexMap<&str, i64> = futures::stream::iter(ETHEREUM_CORE_TABLES)
        .filter_map(|table_ref| {
            let base_args = base_args.clone();
            async move {
                log::info!("querying count of {table_ref}");
                let count = count_table(table_ref.to_string(), base_args)
                    .await
                    .inspect_err(|e| log::error!("failed to query count for {table_ref}: {e}"))
                    .ok()?;

                Some((table_ref, count))
            }
        })
        .collect()
        .await;
}
