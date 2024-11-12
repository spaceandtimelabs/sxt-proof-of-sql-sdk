use crate::prover::ProverResponse;
use proof_of_sql::{
    base::{
        commitment::CommitmentEvaluationProof,
        database::{CommitmentAccessor, OwnedTable},
    },
    sql::{
        parse::QueryExpr,
        proof::{QueryError, VerifiableQueryResult},
    },
};
use serde::Deserialize;
use snafu::Snafu;

/// Errors that can occur when verifying a prover response.
#[derive(Snafu, Debug)]
pub enum VerifyProverResponseError {
    /// Unable to deserialize verifiable query result.
    #[snafu(display("unable to deserialize verifiable query result: {error}"))]
    VerifiableResultDeserialization {
        error: flexbuffers::DeserializationError,
    },
    /// Failed to interpret or verify query results.
    #[snafu(
        display("failed to interpret or verify query results: {source}"),
        context(false)
    )]
    Verification { source: QueryError },
}

impl From<flexbuffers::DeserializationError> for VerifyProverResponseError {
    fn from(error: flexbuffers::DeserializationError) -> Self {
        VerifyProverResponseError::VerifiableResultDeserialization { error }
    }
}

/// Verify a response from the prover service against the provided commitment accessor.
pub fn verify_prover_response<'de, 's, CP: CommitmentEvaluationProof + Deserialize<'de>>(
    prover_response: &'de ProverResponse,
    query_expr: &QueryExpr<CP::Commitment>,
    accessor: &impl CommitmentAccessor<CP::Commitment>,
    verifier_setup: &CP::VerifierPublicSetup<'s>,
) -> Result<OwnedTable<CP::Scalar>, VerifyProverResponseError> {
    let verifiable_result: VerifiableQueryResult<CP> =
        flexbuffers::from_slice(&prover_response.verifiable_result)?;

    // Verify the proof
    let proof = verifiable_result.proof.unwrap();
    let serialized_result = verifiable_result.provable_result.unwrap();
    Ok(proof
        .verify(
            query_expr.proof_expr(),
            accessor,
            &serialized_result,
            verifier_setup,
        )?
        .table)
}
