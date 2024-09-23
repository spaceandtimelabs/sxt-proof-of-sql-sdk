use ark_std::test_rng;
use flexbuffers;
use proof_of_sql::{
    base::database::{owned_table_utility::*, OwnedTableTestAccessor, TestAccessor},
    proof_primitive::dory::{
        DoryEvaluationProof, DoryProverPublicSetup, DoryVerifierPublicSetup, ProverSetup,
        PublicParameters, VerifierSetup,
    },
    sql::{parse::QueryExpr, proof::VerifiableQueryResult},
};
use prover::{prover_client::ProverClient, ProverQuery};
use std::collections::HashMap;

mod prover {
    include!(concat!(env!("OUT_DIR"), "/prover.rs"));
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let sql = "SELECT * FROM table WHERE NOT a;";
    let prover_uri = "http://0.0.0.0:25519";
    // Dory setup
    let public_parameters = PublicParameters::rand(4, &mut test_rng());
    let prover_setup = ProverSetup::from(&public_parameters);
    let verifier_setup = VerifierSetup::from(&public_parameters);
    let dory_prover_setup = DoryProverPublicSetup::new(&prover_setup, 3);
    let dory_verifier_setup = DoryVerifierPublicSetup::new(&verifier_setup, 3);
    // Accessor setup
    let mut accessor =
        OwnedTableTestAccessor::<DoryEvaluationProof>::new_empty_with_setup(dory_prover_setup);
    accessor.add_table(
        "sxt.table".parse()?,
        owned_table([boolean("a", [true, false])]),
        0,
    );
    // Parse the SQL query
    let query = QueryExpr::try_new(sql.parse()?, "sxt".parse()?, &accessor)?;
    let proof_plan = query.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(&proof_plan)?;
    // Send the query to the prover
    let query_context = HashMap::new();
    let prover_query = ProverQuery {
        proof_plan: serialized_proof_plan,
        query_context,
    };
    let mut client = ProverClient::connect(prover_uri).await?;
    let request = tonic::Request::new(prover_query);
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
    let expected_result = owned_table([boolean("a", [false])]);
    assert_eq!(owned_table_result, expected_result);
    println!("Proof verified successfully!");
    Ok(())
}
