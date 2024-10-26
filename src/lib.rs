mod auth;
mod substrate;
mod sxt_chain_runtime;

use dotenv::dotenv;
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

pub async fn query_and_verify(
    sql: &str,
    table_id: TableRef,
) -> Result<OwnedTable<DoryScalar>, Box<dyn core::error::Error>> {
    dotenv().ok();
    let prover_root_url = std::env::var("PROVER_ROOT_URL")?;
    let substrate_node_url = std::env::var("SUBSTRATE_NODE_URL")?;
    //let verifier_setup = VerifierSetup::from(&public_parameters);
    let verifier_setup = VerifierSetup::load_from_file(Path::new("verifier_setup.bin"))?;
    // Accessor setup
    let accessor =
        substrate::query_commitments(&[table_id.resource_id()], &substrate_node_url).await?;
    // Parse the SQL query
    let query: QueryExpr<DynamicDoryCommitment> =
        QueryExpr::try_new(sql.parse()?, "ETHEREUM".parse()?, &accessor)?;
    let proof_plan = query.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;
    // Send the query to the prover
    let mut query_context = HashMap::new();
    let table_ref = TableRef::new("ETHEREUM.CONTRACT_EVT_APPROVALFORALL".parse()?);
    let commitment_range = accessor[&table_ref].range();
    query_context.insert(
        table_id.to_string().to_uppercase(),
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
    let apikey = std::env::var("SXT_API_KEY")?;
    // Usually it is the same as the prover root URL
    let auth_root_url = std::env::var("AUTH_ROOT_URL")?;
    let access_token = auth::get_access_token(&apikey, &auth_root_url).await?;
    let response = client
        .post(format!("https://{}/v1/prove", prover_root_url))
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
