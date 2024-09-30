use proof_of_sql::base::{commitment::{QueryCommitments, TableCommitment}, database::TableRef};
use proof_of_sql_parser::{Identifier, ResourceId};
use proof_of_sql_commitment_map::CommitmentScheme;
use subxt::{
    client::OfflineClientT,
    utils::{AccountId32, MultiAddress},
    OnlineClient, PolkadotConfig,
};
use subxt_signer::sr25519::dev::{self};
use crate::sxt_chain_runtime::api::runtime_types::sxt_core::{ByteString, tables::TableIdentifier};

/// Derive the runtime from the metadata
#[subxt::subxt(runtime_metadata_path = "sxt.scale")]
pub mod sxt_runtime {}
/// Use the standard PolkadotConfig
type SxtConfig = PolkadotConfig;

/// Convert PoSQL `Identifier` to SxT Core `ByteString`
fn identifier_to_byte_string(identifier: &Identifier) -> ByteString {
    let byte_string = ByteString::new();
    let name = identifier.as_str().as_bytes();
    // Unwrapping is allowed since both PoSQL and SxT Core identifiers have the same length restrictions
    ByteString.try_extend(name).unwrap()
}

/// Convert PoSQL resource IDs to SxT Core table identifiers
fn resource_id_to_table_id(resource_id: &ResourceId) -> TableIdentifier {
    TableIdentifier {
        name: identifier_to_byte_string(resource_id.object_name()),
        namespace: identifier_to_byte_string(resource_id.schema()),
    }
}

/// Query the commitments pallet to find which commitments
pub fn query_commitments(
    resource_ids: &[ResourceId],
    url: &str,
    commitment_scheme: CommitmentScheme,
) -> Result<QueryCommitments, Box<dyn std::error::Error>> {
    let api = OnlineClient::<SxtConfig>::from_url(url).await?;
    let mut accessor = QueryCommitments::new();
    resource_ids.iter().map(|id| -> Result<(TableRef, TableCommitment), Box<dyn std::error::Error>>{
        let table_id = resource_id_to_table_id(id);
        let commitments_query = sxt_runtime::storage()
            .commitments()
            .commitments(table_id, commitment_scheme);
        let table_commitments: TableCommitment = api
            .storage()
            .at_latest()
            .await?
            .fetch(&commitments_query)
            .await?
            .unwrap();
        (TableRef::new(id), commitments)
    }).collect::<Result<QueryCommitments, Box<dyn std::error::Error>>>()
}
