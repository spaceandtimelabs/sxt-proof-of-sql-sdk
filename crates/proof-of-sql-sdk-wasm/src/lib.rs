use ark_serialize::{CanonicalDeserialize, Compress, Validate};
use arrow_array::RecordBatch;
use arrow_ipc::writer::StreamWriter;
use gloo_utils::format::JsValueSerdeExt;
use parity_scale_codec::Decode;
use proof_of_sql::{
    base::commitment::{Commitment, QueryCommitments},
    proof_primitive::dory::{DynamicDoryEvaluationProof, VerifierSetup},
};
use serde::Deserialize;
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
pub struct ProverQueryAndQueryExpr {
    pub prover_query_json: JsValue,
    pub query_expr_json: JsValue,
}

#[derive(Decode)]
struct TableCommitmentBytes {
    data: Vec<u8>,
}

#[wasm_bindgen]
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
) -> Result<QueryCommitments<C>, ()>
where
    for<'de> C: Commitment + Deserialize<'de>,
{
    Ok(iter
        .into_iter()
        .map(
            |TableRefAndCommitment {
                 table_ref,
                 table_commitment_hex,
             }| {
                let table_ref = table_ref.parse().expect("TODO");
                let table_commitment_bytes = TableCommitmentBytes::decode(
                    &mut hex::decode(table_commitment_hex).expect("TODO").as_slice(),
                )
                .expect("TODO");

                let table_commitment =
                    postcard::from_bytes(&table_commitment_bytes.data.as_slice()).expect("TODO");

                (table_ref, table_commitment)
            },
        )
        .collect())
}

#[wasm_bindgen]
pub fn plan_prover_query_dory(
    query: &str,
    commitments: Vec<TableRefAndCommitment>,
) -> Result<ProverQueryAndQueryExpr, String> {
    let query_commitments =
        query_commitments_from_table_ref_and_commitment_iter(&commitments).expect("TODO");

    let (prover_query, query_expr) =
        sxt_proof_of_sql_sdk::plan_prover_query_dory(query, &query_commitments).expect("TODO");

    let prover_query_json = JsValue::from_serde(&prover_query).expect("TODO");

    let query_expr_json = JsValue::from_serde(&query_expr).expect("TODO");

    let result = ProverQueryAndQueryExpr {
        prover_query_json,
        query_expr_json,
    };

    Ok(result)
}

#[wasm_bindgen]
pub fn verify_prover_response_dory(
    prover_response_json: JsValue,
    query_expr_json: JsValue,
    commitments: Vec<TableRefAndCommitment>,
) -> Result<Vec<u8>, String> {
    let prover_response = prover_response_json.into_serde().expect("TODO");

    let query_expr = query_expr_json.into_serde().expect("TODO");

    let query_commitments =
        query_commitments_from_table_ref_and_commitment_iter(&commitments).expect("TODO");

    let verified_table_result =
        sxt_proof_of_sql_sdk::verify_prover_response::<DynamicDoryEvaluationProof>(
            &prover_response,
            &query_expr,
            &query_commitments,
            &&*VERIFIER_SETUP,
        )
        .expect("TODO");

    let verified_batch = RecordBatch::try_from(verified_table_result).expect("TODO");

    let mut batch_buffer = Vec::new();

    let mut batch_writer =
        StreamWriter::try_new(&mut batch_buffer, &verified_batch.schema()).expect("TODO");

    batch_writer.write(&verified_batch).expect("TODO");

    batch_writer.finish().expect("TODO");

    batch_writer.into_inner().expect("TODO");

    Ok(batch_buffer)
}
