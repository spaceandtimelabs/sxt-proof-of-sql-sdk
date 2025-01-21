use crate::prover::{CommitmentScheme, ProverContextRange, ProverQuery};
use proof_of_sql::{
    base::commitment::QueryCommitments,
    proof_primitive::dory::DynamicDoryCommitment,
    sql::parse::{ConversionError, QueryExpr},
};
use proof_of_sql_parser::ParseError;
use snafu::Snafu;

/// Proof-of-sql requires a default schema to be provided when creating a QueryExpr.
/// This is used as the schema when tables referenced in the query don't have one.
const DEFAULT_SCHEMA: &str = "PUBLIC";

/// Errors that can occur when planning a query to the prover.
#[derive(Snafu, Debug)]
pub enum PlanProverQueryError {
    /// Unable to parse sql.
    #[snafu(display("unable to parse sql: {source}"), context(false))]
    ParseIdentifier { source: ParseError },
    /// Unable to create a provable AST from query.
    #[snafu(
        display("unable to create a provable AST from query: {source}"),
        context(false)
    )]
    ProvableAst { source: ConversionError },
    /// Unable to serialize proof plan.
    #[snafu(display("unable to serialize proof plan: {error}"))]
    ProofPlanSerialization {
        error: flexbuffers::SerializationError,
    },
}

impl From<flexbuffers::SerializationError> for PlanProverQueryError {
    fn from(error: flexbuffers::SerializationError) -> Self {
        PlanProverQueryError::ProofPlanSerialization { error }
    }
}

/// Create a query for the prover service from sql query text and commitments.
pub fn plan_prover_query_dory(
    query: &str,
    commitments: &QueryCommitments<DynamicDoryCommitment>,
) -> Result<(ProverQuery, QueryExpr), PlanProverQueryError> {
    let query_expr: QueryExpr =
        QueryExpr::try_new(query.parse()?, DEFAULT_SCHEMA.parse()?, commitments)?;
    let proof_plan = query_expr.proof_expr();
    let serialized_proof_plan = flexbuffers::to_vec(proof_plan)?;

    let query_context = commitments
        .iter()
        .map(|(table_ref, commitment)| {
            (
                table_ref.to_string().to_uppercase(),
                ProverContextRange {
                    start: commitment.range().start as u64,
                    ends: vec![commitment.range().end as u64],
                },
            )
        })
        .collect();

    Ok((
        ProverQuery {
            proof_plan: serialized_proof_plan,
            query_context,
            commitment_scheme: CommitmentScheme::DynamicDory.into(),
        },
        query_expr,
    ))
}
