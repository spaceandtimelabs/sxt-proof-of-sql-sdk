use crate::sxt_chain_runtime::api::{
    commitments::storage::types::stored_public_parameters::StoredPublicParameters,
    runtime_types::{
        bounded_collections::bounded_vec::BoundedVec,
        proof_of_sql_commitment_map::{
            commitment_scheme::CommitmentScheme, commitment_storage_map::TableCommitmentBytes,
        },
        sxt_core::tables::TableIdentifier,
    },
    storage,
};
use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use futures::future::try_join_all;
use proof_of_sql::{
    base::{
        commitment::{QueryCommitments, TableCommitment},
        database::TableRef,
    },
    proof_primitive::dory::{DoryCommitment, PublicParameters},
};
use proof_of_sql_parser::{Identifier, ResourceId};
use subxt::{OnlineClient, PolkadotConfig};

/// Use the standard PolkadotConfig
type SxtConfig = PolkadotConfig;

/// Convert PoSQL `Identifier` to SxT Core `BoundedVec<u8>`
fn identifier_to_byte_string(identifier: &Identifier) -> BoundedVec<u8> {
    BoundedVec::<u8>(identifier.as_str().as_bytes().to_vec())
}

/// Convert PoSQL resource IDs to SxT Core table identifiers
fn resource_id_to_table_id(resource_id: &ResourceId) -> TableIdentifier {
    TableIdentifier {
        name: identifier_to_byte_string(&resource_id.object_name()),
        namespace: identifier_to_byte_string(&resource_id.schema()),
    }
}

/// Query the commitments pallet to find which commitments
pub async fn query_commitments(
    resource_ids: &[ResourceId],
    url: &str,
) -> Result<QueryCommitments<DoryCommitment>, Box<dyn core::error::Error>> {
    let api = OnlineClient::<SxtConfig>::from_url(url).await?;

    // Create a collection of futures
    let futures = resource_ids.iter().map(|id| {
        let api = api.clone();
        let id = *id;
        async move {
            let table_id = resource_id_to_table_id(&id);
            let commitments_query = storage()
                .commitments()
                .commitment_storage_map(&table_id, &CommitmentScheme::Dory);
            let table_commitment_bytes: TableCommitmentBytes = api
                .storage()
                .at_latest()
                .await?
                .fetch(&commitments_query)
                .await?
                .ok_or("Commitment not found")?;
            let table_commitments = postcard::from_bytes(&table_commitment_bytes.data.0)?;
            Ok::<(TableRef, TableCommitment<DoryCommitment>), Box<dyn core::error::Error>>((
                TableRef::new(id),
                table_commitments,
            ))
        }
    });

    // Collect and await all futures concurrently
    let results = try_join_all(futures)
        .await?
        .into_iter()
        .collect::<QueryCommitments<DoryCommitment>>();
    Ok(results)
}

/// Obtain stored [`PublicParameters`] from the commitments pallet
pub async fn try_get_public_parameters(
    url: &str,
) -> Result<PublicParameters, Box<dyn core::error::Error>> {
    let api = OnlineClient::<SxtConfig>::from_url(url).await?;
    let public_parameters_query = storage().commitments().stored_public_parameters();
    let public_parameters_bytes: StoredPublicParameters = api
        .storage()
        .at_latest()
        .await?
        .fetch(&public_parameters_query)
        .await?
        .ok_or("Commitment not found")?;
    Ok(PublicParameters::deserialize_with_mode(
        &*public_parameters_bytes.data.0,
        Compress::No,
        Validate::No,
    )?)
}
