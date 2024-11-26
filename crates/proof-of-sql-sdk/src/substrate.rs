use futures::future::try_join_all;
use proof_of_sql::{
    base::{
        commitment::{QueryCommitments, TableCommitment},
        database::TableRef,
    },
    proof_primitive::dory::DynamicDoryCommitment,
};
use proof_of_sql_parser::ResourceId;
use subxt::{blocks::BlockRef, Config, OnlineClient, PolkadotConfig};
use sxt_proof_of_sql_sdk_local::{
    resource_id_to_table_id,
    sxt_chain_runtime::api::{
        runtime_types::proof_of_sql_commitment_map::{
            commitment_scheme::CommitmentScheme, commitment_storage_map::TableCommitmentBytes,
        },
        storage,
    },
};

/// Use the standard PolkadotConfig
pub type SxtConfig = PolkadotConfig;

/// Get the commitments for the given tables at the given SxT block.
///
/// If `block_ref` is `None`, the latest block is used.
pub async fn query_commitments<BR>(
    resource_ids: &[ResourceId],
    url: &str,
    block_ref: Option<BR>,
) -> Result<QueryCommitments<DynamicDoryCommitment>, Box<dyn core::error::Error>>
where
    BR: Into<BlockRef<<SxtConfig as Config>::Hash>> + Clone,
{
    let api = OnlineClient::<SxtConfig>::from_insecure_url(url).await?;

    // Create a collection of futures
    let futures = resource_ids.iter().map(|id| {
        let api = api.clone();
        let id = *id;
        let block_ref = block_ref.clone();
        async move {
            let table_id = resource_id_to_table_id(&id);
            let commitments_query = storage()
                .commitments()
                .commitment_storage_map(&table_id, &CommitmentScheme::DynamicDory);

            let storage_at_block_ref = match block_ref {
                Some(block_ref) => api.storage().at(block_ref),
                None => api.storage().at_latest().await?,
            };

            let table_commitment_bytes: TableCommitmentBytes = storage_at_block_ref
                .fetch(&commitments_query)
                .await?
                .ok_or("Commitment not found")?;
            let table_commitments = postcard::from_bytes(&table_commitment_bytes.data.0)?;
            Ok::<(TableRef, TableCommitment<DynamicDoryCommitment>), Box<dyn core::error::Error>>((
                TableRef::new(id),
                table_commitments,
            ))
        }
    });

    // Collect and await all futures concurrently
    let results = try_join_all(futures)
        .await?
        .into_iter()
        .collect::<QueryCommitments<DynamicDoryCommitment>>();
    Ok(results)
}
