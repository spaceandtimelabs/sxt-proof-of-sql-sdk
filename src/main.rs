mod substrate;
mod sxt_chain_runtime;

use ark_std::test_rng;
use core::time::Duration;
use curve25519_dalek::RistrettoPoint;
use flexbuffers;
use proof_of_sql::{
    base::{
        commitment::{InnerProductProof, QueryCommitments},
        database::{owned_table_utility::*, OwnedTableTestAccessor, TestAccessor},
    },
    proof_primitive::dory::{
        DoryEvaluationProof, DoryProverPublicSetup, DoryVerifierPublicSetup, ProverSetup,
        PublicParameters, VerifierSetup,
    },
    sql::{parse::QueryExpr, proof::VerifiableQueryResult},
};
use prover::{prover_client::ProverClient, ProverContextRange, ProverQuery};
use std::collections::HashMap;

mod prover {
    tonic::include_proto!("sxt.core");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT * FROM proofs_smoke_tests.kyucqklhqcmlvuyc;";
    let prover_uri = "<URI>";
    // Dory setup
    // let public_parameters = PublicParameters::rand(4, &mut test_rng());
    // let prover_setup = ProverSetup::from(&public_parameters);
    // let verifier_setup = VerifierSetup::from(&public_parameters);
    // let dory_prover_setup = DoryProverPublicSetup::new(&prover_setup, 3);
    // let dory_verifier_setup = DoryVerifierPublicSetup::new(&verifier_setup, 3);
    // Accessor setup
    let accessor = substrate::query_commitments(
        &[
            "proofs_smoke_tests".parse()?,
            "proofs_smoke_tests.kyucqklhqcmlvuyc".parse()?,
        ],
        "<URL>",
        "dory".parse()?,
    )?;
    // Parse the SQL query
    let query: QueryExpr<RistrettoPoint> =
        QueryExpr::try_new(sql.parse()?, "proofs_smoke_tests".parse()?, &accessor)?;
    let proof_plan = query.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(&proof_plan)?;
    // Send the query to the prover
    let mut query_context = HashMap::new();
    query_context.insert(
        "proofs_smoke_tests.kyucqklhqcmlvuyc".to_string(),
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
    let verifiable_result: VerifiableQueryResult<InnerProductProof> =
        flexbuffers::from_slice(&stringified_verifiable_result)?;
    // Verify the proof
    let proof = verifiable_result.proof.unwrap();
    let serialized_result = verifiable_result.provable_result.unwrap();
    let owned_table_result = proof
        .verify(query.proof_expr(), &accessor, &serialized_result, &())?
        .table;
    let expected_result = table;
    assert_eq!(owned_table_result, expected_result);
    Ok(())
}
