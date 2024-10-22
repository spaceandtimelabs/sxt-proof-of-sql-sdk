mod substrate;
mod sxt_chain_runtime;

use core::time::Duration;
use proof_of_sql::{
    proof_primitive::dory::{
        DoryCommitment, DoryEvaluationProof, DoryVerifierPublicSetup, VerifierSetup,
    },
    sql::{parse::QueryExpr, proof::VerifiableQueryResult},
};
use prover::{prover_client::ProverClient, ProverContextRange, ProverQuery};
use std::collections::HashMap;

mod prover {
    tonic::include_proto!("sxt.core");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn core::error::Error>> {
    let sql = "SELECT * FROM ethereum.contract_evt_approvalforall;";
    let prover_uri = "<URI>";
    let substrate_node_url = "<URL>";
    // Dory setup
    let sigma = 12;
    let public_parameters = substrate::try_get_public_parameters(substrate_node_url).await?;
    let verifier_setup = VerifierSetup::from(&public_parameters);
    let dory_verifier_setup = DoryVerifierPublicSetup::new(&verifier_setup, sigma);
    // Accessor setup
    let accessor = substrate::query_commitments(
        &["ETHEREUM".parse()?, "CONTRACT_EVT_APPROVALFORALL".parse()?],
        substrate_node_url,
    )
    .await?;
    // Parse the SQL query
    let query: QueryExpr<DoryCommitment> =
        QueryExpr::try_new(sql.parse()?, "ethereum".parse()?, &accessor)?;
    let proof_plan = query.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;
    // Send the query to the prover
    let mut query_context = HashMap::new();
    query_context.insert(
        "ethereum.contract_evt_approvalforall".to_string(),
        ProverContextRange {
            start: 0,
            ends: vec![5],
        },
    );
    let prover_query = ProverQuery {
        proof_plan: serialized_proof_plan,
        query_context,
    };
    let mut client = ProverClient::connect(prover_uri).await?;
    let mut request = tonic::Request::new(prover_query);
    request.set_timeout(Duration::from_secs(60));
    let stringified_verifiable_result = client
        .query_with_proof(request)
        .await?
        .get_ref()
        .verifiable_result
        .clone();
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
