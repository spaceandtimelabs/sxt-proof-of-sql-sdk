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
    VerifiableResultDeserialization { error: bincode::error::DecodeError },
    /// Failed to interpret or verify query results.
    #[snafu(
        display("failed to interpret or verify query results: {source}"),
        context(false)
    )]
    Verification { source: QueryError },
}

impl From<bincode::error::DecodeError> for VerifyProverResponseError {
    fn from(error: bincode::error::DecodeError) -> Self {
        VerifyProverResponseError::VerifiableResultDeserialization { error }
    }
}

/// Verify a response from the prover service against the provided commitment accessor.
pub fn verify_prover_response<'de, 's, CP: CommitmentEvaluationProof + Deserialize<'de>>(
    prover_response: &'de ProverResponse,
    query_expr: &QueryExpr,
    accessor: &impl CommitmentAccessor<CP::Commitment>,
    verifier_setup: &CP::VerifierPublicSetup<'s>,
) -> Result<OwnedTable<CP::Scalar>, VerifyProverResponseError> {
    let verifiable_result: VerifiableQueryResult<CP> = bincode::serde::decode_borrowed_from_slice(
        &prover_response.verifiable_result,
        bincode::config::legacy()
            .with_fixed_int_encoding()
            .with_big_endian(),
    )?;

    // Verify the proof
    Ok(verifiable_result
        .verify(query_expr.proof_expr(), accessor, verifier_setup)?
        .table)
}
