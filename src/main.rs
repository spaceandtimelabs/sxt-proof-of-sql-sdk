mod auth;
mod substrate;
mod sxt_chain_runtime;

use dotenv::dotenv;
use proof_of_sql::{
    proof_primitive::dory::{
        DoryCommitment, DoryEvaluationProof, DoryVerifierPublicSetup, VerifierSetup,
    },
    sql::{parse::QueryExpr, proof::VerifiableQueryResult},
};
use prover::{ProverContextRange, ProverQuery, ProverResponse};
use reqwest::Client;
use std::{collections::HashMap, path::Path};

mod prover {
    tonic::include_proto!("sxt.core");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    // Load environment variables
    dotenv().ok();
    let sql = "SELECT * FROM ETHEREUM.CONTRACT_EVT_APPROVALFORALL;";
    let prover_root_url = std::env::var("PROVER_ROOT_URL")?;
    let substrate_node_url = std::env::var("SUBSTRATE_NODE_URL")?;
    // Dory setup
    let sigma = 12;
    let verifier_setup = VerifierSetup::load_from_file(Path::new("verifier_setup.bin"))?;
    let dory_verifier_setup = DoryVerifierPublicSetup::new(&verifier_setup, sigma);
    // Accessor setup
    let accessor = substrate::query_commitments(
        &["ETHEREUM.CONTRACT_EVT_APPROVALFORALL".parse()?],
        &substrate_node_url,
    )
    .await?;
    // Parse the SQL query
    let query: QueryExpr<DoryCommitment> =
        QueryExpr::try_new(sql.parse()?, "ETHEREUM".parse()?, &accessor)?;
    let proof_plan = query.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;
    // Send the query to the prover
    let mut query_context = HashMap::new();
    query_context.insert(
        "ETHEREUM.CONTRACT_EVT_APPROVALFORALL".to_string(),
        ProverContextRange {
            start: 0,
            ends: vec![5],
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
    let verifiable_result: VerifiableQueryResult<DoryEvaluationProof> =
        flexbuffers::from_slice(&stringified_verifiable_result)?;
    // Verify the proof
    let proof = verifiable_result.proof.unwrap();
    let serialized_result = verifiable_result.provable_result.unwrap();
    let owned_table_result = proof
        .verify(
            query.proof_expr(),
            &accessor,
            &serialized_result,
            &dory_verifier_setup,
        )?
        .table;
    println!("Query result: {:?}", owned_table_result);
    Ok(())
}
