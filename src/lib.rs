mod args;
pub use args::SdkArgs;
mod auth;
mod substrate;
mod sxt_chain_runtime;

use proof_of_sql::{
    base::database::{OwnedTable, TableRef},
    proof_primitive::dory::{
        DoryScalar, DynamicDoryCommitment, DynamicDoryEvaluationProof, VerifierSetup,
    },
    sql::{parse::QueryExpr, proof::VerifiableQueryResult},
};
use prover::{ProverContextRange, ProverQuery, ProverResponse};
use reqwest::Client;
use std::{collections::HashMap, path::Path};

mod prover {
    tonic::include_proto!("sxt.core");
}

/// Query and verify a SQL query
///
/// Run a SQL query and verify the result using Dynamic Dory.
pub async fn query_and_verify(
    args: &SdkArgs,
) -> Result<OwnedTable<DoryScalar>, Box<dyn core::error::Error>> {
    // Parse table_ref into TableRef struct
    let table_ref = TableRef::new(args.table_ref.parse()?);
    let schema = table_ref.schema_id();
    let verifier_setup_path = Path::new(&args.verifier_setup);
    let verifier_setup = VerifierSetup::load_from_file(&verifier_setup_path)?;
    // Accessor setup
    let accessor =
        substrate::query_commitments(&[table_ref.resource_id()], &args.substrate_node_url).await?;
    // Parse the SQL query
    let query: QueryExpr<DynamicDoryCommitment> =
        QueryExpr::try_new(args.query.parse()?, schema, &accessor)?;
    let proof_plan = query.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;
    // Send the query to the prover
    let mut query_context = HashMap::new();
    let commitment_range = accessor[&table_ref].range();
    query_context.insert(
        table_ref.to_string().to_uppercase(),
        ProverContextRange {
            start: commitment_range.start as u64,
            ends: vec![commitment_range.end as u64],
        },
    );
    let prover_query = ProverQuery {
        proof_plan: serialized_proof_plan,
        query_context,
        commitment_scheme: 1,
    };
    let client = Client::new();
    let access_token = auth::get_access_token(&args.sxt_api_key, &args.auth_root_url).await?;
    let response = client
        .post(format!("https://{}/v1/prove", &args.prover_root_url))
        .bearer_auth(&access_token)
        .json(&prover_query)
        .send()
        .await?
        .error_for_status()?;
    let serialized_prover_response = response.text().await?;
    let prover_response = serde_json::from_str::<ProverResponse>(&serialized_prover_response)
        .map_err(|_e| {
            format!(
                "Failed to parse prover response: {}",
                &serialized_prover_response
            )
        })?;
    let stringified_verifiable_result = prover_response.verifiable_result.clone();
    let verifiable_result: VerifiableQueryResult<DynamicDoryEvaluationProof> =
        flexbuffers::from_slice(&stringified_verifiable_result)?;
    // Verify the proof
    let proof = verifiable_result.proof.unwrap();
    let serialized_result = verifiable_result.provable_result.unwrap();
    let owned_table_result = proof
        .verify(
            query.proof_expr(),
            &accessor,
            &serialized_result,
            &&verifier_setup,
        )?
        .table;
    Ok(owned_table_result)
}
