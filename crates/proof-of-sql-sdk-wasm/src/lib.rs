use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use gloo_utils::format::JsValueSerdeExt;
use proof_of_sql::{
    base::commitment::{Commitment, QueryCommitments},
    proof_primitive::dory::{DynamicDoryEvaluationProof, VerifierSetup},
};
use proof_of_sql_parser::ResourceId;
use serde::Deserialize;
use sp_crypto_hashing::{blake2_128, twox_128};
use subxt::ext::codec::{Decode, Encode};
use sxt_proof_of_sql_sdk::{
    resource_id_to_table_id,
    sxt_chain_runtime::api::runtime_types::proof_of_sql_commitment_map::{
        commitment_scheme::CommitmentScheme, commitment_storage_map::TableCommitmentBytes,
    },
};
use wasm_bindgen::prelude::*;

const VERIFIER_SETUP_BYTES: &[u8; 47472] = include_bytes!("../../../verifier_setup.bin");

lazy_static::lazy_static! {
    static ref VERIFIER_SETUP: VerifierSetup = VerifierSetup::deserialize_with_mode(
        &VERIFIER_SETUP_BYTES[..],
        Compress::No,
        Validate::No,
    )
    .unwrap();
}

#[wasm_bindgen(getter_with_clone)]
pub struct ProverQueryAndQueryExprAndCommitments {
    pub prover_query_json: JsValue,
    pub query_expr_json: JsValue,
    pub commitments: Vec<TableRefAndCommitment>,
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableRefAndCommitment {
    table_ref: String,
    table_commitment_hex: String,
}

#[wasm_bindgen]
impl TableRefAndCommitment {
    #[wasm_bindgen(constructor)]
    pub fn new(table_ref: String, table_commitment_hex: String) -> Self {
        TableRefAndCommitment {
            table_ref,
            table_commitment_hex,
        }
    }
}

fn query_commitments_from_table_ref_and_commitment_iter<'a, C>(
    iter: impl IntoIterator<Item = &'a TableRefAndCommitment>,
) -> Result<QueryCommitments<C>, String>
where
    for<'de> C: Commitment + Deserialize<'de>,
{
    iter.into_iter()
        .map(
            |TableRefAndCommitment {
                 table_ref,
                 table_commitment_hex,
             }| {
                let table_ref = table_ref
                    .parse()
                    .map_err(|e| format!("failed to parse table ref: {e}"))?;
                let table_commitment_bytes = TableCommitmentBytes::decode(
                    &mut hex::decode(table_commitment_hex)
                        .map_err(|e| format!("failed to decode table commitment hex: {e}"))?
                        .as_slice(),
                )
                .map_err(|e| format!("failed to decode table commitment bytes: {e}"))?;

                let table_commitment =
                    postcard::from_bytes(&table_commitment_bytes.data.0.as_slice())
                        .map_err(|e| format!("failed to deserialize table commitment: {e}"))?;

                Ok((table_ref, table_commitment))
            },
        )
        .collect()
}

#[wasm_bindgen]
pub fn commitment_storage_key_dory(table_ref: &str) -> Result<String, String> {
    let resource_id: ResourceId = table_ref
        .parse()
        .map_err(|e| format!("failed to parse table ref: {e}"))?;

    let table_id = resource_id_to_table_id(&resource_id);

    let encoded_table_id = table_id.encode();

    let encoded_commitment_scheme = CommitmentScheme::DynamicDory.encode();

    let storage_key = twox_128(b"Commitments")
        .into_iter()
        .chain(twox_128(b"CommitmentStorageMap"))
        .chain(blake2_128(&encoded_table_id))
        .chain(encoded_table_id)
        .chain(blake2_128(&encoded_commitment_scheme))
        .chain(encoded_commitment_scheme)
        .collect::<Vec<_>>();

    Ok(hex::encode(storage_key))
}

#[wasm_bindgen]
pub fn plan_prover_query_dory(
    query: &str,
    commitments: Vec<TableRefAndCommitment>,
) -> Result<ProverQueryAndQueryExprAndCommitments, String> {
    let query_commitments = query_commitments_from_table_ref_and_commitment_iter(&commitments)
        .map_err(|e| format!("failed to construct QueryCommitments: {e}"))?;

    let (prover_query, query_expr) =
        sxt_proof_of_sql_sdk::plan_prover_query_dory(query, &query_commitments)
            .map_err(|e| format!("failed to plan prover query: {e}"))?;

    let prover_query_json = JsValue::from_serde(&prover_query)
        .map_err(|e| format!("failed to convert prover query to json: {e}"))?;

    let query_expr_json = JsValue::from_serde(&query_expr)
        .map_err(|e| format!("failed to convert query expr to json: {e}"))?;

    let result = ProverQueryAndQueryExprAndCommitments {
        prover_query_json,
        query_expr_json,
        commitments,
    };

    Ok(result)
}

#[wasm_bindgen]
pub fn verify_prover_response_dory(
    prover_response_json: JsValue,
    query_expr_json: JsValue,
    commitments: Vec<TableRefAndCommitment>,
) -> Result<JsValue, String> {
    let prover_response = prover_response_json
        .into_serde()
        .map_err(|e| format!("failed to deserialize prover response json: {e}"))?;

    let query_expr = query_expr_json
        .into_serde()
        .map_err(|e| format!("failed to deserialize query expr json: {e}"))?;

    let query_commitments = query_commitments_from_table_ref_and_commitment_iter(&commitments)
        .map_err(|e| format!("failed to construct QueryCommitments: {e}"))?;

    let verified_table_result =
        sxt_proof_of_sql_sdk::verify_prover_response::<DynamicDoryEvaluationProof>(
            &prover_response,
            &query_expr,
            &query_commitments,
            &&*VERIFIER_SETUP,
        )
        .map_err(|e| format!("verification failure: {e}"))?;

    let verified_table_result_json = JsValue::from_serde(&verified_table_result)
        .map_err(|e| format!("failed to convert verified table result to json: {e}"))?;

    Ok(verified_table_result_json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn storage_key_is_correct() {
        let expected = "ca407206ec1ab726b2636c4b145ac28749505e273536fae35330b966dac69e86a4832a125c0464e066dd20add960efb518424c4f434b5320455448455245554d4a9e6f9b8d43f6ad008f8c291929dee201";

        let actual = commitment_storage_key_dory("ETHEREUM.BLOCKS").unwrap();

        assert_eq!(actual, expected);
    }
}
